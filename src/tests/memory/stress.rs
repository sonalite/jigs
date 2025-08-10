use crate::memory::{MEM_ERR_NO_PAGES_AVAILABLE, MEM_SUCCESS, Memory, PAGE_SIZE, PageStore};

#[test]
fn allocate_many_pages() {
    let mut store = PageStore::new(1000);
    let mut mem = Memory::new(&mut store, 1000, 100);

    // Allocate 500 pages
    for i in 0..500 {
        let addr = (i * PAGE_SIZE) as u32;
        assert_eq!(mem.allocate_page(addr), MEM_SUCCESS);
    }
    assert_eq!(mem.num_pages, 500);
}

#[test]
fn allocate_reset_cycle() {
    let mut store = PageStore::new(100);
    let mut mem = Memory::new(&mut store, 50, 20);

    for _ in 0..10 {
        // Allocate some pages
        for i in 0..10 {
            assert_eq!(mem.allocate_page((i * PAGE_SIZE) as u32), MEM_SUCCESS);
        }
        assert_eq!(mem.num_pages, 10);

        // Reset
        mem.reset();
        assert_eq!(mem.num_pages, 0);
        assert_eq!(store.num_available_pages, 100);
    }
}

#[test]
fn sparse_allocation() {
    let mut store = PageStore::new(100);
    let mut mem = Memory::new(&mut store, 100, 50);

    // Allocate pages with large gaps
    let addresses = [0, 1 << 20, 1 << 24, 1 << 28, 0xF0000000];

    for &addr in &addresses {
        assert_eq!(mem.allocate_page(addr), MEM_SUCCESS);
    }
    assert_eq!(mem.num_pages, 5);
}

#[test]
fn random_pattern_allocation() {
    let mut store = PageStore::new(100);
    let mut mem = Memory::new(&mut store, 100, 50);

    // Pseudo-random but deterministic pattern
    let mut addr = 0x12345678u32;
    for _ in 0..50 {
        assert_eq!(mem.allocate_page(addr), MEM_SUCCESS);
        addr = addr.wrapping_mul(1664525).wrapping_add(1013904223);
    }
    assert_eq!(mem.num_pages, 50);
}

#[test]
fn multiple_instances_sharing_store() {
    let mut store = PageStore::new(100);

    let mut mem1 = Memory::new(&mut store, 30, 10);
    let mut mem2 = Memory::new(&mut store, 30, 10);

    // Allocate from first instance
    for i in 0..20 {
        assert_eq!(mem1.allocate_page((i * PAGE_SIZE) as u32), MEM_SUCCESS);
    }

    // Allocate from second instance
    for i in 0..20 {
        assert_eq!(mem2.allocate_page((i * PAGE_SIZE) as u32), MEM_SUCCESS);
    }

    assert_eq!(mem1.num_pages, 20);
    assert_eq!(mem2.num_pages, 20);
    assert_eq!(store.num_available_pages, 60);

    // Reset first instance
    mem1.reset();
    assert_eq!(store.num_available_pages, 80);

    // Second instance still has its pages
    assert_eq!(mem2.num_pages, 20);

    // Reset second instance
    mem2.reset();
    assert_eq!(store.num_available_pages, 100);
}

#[test]
fn exhaust_and_recover() {
    let mut store = PageStore::new(10);
    let mut mem1 = Memory::new(&mut store, 10, 5);
    let mut mem2 = Memory::new(&mut store, 10, 5);

    // Exhaust store with first instance
    for i in 0..10 {
        assert_eq!(mem1.allocate_page((i * PAGE_SIZE) as u32), MEM_SUCCESS);
    }
    assert_eq!(store.num_available_pages, 0);

    // Second instance can't allocate
    assert_eq!(mem2.allocate_page(0), MEM_ERR_NO_PAGES_AVAILABLE);

    // Reset first instance
    mem1.reset();
    assert_eq!(store.num_available_pages, 10);

    // Now second instance can allocate
    assert_eq!(mem2.allocate_page(0), MEM_SUCCESS);
    assert_eq!(store.num_available_pages, 9);
}
