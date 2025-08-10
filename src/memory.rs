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
pub struct PageStore {
    /// Linear memory for all pages - allows direct offset calculation
    /// Page N starts at offset N * PAGE_SIZE (or N << 14)
    pub page_memory: Box<[u8]>,

    /// Pool of available page indices - fixed size for ARM64 access
    /// Contains available page indices in positions [0..num_available_pages]
    pub available_pages: Box<[u16]>,

    /// Number of pages currently available in the pool
    pub num_available_pages: usize,

    /// Number of Memory instances using this PageStore
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

        // Initialize available pages array [0, 1, 2, ..., total_pages-1]
        let mut available_pages = Vec::with_capacity(total_pages);
        for i in 0..total_pages {
            available_pages.push(i as u16);
        }

        Self {
            page_memory,
            available_pages: available_pages.into_boxed_slice(),
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
    }
}

/// Memory system for a single VM instance with two-layer page table
///
/// The two-layer design significantly reduces memory overhead for sparse
/// address space usage, which is common in embedded and sandboxed environments.
pub struct Memory {
    /// Pointer to the PageStore - PageStore must outlive this Memory
    page_store: *mut PageStore,

    /// Level 1 page table: maps L1 index to L2 table index (0-254) or UNMAPPED_L2_TABLE (0xFF)
    /// Using u8 saves memory: 1024 entries × 1 byte = 1KB
    l1_table: Box<[u8; L1_TABLE_SIZE]>,

    /// Pool of Level 2 page tables: each maps L2 index to global page index
    /// Pre-allocated as fixed array for predictable memory usage and ARM64 access
    l2_tables: Box<[Vec<u16>]>,

    /// Fixed array of allocated page indices for ARM64 access
    allocated_indices: Box<[u16]>,

    /// Number of pages currently allocated
    pub num_pages: usize,

    /// Maximum number of pages this VM instance can allocate
    pub max_pages: usize,

    /// Number of L2 tables currently allocated
    pub num_l2_tables: usize,

    /// Maximum number of L2 tables this VM instance can allocate
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
    /// - Panics if max_l2_tables > MAX_L2_TABLES (255)
    pub fn new(page_store: &mut PageStore, max_pages: usize, max_l2_tables: usize) -> Self {
        assert!(
            max_pages <= MAX_PAGES,
            "max_pages {} exceeds maximum allowed ({})",
            max_pages,
            MAX_PAGES
        );
        assert!(
            max_l2_tables <= MAX_L2_TABLES,
            "max_l2_tables {} exceeds maximum allowed ({})",
            max_l2_tables,
            MAX_L2_TABLES
        );

        page_store.instance_count += 1;

        // Initialize L2 tables with unmapped pages
        let mut l2_tables = Vec::with_capacity(max_l2_tables);
        for _ in 0..max_l2_tables {
            l2_tables.push(vec![UNMAPPED_PAGE; L2_TABLE_SIZE]);
        }

        Self {
            page_store: page_store as *mut PageStore,
            l1_table: Box::new([UNMAPPED_L2_TABLE; L1_TABLE_SIZE]),
            l2_tables: l2_tables.into_boxed_slice(),
            allocated_indices: vec![0u16; max_pages].into_boxed_slice(),
            num_pages: 0,
            max_pages,
            num_l2_tables: 0,
            max_l2_tables,
        }
    }

    /// Allocate a page for the given address if not already allocated
    /// Returns true if successful (either newly allocated or already mapped)
    ///
    /// # Two-Layer Allocation Process
    /// 1. Extract L1 and L2 indices from the address
    /// 2. Check if an L2 table exists for this L1 entry
    /// 3. If not, allocate a new L2 table from the pool
    /// 4. Look up the page in the L2 table
    /// 5. If unmapped, allocate a page from the PageStore
    pub fn allocate_page(&mut self, address: u32) -> bool {
        // Extract L1 and L2 indices from address
        // Address layout: [L1 Index: 10 bits][L2 Index: 8 bits][Page Offset: 14 bits]
        let l1_idx = ((address >> L1_INDEX_SHIFT) & L1_INDEX_MASK) as usize;
        let l2_idx = ((address >> L2_INDEX_SHIFT) & L2_INDEX_MASK) as usize;

        // Check if L2 table exists for this L1 entry
        let l2_table_idx = if self.l1_table[l1_idx] == UNMAPPED_L2_TABLE {
            // Need to allocate new L2 table
            if self.num_l2_tables >= self.max_l2_tables {
                return false; // No more L2 tables available
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
        let l2_table = &self.l2_tables[l2_table_idx as usize];
        if l2_table[l2_idx] != UNMAPPED_PAGE {
            return true; // Page already mapped
        }

        // Check if we have room for another page
        if self.num_pages >= self.max_pages {
            return false;
        }

        // Allocate from PageStore
        unsafe {
            let store = &mut *self.page_store;

            // Check if PageStore has available pages
            if store.num_available_pages == 0 {
                return false;
            }

            // Get next available page
            store.num_available_pages -= 1;
            let page_idx = store.available_pages[store.num_available_pages];

            // Track this allocation
            self.allocated_indices[self.num_pages] = page_idx;
            self.num_pages += 1;

            // Map in L2 table
            let l2_table = &mut self.l2_tables[l2_table_idx as usize];
            l2_table[l2_idx] = page_idx;

            true
        }
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
                let page_idx = self.allocated_indices[i];

                // Clear the page memory
                let offset = page_idx as usize * PAGE_SIZE;
                store.page_memory[offset..offset + PAGE_SIZE].fill(0);

                // Add page back to available pool
                store.available_pages[store.num_available_pages] = page_idx;
                store.num_available_pages += 1;
            }

            // Clear all L1 table entries
            self.l1_table.fill(UNMAPPED_L2_TABLE);

            // Clear all allocated L2 tables
            for l2_idx in 0..self.num_l2_tables {
                self.l2_tables[l2_idx].fill(UNMAPPED_PAGE);
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
            let store = &mut *self.page_store;
            store.instance_count -= 1;
        }
    }
}
