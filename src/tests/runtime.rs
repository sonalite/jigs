use crate::{Instance, Memory, Module, PageStore};

#[test]
fn call_function_without_module() {
    let mut page_store = PageStore::new(256); // 256 pages (1MB with 4KB pages)
    let memory = Memory::new(&mut page_store, 256, 16);
    let mut instance = Instance::new(memory);

    let result = unsafe { instance.call_function(0) };

    assert_eq!(result, Err("Instance not attached to module"));
}

#[test]
fn call_function_with_empty_module() {
    let mut page_store = PageStore::new(256); // 256 pages (1MB with 4KB pages)
    let memory = Memory::new(&mut page_store, 256, 16);
    let mut instance = Instance::new(memory);
    let mut module = Module::new(1024).unwrap();

    instance.attach(&mut module);

    let result = unsafe { instance.call_function(0) };

    assert_eq!(result, Err("Module has no compiled code"));

    instance.detach();
}

#[cfg(target_arch = "aarch64")]
#[test]
fn call_function_with_ret_instruction() {
    let mut page_store = PageStore::new(256); // 256 pages (1MB with 4KB pages)
    let memory = Memory::new(&mut page_store, 256, 16);
    let mut instance = Instance::new(memory);
    let mut module = Module::new(1024).unwrap();

    // Create a simple RISC-V program (doesn't matter what it is, will compile to RET)
    let riscv_code = vec![
        0x00, 0x00, 0x00, 0x00, // NOP (addi x0, x0, 0)
    ];

    module.set_code(&riscv_code).unwrap();
    instance.attach(&mut module);

    // This should execute the RET instruction and return without crashing
    let result = unsafe { instance.call_function(0) };

    assert_eq!(result, Ok(()));

    instance.detach();
}
