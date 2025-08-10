use crate::memory::{MEM_SUCCESS, Memory, PAGE_OFFSET_MASK, PAGE_SIZE, PageStore, UNMAPPED_PAGE};

/// Helper function to get the physical page pointer for a given address
fn get_page_ptr(memory: &Memory, address: u32) -> Option<*mut u8> {
    let l1_idx = ((address >> 22) & 0x3FF) as usize;
    let l2_idx = ((address >> 14) & 0xFF) as usize;

    let l2_table_idx = memory.l1_table[l1_idx];
    if l2_table_idx == 0xFF {
        return None;
    }

    unsafe {
        let l2_entry_offset = (l2_table_idx as usize) * 256 + l2_idx;
        let page_idx = *memory.l2_tables.add(l2_entry_offset);
        if page_idx == UNMAPPED_PAGE {
            return None;
        }

        Some(memory.page_memory.add(page_idx as usize * PAGE_SIZE))
    }
}

#[test]
fn empty_buffer() {
    let mut store = PageStore::new(10);
    let memory = Memory::new(&mut store, 5, 2);
    let mut buffer = [];
    memory.read(0, &mut buffer);
}

#[test]
fn single_byte_unallocated() {
    let mut store = PageStore::new(10);
    let memory = Memory::new(&mut store, 5, 2);
    let mut buffer = [0xFF];
    memory.read(0, &mut buffer);
    assert_eq!(buffer[0], 0);
}

#[test]
fn multiple_bytes_unallocated() {
    let mut store = PageStore::new(10);
    let memory = Memory::new(&mut store, 5, 2);
    let mut buffer = vec![0xFF; 100];
    memory.read(0, &mut buffer);
    assert!(buffer.iter().all(|&b| b == 0));
}

#[test]
fn single_byte_allocated() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 0).unwrap();
        *page_ptr = 42;
    }
    let mut buffer = [0];
    memory.read(0, &mut buffer);
    assert_eq!(buffer[0], 42);
}

#[test]
fn multiple_bytes_same_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 0).unwrap();
        for i in 0..10 {
            *page_ptr.add(i) = i as u8;
        }
    }
    let mut buffer = vec![0xFF; 10];
    memory.read(0, &mut buffer);
    for i in 0..10 {
        assert_eq!(buffer[i], i as u8);
    }
}

#[test]
fn read_across_page_boundary() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let first_page_end = PAGE_SIZE as u32 - 2;
    assert_eq!(memory.allocate_page(first_page_end), MEM_SUCCESS);
    assert_eq!(memory.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    unsafe {
        let first_page = get_page_ptr(&memory, 0).unwrap();
        let second_page = get_page_ptr(&memory, PAGE_SIZE as u32).unwrap();
        *first_page.add(PAGE_SIZE - 2) = 0xAA;
        *first_page.add(PAGE_SIZE - 1) = 0xBB;
        *second_page = 0xCC;
        *second_page.add(1) = 0xDD;
    }
    let mut buffer = vec![0; 4];
    memory.read(first_page_end, &mut buffer);
    assert_eq!(buffer, vec![0xAA, 0xBB, 0xCC, 0xDD]);
}

#[test]
fn read_mixed_allocated_unallocated() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    assert_eq!(memory.allocate_page(2 * PAGE_SIZE as u32), MEM_SUCCESS);
    unsafe {
        let first_page = get_page_ptr(&memory, 0).unwrap();
        let third_page = get_page_ptr(&memory, 2 * PAGE_SIZE as u32).unwrap();
        for i in 0..PAGE_SIZE {
            *first_page.add(i) = 0x11;
            *third_page.add(i) = 0x33;
        }
    }
    let mut buffer = vec![0xFF; PAGE_SIZE * 3];
    memory.read(0, &mut buffer);
    for i in 0..PAGE_SIZE {
        assert_eq!(buffer[i], 0x11);
    }
    for i in PAGE_SIZE..(2 * PAGE_SIZE) {
        assert_eq!(buffer[i], 0);
    }
    for i in (2 * PAGE_SIZE)..(3 * PAGE_SIZE) {
        assert_eq!(buffer[i], 0x33);
    }
}

#[test]
fn read_with_offset_in_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(100), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 100).unwrap();
        for i in 0..PAGE_SIZE {
            *page_ptr.add(i) = (i % 256) as u8;
        }
    }
    let mut buffer = vec![0; 10];
    memory.read(100, &mut buffer);
    for i in 0..10 {
        assert_eq!(buffer[i], ((100 + i) % 256) as u8);
    }
}

#[test]
fn read_entire_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 0).unwrap();
        for i in 0..PAGE_SIZE {
            *page_ptr.add(i) = (i % 256) as u8;
        }
    }
    let mut buffer = vec![0xFF; PAGE_SIZE];
    memory.read(0, &mut buffer);
    for i in 0..PAGE_SIZE {
        assert_eq!(buffer[i], (i % 256) as u8);
    }
}

#[test]
fn read_multiple_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    for i in 0..3 {
        assert_eq!(memory.allocate_page(i * PAGE_SIZE as u32), MEM_SUCCESS);
    }
    unsafe {
        for page in 0..3 {
            let page_ptr = get_page_ptr(&memory, page * PAGE_SIZE as u32).unwrap();
            for i in 0..PAGE_SIZE {
                *page_ptr.add(i) = (page + 1) as u8;
            }
        }
    }
    let mut buffer = vec![0; PAGE_SIZE * 3];
    memory.read(0, &mut buffer);
    for page in 0..3 {
        for i in 0..PAGE_SIZE {
            assert_eq!(buffer[page * PAGE_SIZE + i], (page + 1) as u8);
        }
    }
}

#[test]
fn read_at_page_boundary() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 0).unwrap();
        *page_ptr.add(PAGE_SIZE - 1) = 0x42;
    }
    let mut buffer = [0];
    memory.read(PAGE_SIZE as u32 - 1, &mut buffer);
    assert_eq!(buffer[0], 0x42);
}

#[test]
fn read_unallocated_l2_table() {
    let mut store = PageStore::new(10);
    let memory = Memory::new(&mut store, 5, 2);
    let high_address = 0x40000000;
    let mut buffer = vec![0xFF; 100];
    memory.read(high_address, &mut buffer);
    assert!(buffer.iter().all(|&b| b == 0));
}

#[test]
fn read_partial_page_at_end() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = PAGE_SIZE as u32 - 10;
    assert_eq!(memory.allocate_page(addr), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, addr).unwrap();
        for i in (PAGE_SIZE - 10)..PAGE_SIZE {
            *page_ptr.add(i) = 0xEE;
        }
    }
    let mut buffer = vec![0; 20];
    memory.read(addr, &mut buffer);
    for i in 0..10 {
        assert_eq!(buffer[i], 0xEE);
    }
    for i in 10..20 {
        assert_eq!(buffer[i], 0);
    }
}

#[test]
fn read_zero_at_various_alignments() {
    let mut store = PageStore::new(10);
    let memory = Memory::new(&mut store, 5, 2);
    let alignments = [0, 1, 2, 3, 4, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128];
    for &align in &alignments {
        let mut buffer = vec![0xFF; 256];
        memory.read(align, &mut buffer);
        assert!(buffer.iter().all(|&b| b == 0));
    }
}

#[test]
fn read_after_reset() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 0).unwrap();
        *page_ptr = 0x42;
    }
    memory.reset();
    let mut buffer = [0xFF];
    memory.read(0, &mut buffer);
    assert_eq!(buffer[0], 0);
}

#[test]
fn read_sparse_l2_entries() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    assert_eq!(memory.allocate_page(10 * PAGE_SIZE as u32), MEM_SUCCESS);
    unsafe {
        let page1 = get_page_ptr(&memory, 0).unwrap();
        let page2 = get_page_ptr(&memory, 10 * PAGE_SIZE as u32).unwrap();
        *page1 = 0x11;
        *page2 = 0x22;
    }
    let mut buffer1 = [0];
    let mut buffer2 = [0];
    let mut buffer3 = [0];
    memory.read(0, &mut buffer1);
    memory.read(PAGE_SIZE as u32, &mut buffer3);
    memory.read(10 * PAGE_SIZE as u32, &mut buffer2);
    assert_eq!(buffer1[0], 0x11);
    assert_eq!(buffer2[0], 0x22);
    assert_eq!(buffer3[0], 0);
}

#[test]
fn read_large_buffer_performance() {
    let mut store = PageStore::new(100);
    let mut memory = Memory::new(&mut store, 50, 10);
    for i in 0..10 {
        assert_eq!(memory.allocate_page(i * PAGE_SIZE as u32), MEM_SUCCESS);
    }
    unsafe {
        for page in 0..10 {
            let page_ptr = get_page_ptr(&memory, page as u32 * PAGE_SIZE as u32).unwrap();
            for i in 0..PAGE_SIZE {
                *page_ptr.add(i) = ((page * PAGE_SIZE + i) % 256) as u8;
            }
        }
    }
    let mut buffer = vec![0; PAGE_SIZE * 10];
    memory.read(0, &mut buffer);
    for i in 0..buffer.len() {
        assert_eq!(buffer[i], (i % 256) as u8);
    }
}

#[test]
fn read_with_high_l1_index() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let high_addr = 0xFFC00000;
    assert_eq!(memory.allocate_page(high_addr), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, high_addr).unwrap();
        *page_ptr = 0x99;
    }
    let mut buffer = [0];
    memory.read(high_addr, &mut buffer);
    assert_eq!(buffer[0], 0x99);
}

#[test]
fn read_with_high_l2_index() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = (255 << 14) as u32;
    assert_eq!(memory.allocate_page(addr), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, addr).unwrap();
        *page_ptr = 0x88;
    }
    let mut buffer = [0];
    memory.read(addr, &mut buffer);
    assert_eq!(buffer[0], 0x88);
}

#[test]
fn read_all_page_offsets() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, 0).unwrap();
        for i in 0..PAGE_SIZE {
            *page_ptr.add(i) = (i % 256) as u8;
        }
    }
    for offset in 0..PAGE_SIZE {
        let mut buffer = [0];
        memory.read(offset as u32, &mut buffer);
        assert_eq!(buffer[0], (offset % 256) as u8);
    }
}

#[test]
fn read_crosses_multiple_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 4);
    for i in 0..4 {
        assert_eq!(memory.allocate_page(i * PAGE_SIZE as u32), MEM_SUCCESS);
    }
    unsafe {
        for page in 0..4 {
            let page_ptr = get_page_ptr(&memory, page * PAGE_SIZE as u32).unwrap();
            for i in 0..PAGE_SIZE {
                *page_ptr.add(i) = page as u8;
            }
        }
    }
    let start = PAGE_SIZE / 2;
    let len = PAGE_SIZE * 3;
    let mut buffer = vec![0xFF; len];
    memory.read(start as u32, &mut buffer);
    for i in 0..(PAGE_SIZE / 2) {
        assert_eq!(buffer[i], 0);
    }
    for i in (PAGE_SIZE / 2)..((PAGE_SIZE * 3) / 2) {
        assert_eq!(buffer[i], 1);
    }
    for i in ((PAGE_SIZE * 3) / 2)..((PAGE_SIZE * 5) / 2) {
        assert_eq!(buffer[i], 2);
    }
    for i in ((PAGE_SIZE * 5) / 2)..len {
        assert_eq!(buffer[i], 3);
    }
}

#[test]
fn read_single_byte_each_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 3);
    for i in 0..3 {
        assert_eq!(memory.allocate_page(i * PAGE_SIZE as u32), MEM_SUCCESS);
    }
    unsafe {
        for page in 0..3 {
            let page_ptr = get_page_ptr(&memory, page * PAGE_SIZE as u32).unwrap();
            *page_ptr = (page + 1) as u8;
        }
    }
    for page in 0..3 {
        let mut buffer = [0];
        memory.read(page * PAGE_SIZE as u32, &mut buffer);
        assert_eq!(buffer[0], (page + 1) as u8);
    }
}

#[test]
fn read_exact_page_alignment() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.allocate_page(0), MEM_SUCCESS);
    assert_eq!(memory.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    unsafe {
        let page1 = get_page_ptr(&memory, 0).unwrap();
        let page2 = get_page_ptr(&memory, PAGE_SIZE as u32).unwrap();
        for i in 0..PAGE_SIZE {
            *page1.add(i) = 0x11;
            *page2.add(i) = 0x22;
        }
    }
    let mut buffer = vec![0; PAGE_SIZE * 2];
    memory.read(0, &mut buffer);
    for i in 0..PAGE_SIZE {
        assert_eq!(buffer[i], 0x11);
        assert_eq!(buffer[PAGE_SIZE + i], 0x22);
    }
}

#[test]
fn read_with_wraparound() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = 0xFFFFFFFC;
    assert_eq!(memory.allocate_page(addr), MEM_SUCCESS);
    unsafe {
        let page_ptr = get_page_ptr(&memory, addr).unwrap();
        let offset = (addr & PAGE_OFFSET_MASK) as usize;
        for i in 0..4 {
            *page_ptr.add(offset + i) = (0xF0 + i) as u8;
        }
    }
    let mut buffer = vec![0; 8];
    memory.read(addr, &mut buffer);
    assert_eq!(buffer[0], 0xF0);
    assert_eq!(buffer[1], 0xF1);
    assert_eq!(buffer[2], 0xF2);
    assert_eq!(buffer[3], 0xF3);
    for i in 4..8 {
        assert_eq!(buffer[i], 0);
    }
}
