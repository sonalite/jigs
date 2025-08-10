/// Page-based memory system for RISC-V virtual machine
///
/// Note: This memory system can only be used from a single thread.
///
/// This module implements a sparse page-based memory system with:
/// - 32-bit address space with 16KB pages
/// - Two-layer page table for memory efficiency
/// - Lazy page allocation from a global shared pool
/// - Direct pointer access for native ARM64 code
/// - Reset functionality between executions
///
/// # Two-Layer Page Table Architecture
///
/// The 32-bit address space is divided as follows:
/// - Bits 31-22 (10 bits): Level 1 index - selects L2 table
/// - Bits 21-14 (8 bits): Level 2 index - selects page within L2 table
/// - Bits 13-0 (14 bits): Page offset - byte within 16KB page
///
/// Each L2 table covers 4MB of address space (256 pages × 16KB).
/// Maximum coverage is 1020MB with 255 L2 tables.
///
/// # Safety
/// PageStore MUST outlive all Memory instances. The PageStore will panic
/// if dropped while Memory instances still exist.
use std::fmt;

/// Success return code for memory operations
pub const MEM_SUCCESS: i32 = 0;

/// Error: No more L2 tables available
pub const MEM_ERR_NO_L2_TABLES: i32 = 1;

/// Error: Instance page limit reached
pub const MEM_ERR_PAGE_LIMIT: i32 = 2;

/// Error: PageStore has no available pages
pub const MEM_ERR_NO_PAGES_AVAILABLE: i32 = 3;

/// Size of a memory page in bytes (16KB)
pub const PAGE_SIZE: usize = 1 << 14;

/// Number of bits for page offset
pub const PAGE_OFFSET_BITS: usize = 14;

/// Mask for extracting page offset from address
pub const PAGE_OFFSET_MASK: u32 = (PAGE_SIZE - 1) as u32;

// Two-layer page table constants

/// Number of bits for L1 index (bits 31-22 of address)
const L1_INDEX_BITS: usize = 10;

/// Number of bits for L2 index (bits 21-14 of address)
const L2_INDEX_BITS: usize = 8;

/// Bit position where L1 index starts
const L1_INDEX_SHIFT: usize = 22;

/// Bit position where L2 index starts (same as page offset)
const L2_INDEX_SHIFT: usize = PAGE_OFFSET_BITS;

/// Number of entries in L1 table (2^10 = 1024)
const L1_TABLE_SIZE: usize = 1 << L1_INDEX_BITS;

/// Number of entries in each L2 table (2^8 = 256)
const L2_TABLE_SIZE: usize = 1 << L2_INDEX_BITS;

/// Mask for extracting L1 index from shifted address
const L1_INDEX_MASK: u32 = (L1_TABLE_SIZE - 1) as u32;

/// Mask for extracting L2 index from shifted address
const L2_INDEX_MASK: u32 = (L2_TABLE_SIZE - 1) as u32;

/// Default maximum number of L2 tables that can be allocated
/// Limited to 255 because UNMAPPED_L2_TABLE uses the value 0xFF
pub const MAX_L2_TABLES: usize = 255;

/// Special value indicating an unmapped L2 table in L1 entries
const UNMAPPED_L2_TABLE: u8 = 0xFF;

/// Maximum number of pages that can be allocated
/// Limited to 65535 because UNMAPPED_PAGE uses the value 0xFFFF
/// This gives us 65535 * 16KB = ~1GB minus one page
pub const MAX_PAGES: usize = 65535;

/// Special value indicating an unmapped page in L2 entries
/// Uses 0xFFFF which is why MAX_PAGES must be one less
pub const UNMAPPED_PAGE: u16 = 0xFFFF;

/// Global page store that manages memory pages across all VM instances
/// Pages are allocated from and returned to a pool
#[repr(C)]
pub struct PageStore {
    /// Linear memory for all pages - allows direct offset calculation
    /// Page N starts at offset N * PAGE_SIZE (or N << 14)
    /// Offset: 0x00
    pub page_memory: *mut u8,

    /// Total size of page_memory in bytes
    /// Offset: 0x08
    pub page_memory_size: usize,

    /// Pool of available page indices - fixed size for ARM64 access
    /// Contains available page indices in positions [0..num_available_pages]
    /// Offset: 0x10
    pub available_pages: *mut u16,

    /// Total capacity of available_pages array
    /// Offset: 0x18
    pub available_pages_capacity: usize,

    /// Number of pages currently available in the pool
    /// Offset: 0x20
    pub num_available_pages: usize,

    /// Number of Memory instances using this PageStore
    /// Offset: 0x28
    pub instance_count: usize,
}

impl PageStore {
    /// Create a new page store with the specified total number of pages
    ///
    /// # Panics
    /// Panics if total_pages > MAX_PAGES (65535)
    pub fn new(total_pages: usize) -> Self {
        assert!(
            total_pages <= MAX_PAGES,
            "total_pages {} exceeds maximum allowed ({})",
            total_pages,
            MAX_PAGES
        );

        // Pre-allocate linear memory for all pages
        let total_bytes = total_pages * PAGE_SIZE;
        let page_memory = vec![0u8; total_bytes].into_boxed_slice();
        let page_memory_ptr = Box::into_raw(page_memory) as *mut u8;

        // Initialize available pages array [0, 1, 2, ..., total_pages-1]
        let mut available_pages = Vec::with_capacity(total_pages);
        for i in 0..total_pages {
            available_pages.push(i as u16);
        }
        let available_pages = available_pages.into_boxed_slice();
        let available_pages_ptr = Box::into_raw(available_pages) as *mut u16;

        Self {
            page_memory: page_memory_ptr,
            page_memory_size: total_bytes,
            available_pages: available_pages_ptr,
            available_pages_capacity: total_pages,
            num_available_pages: total_pages,
            instance_count: 0,
        }
    }
}

impl Drop for PageStore {
    fn drop(&mut self) {
        if self.instance_count > 0 {
            panic!(
                "PageStore dropped while {} Memory instance(s) still exist",
                self.instance_count
            );
        }

        // Clean up allocated memory
        unsafe {
            if !self.page_memory.is_null() {
                let page_memory = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.page_memory,
                    self.page_memory_size,
                ));
                drop(page_memory);
            }

            if !self.available_pages.is_null() {
                let available_pages = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.available_pages,
                    self.available_pages_capacity,
                ));
                drop(available_pages);
            }
        }
    }
}

/// Memory system for a single VM instance with two-layer page table
///
/// The two-layer design significantly reduces memory overhead for sparse
/// address space usage, which is common in embedded and sandboxed environments.
#[repr(C)]
pub struct Memory {
    /// Pointer to the PageStore - PageStore must outlive this Memory
    /// Offset: 0x000
    pub page_store: *mut PageStore,

    /// Cached pointer to the start of page memory for fast access
    /// This points to the same memory as PageStore.page_memory
    /// Offset: 0x008
    pub page_memory: *mut u8,

    /// Level 1 page table: maps L1 index to L2 table index (0-254) or UNMAPPED_L2_TABLE (0xFF)
    /// Using u8 saves memory: 1024 entries × 1 byte = 1KB
    /// Embedded directly in the struct for efficient access
    /// Offset: 0x010
    /// Size: 0x400 (1024 bytes)
    pub l1_table: [u8; L1_TABLE_SIZE],

    /// Pool of Level 2 page tables: each maps L2 index to global page index
    /// Pre-allocated as contiguous array for predictable memory usage and ARM64 access
    /// Each L2 table is L2_TABLE_SIZE (256) u16 entries
    /// Table N starts at offset N * L2_TABLE_SIZE * sizeof(u16)
    /// Offset: 0x410
    pub l2_tables: *mut u16,

    /// Fixed array of allocated page indices for ARM64 access
    /// Offset: 0x418
    pub allocated_indices: *mut u16,

    /// Number of pages currently allocated
    /// Offset: 0x420
    pub num_pages: usize,

    /// Maximum number of pages this VM instance can allocate
    /// Offset: 0x428
    pub max_pages: usize,

    /// Number of L2 tables currently allocated
    /// Offset: 0x430
    pub num_l2_tables: usize,

    /// Maximum number of L2 tables this VM instance can allocate
    /// Offset: 0x438
    pub max_l2_tables: usize,
}

impl Memory {
    /// Create a new memory system that uses the provided page store
    ///
    /// # Safety
    /// The PageStore must outlive this Memory instance
    ///
    /// # Panics
    /// - Panics if max_pages > MAX_PAGES (65535)
    /// - Panics if max_pages > PageStore's available pages
    /// - Panics if max_l2_tables > MAX_L2_TABLES (255)
    pub fn new(page_store: &mut PageStore, max_pages: usize, max_l2_tables: usize) -> Self {
        assert!(
            max_pages <= MAX_PAGES,
            "max_pages {} exceeds maximum allowed ({})",
            max_pages,
            MAX_PAGES
        );
        assert!(
            max_pages <= page_store.num_available_pages,
            "max_pages {} exceeds available pages in PageStore ({})",
            max_pages,
            page_store.num_available_pages
        );
        assert!(
            max_l2_tables <= MAX_L2_TABLES,
            "max_l2_tables {} exceeds maximum allowed ({})",
            max_l2_tables,
            MAX_L2_TABLES
        );

        page_store.instance_count += 1;

        // Allocate L2 tables as contiguous array
        // Each table is L2_TABLE_SIZE entries, all tables in a row
        let total_l2_entries = max_l2_tables * L2_TABLE_SIZE;
        let l2_tables = vec![UNMAPPED_PAGE; total_l2_entries].into_boxed_slice();
        let l2_tables_ptr = Box::into_raw(l2_tables) as *mut u16;

        // Allocate allocated_indices array
        let allocated_indices = vec![0u16; max_pages].into_boxed_slice();
        let allocated_indices_ptr = Box::into_raw(allocated_indices) as *mut u16;

        Self {
            page_store: page_store as *mut PageStore,
            page_memory: page_store.page_memory,
            l1_table: [UNMAPPED_L2_TABLE; L1_TABLE_SIZE],
            l2_tables: l2_tables_ptr,
            allocated_indices: allocated_indices_ptr,
            num_pages: 0,
            max_pages,
            num_l2_tables: 0,
            max_l2_tables,
        }
    }

    /// Allocate a page for the given address if not already allocated
    ///
    /// # Returns
    /// - `MEM_SUCCESS` (0): Page successfully allocated or already mapped
    /// - `MEM_ERR_NO_L2_TABLES` (1): No more L2 tables available
    /// - `MEM_ERR_PAGE_LIMIT` (2): Instance page limit reached
    /// - `MEM_ERR_NO_PAGES_AVAILABLE` (3): PageStore has no available pages
    ///
    /// # Two-Layer Allocation Process
    /// 1. Extract L1 and L2 indices from the address
    /// 2. Check if an L2 table exists for this L1 entry
    /// 3. If not, allocate a new L2 table from the pool
    /// 4. Look up the page in the L2 table
    /// 5. If unmapped, allocate a page from the PageStore
    pub fn allocate_page(&mut self, address: u32) -> i32 {
        // Extract L1 and L2 indices from address
        // Address layout: [L1 Index: 10 bits][L2 Index: 8 bits][Page Offset: 14 bits]
        let l1_idx = ((address >> L1_INDEX_SHIFT) & L1_INDEX_MASK) as usize;
        let l2_idx = ((address >> L2_INDEX_SHIFT) & L2_INDEX_MASK) as usize;

        // Check if L2 table exists for this L1 entry
        let l2_table_idx = if self.l1_table[l1_idx] == UNMAPPED_L2_TABLE {
            // Need to allocate new L2 table
            if self.num_l2_tables >= self.max_l2_tables {
                return MEM_ERR_NO_L2_TABLES;
            }

            let new_l2_idx = self.num_l2_tables as u8;
            self.l1_table[l1_idx] = new_l2_idx;

            // L2 table is already initialized with UNMAPPED_PAGE values
            self.num_l2_tables += 1;
            new_l2_idx
        } else {
            self.l1_table[l1_idx]
        };

        // Check if page is already mapped in L2 table
        unsafe {
            // Calculate offset to the L2 table entry
            let l2_entry_offset = (l2_table_idx as usize) * L2_TABLE_SIZE + l2_idx;
            if *self.l2_tables.add(l2_entry_offset) != UNMAPPED_PAGE {
                return MEM_SUCCESS; // Page already mapped
            }
        }

        // Check if we have room for another page
        if self.num_pages >= self.max_pages {
            return MEM_ERR_PAGE_LIMIT;
        }

        // Allocate from PageStore
        unsafe {
            let store = &mut *self.page_store;

            // Check if PageStore has available pages
            if store.num_available_pages == 0 {
                return MEM_ERR_NO_PAGES_AVAILABLE;
            }

            // Get next available page
            store.num_available_pages -= 1;
            let page_idx = *store.available_pages.add(store.num_available_pages);

            // Track this allocation
            *self.allocated_indices.add(self.num_pages) = page_idx;
            self.num_pages += 1;

            // Map in L2 table
            let l2_table_idx = self.l1_table[l1_idx] as usize;
            let l2_entry_offset = l2_table_idx * L2_TABLE_SIZE + l2_idx;
            *self.l2_tables.add(l2_entry_offset) = page_idx;

            MEM_SUCCESS
        }
    }

    /// Read data from memory into the provided buffer
    ///
    /// Reads `buffer.len()` bytes starting from the given address. If a page
    /// is not allocated, the corresponding bytes in the buffer are filled with zeros.
    ///
    /// This method is optimized for performance and handles:
    /// - Reading across page boundaries
    /// - Sparse memory regions (unallocated pages read as zeros)
    /// - Partial page reads
    /// - Address wraparound (reading past 0xFFFFFFFF continues from 0x00000000)
    ///
    /// # Arguments
    /// * `address` - The starting address to read from
    /// * `buffer` - The buffer to fill with read data
    ///
    /// # Address Wraparound
    /// The method uses `wrapping_add` for address arithmetic, so reads that
    /// extend past the end of the 32-bit address space (0xFFFFFFFF) will wrap
    /// around to the beginning (0x00000000) and continue reading.
    pub fn read(&self, address: u32, buffer: &mut [u8]) {
        let mut addr = address;
        let mut offset = 0;
        let len = buffer.len();

        while offset < len {
            // Calculate how many bytes to read from current page
            let page_offset = (addr & PAGE_OFFSET_MASK) as usize;
            let bytes_in_page = (PAGE_SIZE - page_offset).min(len - offset);

            // Extract L1 and L2 indices
            let l1_idx = ((addr >> L1_INDEX_SHIFT) & L1_INDEX_MASK) as usize;
            let l2_idx = ((addr >> L2_INDEX_SHIFT) & L2_INDEX_MASK) as usize;

            // Check if L2 table exists
            let l2_table_idx = self.l1_table[l1_idx];
            if l2_table_idx == UNMAPPED_L2_TABLE {
                // No L2 table - fill with zeros
                buffer[offset..offset + bytes_in_page].fill(0);
            } else {
                // Get page index from L2 table
                unsafe {
                    let l2_entry_offset = (l2_table_idx as usize) * L2_TABLE_SIZE + l2_idx;
                    let page_idx = *self.l2_tables.add(l2_entry_offset);

                    if page_idx == UNMAPPED_PAGE {
                        // Page not allocated - fill with zeros
                        buffer[offset..offset + bytes_in_page].fill(0);
                    } else {
                        // Copy data from the page
                        let page_addr = self
                            .page_memory
                            .add(page_idx as usize * PAGE_SIZE + page_offset);
                        std::ptr::copy_nonoverlapping(
                            page_addr,
                            buffer[offset..].as_mut_ptr(),
                            bytes_in_page,
                        );
                    }
                }
            }

            offset += bytes_in_page;
            addr = addr.wrapping_add(bytes_in_page as u32);
        }
    }

    /// Write data from a buffer into memory
    ///
    /// Writes `buffer.len()` bytes starting at the given address. If a page
    /// is not allocated, it will be allocated on demand. If allocation fails,
    /// an error code is returned.
    ///
    /// This method is optimized for performance and handles:
    /// - Writing across page boundaries
    /// - Automatic page allocation on write
    /// - Partial page writes
    /// - Address wraparound (writing past 0xFFFFFFFF continues from 0x00000000)
    ///
    /// # Arguments
    /// * `address` - The starting address to write to
    /// * `buffer` - The buffer containing data to write
    ///
    /// # Returns
    /// - `MEM_SUCCESS` (0): Write completed successfully
    /// - `MEM_ERR_NO_L2_TABLES` (1): No more L2 tables available
    /// - `MEM_ERR_PAGE_LIMIT` (2): Instance page limit reached
    /// - `MEM_ERR_NO_PAGES_AVAILABLE` (3): PageStore has no available pages
    ///
    /// # Address Wraparound
    /// The method uses `wrapping_add` for address arithmetic, so writes that
    /// extend past the end of the 32-bit address space (0xFFFFFFFF) will wrap
    /// around to the beginning (0x00000000) and continue writing.
    pub fn write(&mut self, address: u32, buffer: &[u8]) -> i32 {
        let mut addr = address;
        let mut offset = 0;
        let len = buffer.len();

        while offset < len {
            // Calculate how many bytes to write to current page
            let page_offset = (addr & PAGE_OFFSET_MASK) as usize;
            let bytes_in_page = (PAGE_SIZE - page_offset).min(len - offset);

            // Ensure page is allocated
            let page_base = addr & !PAGE_OFFSET_MASK;
            let alloc_result = self.allocate_page(page_base);
            if alloc_result != MEM_SUCCESS {
                return alloc_result;
            }

            // Extract L1 and L2 indices to get the page
            let l1_idx = ((addr >> L1_INDEX_SHIFT) & L1_INDEX_MASK) as usize;
            let l2_idx = ((addr >> L2_INDEX_SHIFT) & L2_INDEX_MASK) as usize;

            // Get page index from L2 table (guaranteed to exist after allocate_page)
            unsafe {
                let l2_table_idx = self.l1_table[l1_idx];
                let l2_entry_offset = (l2_table_idx as usize) * L2_TABLE_SIZE + l2_idx;
                let page_idx = *self.l2_tables.add(l2_entry_offset);

                // Write data to the page
                let page_addr = self
                    .page_memory
                    .add(page_idx as usize * PAGE_SIZE + page_offset);
                std::ptr::copy_nonoverlapping(buffer[offset..].as_ptr(), page_addr, bytes_in_page);
            }

            offset += bytes_in_page;
            addr = addr.wrapping_add(bytes_in_page as u32);
        }

        MEM_SUCCESS
    }

    /// Reset this memory instance, returning all pages to the pool
    ///
    /// This clears both levels of the page table hierarchy:
    /// 1. Returns all allocated pages to the PageStore
    /// 2. Clears all L2 table entries
    /// 3. Resets all L1 table entries to unmapped
    /// 4. Resets L2 table allocation counter
    pub fn reset(&mut self) {
        if self.num_pages == 0 {
            return;
        }

        unsafe {
            let store = &mut *self.page_store;

            // Return each page to the pool
            for i in 0..self.num_pages {
                let page_idx = *self.allocated_indices.add(i);

                // Clear the page memory
                let offset = page_idx as usize * PAGE_SIZE;
                let page_ptr = self.page_memory.add(offset);
                std::ptr::write_bytes(page_ptr, 0, PAGE_SIZE);

                // Add page back to available pool
                *store.available_pages.add(store.num_available_pages) = page_idx;
                store.num_available_pages += 1;
            }

            // Clear all L1 table entries
            self.l1_table.fill(UNMAPPED_L2_TABLE);

            // Clear all allocated L2 tables
            for l2_idx in 0..self.num_l2_tables {
                let table_offset = l2_idx * L2_TABLE_SIZE;
                for i in 0..L2_TABLE_SIZE {
                    *self.l2_tables.add(table_offset + i) = UNMAPPED_PAGE;
                }
            }

            self.num_l2_tables = 0;
            self.num_pages = 0;
        }
    }
}

impl fmt::Debug for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Each L2 table covers: 256 pages × 16KB = 4MB
        let l2_coverage_mb = self.num_l2_tables * L2_TABLE_SIZE * PAGE_SIZE / (1024 * 1024);
        f.debug_struct("Memory")
            .field("num_pages", &self.num_pages)
            .field("max_pages", &self.max_pages)
            .field("num_l2_tables", &self.num_l2_tables)
            .field("max_l2_tables", &self.max_l2_tables)
            .field("l2_coverage_mb", &l2_coverage_mb)
            .finish()
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        unsafe {
            // Reset to return pages to pool
            self.reset();

            let store = &mut *self.page_store;
            store.instance_count -= 1;

            // Clean up L2 tables
            if !self.l2_tables.is_null() {
                let total_l2_entries = self.max_l2_tables * L2_TABLE_SIZE;
                let l2_tables = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.l2_tables,
                    total_l2_entries,
                ));
                drop(l2_tables);
            }

            // Clean up allocated_indices
            if !self.allocated_indices.is_null() {
                let allocated_indices = Box::from_raw(std::slice::from_raw_parts_mut(
                    self.allocated_indices,
                    self.max_pages,
                ));
                drop(allocated_indices);
            }
        }
    }
}
