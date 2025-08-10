use crate::{
    instance::Instance,
    memory::{Memory, PageStore},
    module::{CompileError, Module},
};

#[test]
fn new_empty() {
    let result = Module::new(1);
    assert!(result.is_ok());
}

#[test]
fn new_with_various_sizes() {
    let result = Module::new(1024);
    assert!(result.is_ok());

    let result = Module::new(4096);
    assert!(result.is_ok());

    let result = Module::new(16384);
    assert!(result.is_ok());
}

#[test]
fn set_code_on_new_module() {
    let mut module = Module::new(100).unwrap();
    let code = [0x00, 0x00, 0x00, 0x00];
    let result = module.set_code(&code);
    assert!(result.is_ok());
}

#[test]
fn set_code_multiple_times() {
    let mut module = Module::new(100).unwrap();

    // Set code first time
    let code1 = [0x00, 0x00, 0x00, 0x00];
    let result = module.set_code(&code1);
    assert!(result.is_ok());

    // Set code second time (should work as no instances attached)
    let code2 = [0xFF, 0xFF, 0xFF, 0xFF];
    let result = module.set_code(&code2);
    assert!(result.is_ok());
}

#[test]
fn set_code_with_attached_instance() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(100).unwrap();
    let mut instance = Instance::new(memory);

    // Attach instance
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);

    // Try to set code - should fail
    let code = [0x00, 0x00, 0x00, 0x00];
    let result = module.set_code(&code);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), CompileError::InstancesAttached);
}

#[test]
fn set_code_after_detaching_instance() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(100).unwrap();
    let mut instance = Instance::new(memory);

    // Attach and then detach instance
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
    instance.detach();
    assert_eq!(module.instance_count, 0);

    // Now setting code should work
    let code = [0x00, 0x00, 0x00, 0x00];
    let result = module.set_code(&code);
    assert!(result.is_ok());
}

#[test]
fn set_code_empty() {
    let mut module = Module::new(1).unwrap();
    let result = module.set_code(&[]);
    assert!(result.is_ok());
}

#[test]
fn set_code_with_data() {
    let mut module = Module::new(4).unwrap();
    let code = [0x00, 0x00, 0x00, 0x00];
    let result = module.set_code(&code);
    assert!(result.is_ok());
}

#[test]
fn initial_instance_count() {
    let module = Module::new(1).unwrap();
    assert_eq!(module.instance_count, 0);
}

#[test]
fn attach_instance() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(1).unwrap();
    let mut instance = Instance::new(memory);
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
}

#[test]
fn detach_instance() {
    let mut store = PageStore::new(100);
    let memory1 = Memory::new(&mut store, 50, 10);
    let memory2 = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(1).unwrap();
    let mut instance1 = Instance::new(memory1);
    let mut instance2 = Instance::new(memory2);
    instance1.attach(&mut module);
    instance2.attach(&mut module);
    assert_eq!(module.instance_count, 2);
    instance1.detach();
    assert_eq!(module.instance_count, 1);
}

#[test]
fn multiple_attachments() {
    let mut store = PageStore::new(500);
    let mut module = Module::new(1).unwrap();
    let mut instances = Vec::new();
    for _ in 0..5 {
        let memory = Memory::new(&mut store, 50, 10);
        let mut instance = Instance::new(memory);
        instance.attach(&mut module);
        instances.push(instance);
    }
    assert_eq!(module.instance_count, 5);
}

#[test]
#[should_panic(expected = "Module dropped with 1 attached instances")]
fn drop_with_attached_instance() {
    let mut module = Module::new(1).unwrap();
    module.instance_count = 1;
    drop(module);
}

#[test]
#[should_panic(expected = "Module dropped with 3 attached instances")]
fn drop_with_multiple_attached_instances() {
    let mut module = Module::new(1).unwrap();
    module.instance_count = 3;
    drop(module);
}

#[test]
fn drop_after_detach() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(1).unwrap();
    let mut instance = Instance::new(memory);
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
    drop(instance);
    assert_eq!(module.instance_count, 0);
}

#[test]
fn code_buffer_allocation() {
    // Test with various code sizes
    let result = Module::new(1024);
    assert!(result.is_ok());

    let result = Module::new(4096);
    assert!(result.is_ok());

    let result = Module::new(16384);
    assert!(result.is_ok());
}

#[test]
fn code_buffer_size_multiplier() {
    // The buffer should be allocated with the multiplier
    // We can ensure set_code works with code up to the specified size
    let mut module = Module::new(1024).unwrap();
    let code = vec![0u8; 1024];
    let result = module.set_code(&code);
    assert!(result.is_ok());
}

#[test]
fn code_buffer_alignment() {
    // Test that various sizes work (alignment is ensured internally)
    // Sizes that aren't 4-byte aligned should still work
    let result = Module::new(1);
    assert!(result.is_ok());

    let result = Module::new(3);
    assert!(result.is_ok());

    let result = Module::new(5);
    assert!(result.is_ok());

    let result = Module::new(1023);
    assert!(result.is_ok());
}

#[test]
fn new_allocation_failure() {
    // Try to allocate an impossibly large buffer to trigger allocation failure
    // This will try to allocate ~16 exabytes (4 * 4 exabytes), which will definitely fail
    let result = Module::new(usize::MAX / 4);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, crate::module::CompileError::AllocationFailed);
    }
}

#[test]
fn set_code_too_large() {
    // Create a module with small buffer
    let mut module = Module::new(10).unwrap();

    // Try to set code that's larger than the buffer capacity
    // The module can hold 10 * 4 = 40 bytes of ARM64 code
    // So trying to set 11 bytes of RISC-V code should fail
    let code = vec![0u8; 11];
    let result = module.set_code(&code);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), CompileError::CodeTooLarge);
}

#[test]
fn set_code_exactly_at_limit() {
    // Create a module with specific buffer size
    let mut module = Module::new(10).unwrap();

    // Set code that exactly fits the buffer
    let code = vec![0u8; 10];
    let result = module.set_code(&code);
    assert!(result.is_ok());
}
