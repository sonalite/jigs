use crate::memory::{
    MEM_ERR_NO_L2_TABLES, MEM_ERR_PAGE_LIMIT, MEM_SUCCESS, Memory, PAGE_SIZE, PageStore,
};

#[test]
fn zero_capacity_memory() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 0, 0);

    // Can't allocate anything - hits L2 table limit first since we have 0 L2 tables
    assert_eq!(mem.allocate_page(0), MEM_ERR_NO_L2_TABLES);
    assert_eq!(mem.num_pages, 0);
}

#[test]
fn zero_l2_tables() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 10, 0);

    // Can't allocate because no L2 tables allowed
    assert_eq!(mem.allocate_page(0), MEM_ERR_NO_L2_TABLES);
    assert_eq!(mem.num_l2_tables, 0);
}

#[test]
fn single_page_single_l2() {
    let mut store = PageStore::new(1);
    let mut mem = Memory::new(&mut store, 1, 1);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_ERR_PAGE_LIMIT); // No more pages
    assert_eq!(mem.allocate_page(1 << 22), MEM_ERR_NO_L2_TABLES); // Would need new L2 table
}

#[test]
fn alternating_l2_allocation() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 10, 5);

    // Allocate pages that alternate between L2 tables
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.allocate_page((1 << 22) + PAGE_SIZE as u32), MEM_SUCCESS);

    assert_eq!(mem.num_pages, 4);
    assert_eq!(mem.num_l2_tables, 2);
}

#[test]
fn exact_limits() {
    let mut store = PageStore::new(3);
    let mut mem = Memory::new(&mut store, 3, 2);

    // Allocate exactly to limits
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS);

    // All limits reached
    assert_eq!(mem.allocate_page(2 * PAGE_SIZE as u32), MEM_ERR_PAGE_LIMIT);
    assert_eq!(mem.allocate_page(2 << 22), MEM_ERR_NO_L2_TABLES);

    assert_eq!(mem.num_pages, 3);
    assert_eq!(mem.num_l2_tables, 2);
}
