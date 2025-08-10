use crate::memory::{
    MEM_ERR_NO_L2_TABLES, MEM_ERR_PAGE_LIMIT, MEM_SUCCESS, Memory, PAGE_OFFSET_MASK, PAGE_SIZE,
    PageStore,
};

#[test]
fn single_page() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 1);
    assert_eq!(mem.num_l2_tables, 1);
    assert_eq!(store.num_available_pages, 9);
}

#[test]
fn same_page_twice() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 1);

    // Allocating same page again should succeed without allocating new page
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 1);
}

#[test]
fn different_pages_same_l2() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    // These addresses map to same L2 table but different pages
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32 * 2), MEM_SUCCESS);

    assert_eq!(mem.num_pages, 3);
    assert_eq!(mem.num_l2_tables, 1);
}

#[test]
fn different_l2_tables() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    // These addresses require different L2 tables
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS); // Different L1 index

    assert_eq!(mem.num_pages, 2);
    assert_eq!(mem.num_l2_tables, 2);
}

#[test]
fn max_pages_limit() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 2, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32 * 2), MEM_ERR_PAGE_LIMIT); // Should fail

    assert_eq!(mem.num_pages, 2);
}

#[test]
fn max_l2_tables_limit() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 10, 2);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(2 << 22), MEM_ERR_NO_L2_TABLES); // Should fail - no more L2 tables

    assert_eq!(mem.num_l2_tables, 2);
}

#[test]
fn pagestore_exhaustion() {
    let mut store = PageStore::new(2);
    let mut mem = Memory::new(&mut store, 2, 3); // Can't exceed PageStore's available pages

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32 * 2), MEM_ERR_PAGE_LIMIT); // Should fail - Instance page limit reached

    assert_eq!(mem.num_pages, 2);
    assert_eq!(store.num_available_pages, 0);
}

#[test]
fn address_components() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    // Test various address patterns
    let test_addr = 0x12345678;
    assert_eq!(mem.allocate_page(test_addr), MEM_SUCCESS);

    // Verify we allocated exactly one page
    assert_eq!(mem.num_pages, 1);

    // Same page, different offset
    assert_eq!(
        mem.allocate_page(test_addr & !PAGE_OFFSET_MASK),
        MEM_SUCCESS
    );
    assert_eq!(mem.num_pages, 1);
}

#[test]
fn allocated_indices_tracking() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);

    // Verify allocated indices are tracked correctly
    assert_eq!(mem.allocated_indices[0], 9); // First allocation gets last available
    assert_eq!(mem.allocated_indices[1], 8); // Second gets next
}
