use crate::memory::{MAX_L2_TABLES, MAX_PAGES, MEM_SUCCESS, Memory, PageStore};

#[test]
fn basic() {
    let mut store = PageStore::new(100);
    let mem = Memory::new(&mut store, 50, 10);
    assert_eq!(mem.num_pages, 0);
    assert_eq!(mem.max_pages, 50);
    assert_eq!(mem.num_l2_tables, 0);
    assert_eq!(mem.max_l2_tables, 10);
    assert_eq!(store.instance_count, 1);
}

#[test]
fn zero_limits() {
    let mut store = PageStore::new(100);
    let mem = Memory::new(&mut store, 0, 0);
    assert_eq!(mem.max_pages, 0);
    assert_eq!(mem.max_l2_tables, 0);
}

#[test]
fn max_limits() {
    let mut store = PageStore::new(MAX_PAGES); // Need enough pages for max allocation
    let mem = Memory::new(&mut store, MAX_PAGES, MAX_L2_TABLES);
    assert_eq!(mem.max_pages, MAX_PAGES);
    assert_eq!(mem.max_l2_tables, MAX_L2_TABLES);
}

#[test]
#[should_panic(expected = "max_pages 65536 exceeds maximum allowed")]
fn exceeds_max_pages() {
    let mut store = PageStore::new(100);
    Memory::new(&mut store, MAX_PAGES + 1, 10);
}

#[test]
#[should_panic(expected = "max_l2_tables 256 exceeds maximum allowed")]
fn exceeds_max_l2_tables() {
    let mut store = PageStore::new(100);
    Memory::new(&mut store, 100, MAX_L2_TABLES + 1);
}

#[test]
#[should_panic(expected = "max_pages 101 exceeds available pages in PageStore (100)")]
fn exceeds_available_pages() {
    let mut store = PageStore::new(100);
    Memory::new(&mut store, 101, 10);
}

#[test]
fn drop_decrements_instance_count() {
    let mut store = PageStore::new(100);
    assert_eq!(store.instance_count, 0);
    {
        let _mem = Memory::new(&mut store, 50, 10);
        assert_eq!(store.instance_count, 1);
    }
    assert_eq!(store.instance_count, 0);
}

#[test]
fn multiple_instances() {
    let mut store = PageStore::new(100);
    assert_eq!(store.instance_count, 0);

    let mem1 = Memory::new(&mut store, 30, 5);
    assert_eq!(store.instance_count, 1);

    let mem2 = Memory::new(&mut store, 30, 5);
    assert_eq!(store.instance_count, 2);

    drop(mem1);
    assert_eq!(store.instance_count, 1);

    drop(mem2);
    assert_eq!(store.instance_count, 0);
}

#[test]
fn debug_format() {
    let mut store = PageStore::new(100);
    let mem = Memory::new(&mut store, 50, 10);
    let debug_str = format!("{:?}", mem);
    assert!(debug_str.contains("Memory"));
    assert!(debug_str.contains("num_pages: 0"));
    assert!(debug_str.contains("max_pages: 50"));
    assert!(debug_str.contains("num_l2_tables: 0"));
    assert!(debug_str.contains("max_l2_tables: 10"));
    assert!(debug_str.contains("l2_coverage_mb: 0"));
}

#[test]
fn debug_format_with_l2_tables() {
    let mut store = PageStore::new(100);
    let mut mem = Memory::new(&mut store, 50, 10);

    // Allocate a page to force L2 table allocation
    assert_eq!(mem.allocate_page(0), MEM_SUCCESS);

    let debug_str = format!("{:?}", mem);
    assert!(debug_str.contains("num_l2_tables: 1"));
    assert!(debug_str.contains("l2_coverage_mb: 4"));
}
