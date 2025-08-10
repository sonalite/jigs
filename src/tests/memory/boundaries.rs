use crate::memory::{MEM_SUCCESS, Memory, PAGE_SIZE, PageStore};

#[test]
fn page_boundary_addresses() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    // Last byte of first page
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32 - 1), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 1);

    // First byte of second page
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 2);
}

#[test]
fn l2_table_boundary() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 10, 5);

    // Last page in first L2 table (256 pages per L2 table)
    let last_page_first_l2 = (256 * PAGE_SIZE - 1) as u32;
    assert_eq!(mem.allocate_page(last_page_first_l2), MEM_SUCCESS);
    assert_eq!(mem.num_l2_tables, 1);

    // First page in what would logically be the next L2 table
    // but due to the two-level page table design, L1 index changes at 4MB boundaries
    let first_page_second_l2 = (256 * PAGE_SIZE) as u32;
    assert_eq!(mem.allocate_page(first_page_second_l2), MEM_SUCCESS);
    // This creates a new L2 table since we're in a different part of the address space
    assert_eq!(mem.num_l2_tables, 2);
}

#[test]
fn max_address() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 255);

    // Maximum 32-bit address
    assert_eq!(mem.allocate_page(0xFFFFFFFF), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 1);
}

#[test]
fn all_l1_indices() {
    let mut store = PageStore::new(1024);
    let mut mem = Memory::new(&mut store, 1024, 255);

    // Test allocating pages that hit different L1 indices
    for i in 0..10 {
        let addr = (i as u32) << 22;
        assert_eq!(mem.allocate_page(addr), MEM_SUCCESS);
    }
    assert_eq!(mem.num_l2_tables, 10);
}

#[test]
fn all_l2_indices_in_table() {
    let mut store = PageStore::new(256);
    let mut mem = Memory::new(&mut store, 256, 10);

    // Allocate all 256 pages in a single L2 table
    for i in 0..256 {
        let addr = (i * PAGE_SIZE) as u32;
        assert_eq!(mem.allocate_page(addr), MEM_SUCCESS);
    }
    assert_eq!(mem.num_pages, 256);
    assert_eq!(mem.num_l2_tables, 1);
}
