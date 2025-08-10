use crate::{
    instance::Instance,
    memory::{Memory, PageStore},
    module::Module,
};

#[test]
fn compile_empty() {
    // Even with empty code, we need a non-zero max_code_size
    let result = Module::compile(&[], 1);
    assert!(result.is_ok());
}

#[test]
fn compile_with_code() {
    let code = [0x00, 0x00, 0x00, 0x00];
    let result = Module::compile(&code, code.len());
    assert!(result.is_ok());
}

#[test]
fn initial_instance_count() {
    let module = Module::compile(&[], 1).unwrap();
    assert_eq!(module.instance_count, 0);
}

#[test]
fn attach_instance() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::compile(&[], 1).unwrap();
    let mut instance = Instance::new(memory);
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
}

#[test]
fn detach_instance() {
    let mut store = PageStore::new(100);
    let memory1 = Memory::new(&mut store, 50, 10);
    let memory2 = Memory::new(&mut store, 50, 10);
    let mut module = Module::compile(&[], 1).unwrap();
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
    let mut module = Module::compile(&[], 1).unwrap();
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
    let mut module = Module::compile(&[], 1).unwrap();
    module.instance_count = 1;
    drop(module);
}

#[test]
#[should_panic(expected = "Module dropped with 3 attached instances")]
fn drop_with_multiple_attached_instances() {
    let mut module = Module::compile(&[], 1).unwrap();
    module.instance_count = 3;
    drop(module);
}

#[test]
fn drop_after_detach() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::compile(&[], 1).unwrap();
    let mut instance = Instance::new(memory);
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
    drop(instance);
    assert_eq!(module.instance_count, 0);
}

#[test]
fn code_buffer_allocation() {
    // Test with various code sizes
    let result = Module::compile(&[], 1024);
    assert!(result.is_ok());

    let result = Module::compile(&[], 4096);
    assert!(result.is_ok());

    let result = Module::compile(&[], 16384);
    assert!(result.is_ok());
}

#[test]
fn code_buffer_size_multiplier() {
    // The buffer should be allocated with the multiplier
    // We can't directly test the internal size, but we can ensure
    // it compiles successfully with various sizes
    let code = vec![0u8; 1024];
    let result = Module::compile(&code, code.len());
    assert!(result.is_ok());
}

#[test]
fn code_buffer_alignment() {
    // Test that various sizes work (alignment is ensured internally)
    // Sizes that aren't 4-byte aligned should still work
    let result = Module::compile(&[], 1);
    assert!(result.is_ok());

    let result = Module::compile(&[], 3);
    assert!(result.is_ok());

    let result = Module::compile(&[], 5);
    assert!(result.is_ok());

    let result = Module::compile(&[], 1023);
    assert!(result.is_ok());
}

#[test]
fn allocation_failure() {
    // Try to allocate an impossibly large buffer to trigger allocation failure
    // This will try to allocate ~16 exabytes (4 * 4 exabytes), which will definitely fail
    let result = Module::compile(&[], usize::MAX / 4);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e, crate::module::CompileError::AllocationFailed);
    }
}
