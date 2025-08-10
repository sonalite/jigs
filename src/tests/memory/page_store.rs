use crate::memory::{MAX_PAGES, PAGE_SIZE, PageStore};

#[test]
fn basic() {
    let store = PageStore::new(10);
    assert_eq!(store.num_available_pages, 10);
    assert_eq!(store.instance_count, 0);
    assert_eq!(store.page_memory_size, 10 * PAGE_SIZE);
    assert_eq!(store.available_pages_capacity, 10);
}

#[test]
fn zero_pages() {
    let store = PageStore::new(0);
    assert_eq!(store.num_available_pages, 0);
    assert_eq!(store.page_memory_size, 0);
    assert_eq!(store.available_pages_capacity, 0);
}

#[test]
fn max_pages() {
    let store = PageStore::new(MAX_PAGES);
    assert_eq!(store.num_available_pages, MAX_PAGES);
    assert_eq!(store.page_memory_size, MAX_PAGES * PAGE_SIZE);
    assert_eq!(store.available_pages_capacity, MAX_PAGES);
}

#[test]
#[should_panic(expected = "total_pages 65536 exceeds maximum allowed")]
fn exceeds_max_pages() {
    PageStore::new(MAX_PAGES + 1);
}

#[test]
fn available_pages_initialization() {
    let store = PageStore::new(5);
    unsafe {
        assert_eq!(*store.available_pages.add(0), 0);
        assert_eq!(*store.available_pages.add(1), 1);
        assert_eq!(*store.available_pages.add(2), 2);
        assert_eq!(*store.available_pages.add(3), 3);
        assert_eq!(*store.available_pages.add(4), 4);
    }
}

#[test]
fn page_memory_zeroed() {
    let store = PageStore::new(2);
    unsafe {
        for i in 0..store.page_memory_size {
            assert_eq!(*store.page_memory.add(i), 0);
        }
    }
}

#[test]
fn drop_with_no_instances() {
    let store = PageStore::new(10);
    drop(store); // Should not panic
}

#[test]
#[should_panic(expected = "PageStore dropped while 1 Memory instance(s) still exist")]
fn drop_with_active_instance() {
    let mut store = PageStore::new(10);
    store.instance_count = 1;
    drop(store);
}

#[test]
#[should_panic(expected = "PageStore dropped while 3 Memory instance(s) still exist")]
fn drop_with_multiple_instances() {
    let mut store = PageStore::new(10);
    store.instance_count = 3;
    drop(store);
}
