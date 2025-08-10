use crate::memory::{MEM_SUCCESS, Memory, PAGE_SIZE, PageStore, UNMAPPED_PAGE};

#[test]
fn empty_memory() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    mem.reset();
    assert_eq!(mem.num_pages, 0);
    assert_eq!(mem.num_l2_tables, 0);
}

#[test]
fn single_page() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 1);
    assert_eq!(store.num_available_pages, 9);

    mem.reset();
    assert_eq!(mem.num_pages, 0);
    assert_eq!(mem.num_l2_tables, 0);
    assert_eq!(store.num_available_pages, 10);
}

#[test]
fn multiple_pages() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 3);
    assert_eq!(mem.num_l2_tables, 2);

    mem.reset();
    assert_eq!(mem.num_pages, 0);
    assert_eq!(mem.num_l2_tables, 0);
    assert_eq!(store.num_available_pages, 10);
}

#[test]
fn memory_cleared() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);

    // Write some data to the page
    let page_idx = mem.allocated_indices[0] as usize;
    let offset = page_idx * PAGE_SIZE;
    store.page_memory[offset] = 0x42;
    store.page_memory[offset + 1] = 0x43;

    mem.reset();

    // Verify memory was cleared
    assert_eq!(store.page_memory[offset], 0);
    assert_eq!(store.page_memory[offset + 1], 0);
}

#[test]
fn can_reallocate_after_reset() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS);
    mem.reset();

    // Should be able to allocate again
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);
    assert_eq!(mem.num_pages, 2);
    assert_eq!(mem.num_l2_tables, 1);
}

#[test]
fn l1_table_cleared() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(1 << 22), MEM_SUCCESS);

    // Verify L1 entries are set
    assert_ne!(mem.l1_table[0], 0xFF);
    assert_ne!(mem.l1_table[1], 0xFF);

    mem.reset();

    // Verify L1 table is cleared
    for entry in mem.l1_table.iter() {
        assert_eq!(*entry, 0xFF);
    }
}

#[test]
fn l2_tables_cleared() {
    let mut store = PageStore::new(10);
    let mut mem = Memory::new(&mut store, 5, 3);

    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);
    assert_eq!(mem.allocate_page(PAGE_SIZE as u32), MEM_SUCCESS);

    // Verify L2 entries are set
    assert_ne!(mem.l2_tables[0][0], UNMAPPED_PAGE);
    assert_ne!(mem.l2_tables[0][1], UNMAPPED_PAGE);

    mem.reset();

    // Verify L2 tables are cleared
    for l2_table in mem.l2_tables.iter() {
        for entry in l2_table.iter() {
            assert_eq!(*entry, UNMAPPED_PAGE);
        }
    }
}
