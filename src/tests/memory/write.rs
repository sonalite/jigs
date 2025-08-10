use crate::memory::{
    MEM_ERR_NO_L2_TABLES, MEM_ERR_NO_PAGES_AVAILABLE, MEM_ERR_PAGE_LIMIT, MEM_SUCCESS, Memory,
    PAGE_SIZE, PageStore,
};

#[test]
fn empty_buffer() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = [];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
}

#[test]
fn single_byte_new_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = [42];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = [0];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer[0], 42);
}

#[test]
fn multiple_bytes_same_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = vec![0; 10];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_across_page_boundary() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = PAGE_SIZE as u32 - 2;
    let buffer = vec![0xAA, 0xBB, 0xCC, 0xDD];
    assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 2);
    let mut read_buffer = vec![0; 4];
    memory.read(addr, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_multiple_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = vec![0x11; PAGE_SIZE * 3];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 3);
    let mut read_buffer = vec![0; PAGE_SIZE * 3];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_with_offset_in_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = 100;
    let buffer = vec![0x42; 100];
    assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = vec![0; 100];
    memory.read(addr, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn overwrite_existing_data() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer1 = vec![0x11; 100];
    let buffer2 = vec![0x22; 100];
    assert_eq!(memory.write(0, &buffer1), MEM_SUCCESS);
    assert_eq!(memory.write(0, &buffer2), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = vec![0; 100];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer, buffer2);
}

#[test]
fn partial_overwrite() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer1 = vec![0x11; 10];
    let buffer2 = vec![0x22; 5];
    assert_eq!(memory.write(0, &buffer1), MEM_SUCCESS);
    assert_eq!(memory.write(2, &buffer2), MEM_SUCCESS);
    let mut read_buffer = vec![0; 10];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer[0], 0x11);
    assert_eq!(read_buffer[1], 0x11);
    for i in 2..7 {
        assert_eq!(read_buffer[i], 0x22);
    }
    for i in 7..10 {
        assert_eq!(read_buffer[i], 0x11);
    }
}

#[test]
fn write_entire_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = vec![0x55; PAGE_SIZE];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = vec![0; PAGE_SIZE];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_at_page_boundary() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = PAGE_SIZE as u32 - 1;
    let buffer = [0x99];
    assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = [0];
    memory.read(addr, &mut read_buffer);
    assert_eq!(read_buffer[0], 0x99);
}

#[test]
fn write_sparse_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr1 = 0;
    let addr2 = 10 * PAGE_SIZE as u32;
    let buffer1 = [0x11];
    let buffer2 = [0x22];
    assert_eq!(memory.write(addr1, &buffer1), MEM_SUCCESS);
    assert_eq!(memory.write(addr2, &buffer2), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 2);
    let mut read1 = [0];
    let mut read2 = [0];
    let mut read_middle = [0];
    memory.read(addr1, &mut read1);
    memory.read(addr2, &mut read2);
    memory.read(PAGE_SIZE as u32, &mut read_middle);
    assert_eq!(read1[0], 0x11);
    assert_eq!(read2[0], 0x22);
    assert_eq!(read_middle[0], 0);
}

#[test]
fn write_allocates_l2_table() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let high_addr = 0x40000000;
    let buffer = [0x77];
    assert_eq!(memory.num_l2_tables, 0);
    assert_eq!(memory.write(high_addr, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_l2_tables, 1);
    let mut read_buffer = [0];
    memory.read(high_addr, &mut read_buffer);
    assert_eq!(read_buffer[0], 0x77);
}

#[test]
fn write_multiple_l2_tables() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr1 = 0;
    let addr2 = 0x40000000;
    let buffer = [0x88];
    assert_eq!(memory.write(addr1, &buffer), MEM_SUCCESS);
    assert_eq!(memory.write(addr2, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_l2_tables, 2);
}

#[test]
fn write_error_no_l2_tables() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 1);
    let addr1 = 0;
    let addr2 = 0x40000000;
    let buffer = [0x11];
    assert_eq!(memory.write(addr1, &buffer), MEM_SUCCESS);
    assert_eq!(memory.write(addr2, &buffer), MEM_ERR_NO_L2_TABLES);
    assert_eq!(memory.num_l2_tables, 1);
}

#[test]
fn write_error_page_limit() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 2, 2);
    let buffer = [0x11];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.write(PAGE_SIZE as u32, &buffer), MEM_SUCCESS);
    assert_eq!(
        memory.write(2 * PAGE_SIZE as u32, &buffer),
        MEM_ERR_PAGE_LIMIT
    );
    assert_eq!(memory.num_pages, 2);
}

#[test]
fn write_error_no_pages_available() {
    let mut store = PageStore::new(2);
    let mut mem1 = Memory::new(&mut store, 2, 1);
    let mut mem2 = Memory::new(&mut store, 2, 1);
    let buffer = [0x11];
    assert_eq!(mem1.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(mem2.write(0, &buffer), MEM_SUCCESS);
    // Now all pages in the store are allocated, next write should fail with NO_PAGES_AVAILABLE
    assert_eq!(
        mem2.write(PAGE_SIZE as u32, &buffer),
        MEM_ERR_NO_PAGES_AVAILABLE
    );
}

#[test]
fn write_error_stops_on_first_failure() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 2, 2);
    let buffer = vec![0x11; PAGE_SIZE * 3];
    let result = memory.write(0, &buffer);
    assert_eq!(result, MEM_ERR_PAGE_LIMIT);
    assert_eq!(memory.num_pages, 2);
}

#[test]
fn write_with_high_l1_index() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let high_addr = 0xFFC00000;
    let buffer = [0x99];
    assert_eq!(memory.write(high_addr, &buffer), MEM_SUCCESS);
    let mut read_buffer = [0];
    memory.read(high_addr, &mut read_buffer);
    assert_eq!(read_buffer[0], 0x99);
}

#[test]
fn write_with_high_l2_index() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = (255 << 14) as u32;
    let buffer = [0x88];
    assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    let mut read_buffer = [0];
    memory.read(addr, &mut read_buffer);
    assert_eq!(read_buffer[0], 0x88);
}

#[test]
fn write_all_page_offsets() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    for offset in 0..PAGE_SIZE {
        let buffer = [(offset % 256) as u8];
        assert_eq!(memory.write(offset as u32, &buffer), MEM_SUCCESS);
    }
    assert_eq!(memory.num_pages, 1);
    for offset in 0..PAGE_SIZE {
        let mut read_buffer = [0];
        memory.read(offset as u32, &mut read_buffer);
        assert_eq!(read_buffer[0], (offset % 256) as u8);
    }
}

#[test]
fn write_crosses_multiple_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 4);
    let start = PAGE_SIZE / 2;
    let buffer = vec![0x44; PAGE_SIZE * 3];
    assert_eq!(memory.write(start as u32, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 4);
    let mut read_buffer = vec![0; PAGE_SIZE * 3];
    memory.read(start as u32, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_after_reset() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer1 = [0x11];
    assert_eq!(memory.write(0, &buffer1), MEM_SUCCESS);
    memory.reset();
    let buffer2 = [0x22];
    assert_eq!(memory.write(0, &buffer2), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 1);
    let mut read_buffer = [0];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer[0], 0x22);
}

#[test]
fn write_exact_page_alignment() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = vec![0x66; PAGE_SIZE * 2];
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 2);
    let mut read_buffer = vec![0; PAGE_SIZE * 2];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_with_wraparound() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = 0xFFFFFFFC;
    let buffer = vec![0xF0, 0xF1, 0xF2, 0xF3, 0xF4, 0xF5, 0xF6, 0xF7];
    assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    let mut read_buffer = vec![0; 8];
    memory.read(addr, &mut read_buffer);
    assert_eq!(read_buffer[0], 0xF0);
    assert_eq!(read_buffer[1], 0xF1);
    assert_eq!(read_buffer[2], 0xF2);
    assert_eq!(read_buffer[3], 0xF3);
    for i in 4..8 {
        assert_eq!(read_buffer[i], buffer[i]);
    }
}

#[test]
fn write_incremental_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 3);
    for page in 0..3 {
        let addr = page * PAGE_SIZE as u32;
        let buffer = vec![(page + 1) as u8; 100];
        assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    }
    assert_eq!(memory.num_pages, 3);
    for page in 0..3 {
        let addr = page * PAGE_SIZE as u32;
        let mut read_buffer = vec![0; 100];
        memory.read(addr, &mut read_buffer);
        assert!(read_buffer.iter().all(|&b| b == (page + 1) as u8));
    }
}

#[test]
fn write_pattern_verification() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let pattern: Vec<u8> = (0..256).map(|i| i as u8).collect();
    for offset in [0, 1, 7, 8, 15, 16, 31, 32, 63, 64, 127, 128] {
        let addr = offset * 100;
        assert_eq!(memory.write(addr, &pattern), MEM_SUCCESS);
        let mut read_buffer = vec![0; 256];
        memory.read(addr, &mut read_buffer);
        assert_eq!(read_buffer, pattern);
    }
}

#[test]
fn write_zero_bytes_at_various_addresses() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let buffer = vec![0; 100];
    for addr in [0, 100, 1000, 10000, PAGE_SIZE as u32, 0x100000] {
        assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
        let mut read_buffer = vec![0xFF; 100];
        memory.read(addr, &mut read_buffer);
        assert!(read_buffer.iter().all(|&b| b == 0));
    }
}

#[test]
fn write_large_buffer_performance() {
    let mut store = PageStore::new(100);
    let mut memory = Memory::new(&mut store, 50, 10);
    let buffer: Vec<u8> = (0..PAGE_SIZE * 10).map(|i| (i % 256) as u8).collect();
    assert_eq!(memory.write(0, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 10);
    let mut read_buffer = vec![0; PAGE_SIZE * 10];
    memory.read(0, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}

#[test]
fn write_single_byte_each_page() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 3);
    for page in 0..3 {
        let addr = page * PAGE_SIZE as u32;
        let buffer = [(page + 1) as u8];
        assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    }
    assert_eq!(memory.num_pages, 3);
    for page in 0..3 {
        let addr = page * PAGE_SIZE as u32;
        let mut read_buffer = [0];
        memory.read(addr, &mut read_buffer);
        assert_eq!(read_buffer[0], (page + 1) as u8);
    }
}

#[test]
fn write_reuses_allocated_pages() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    assert_eq!(memory.write(0, &[0x11]), MEM_SUCCESS);
    let pages_after_first = memory.num_pages;
    assert_eq!(memory.write(1, &[0x22]), MEM_SUCCESS);
    assert_eq!(memory.num_pages, pages_after_first);
    let mut buffer = vec![0; 2];
    memory.read(0, &mut buffer);
    assert_eq!(buffer[0], 0x11);
    assert_eq!(buffer[1], 0x22);
}

#[test]
fn write_partial_page_at_end() {
    let mut store = PageStore::new(10);
    let mut memory = Memory::new(&mut store, 5, 2);
    let addr = PAGE_SIZE as u32 - 10;
    let buffer = vec![0xEE; 20];
    assert_eq!(memory.write(addr, &buffer), MEM_SUCCESS);
    assert_eq!(memory.num_pages, 2);
    let mut read_buffer = vec![0; 20];
    memory.read(addr, &mut read_buffer);
    assert_eq!(read_buffer, buffer);
}
