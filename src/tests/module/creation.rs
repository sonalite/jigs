use crate::{instance::Instance, module::Module};

#[test]
fn compile_empty() {
    let result = Module::compile(&[]);
    assert!(result.is_ok());
}

#[test]
fn compile_with_code() {
    let code = [0x00, 0x00, 0x00, 0x00];
    let result = Module::compile(&code);
    assert!(result.is_ok());
}

#[test]
fn initial_instance_count() {
    let module = Module::compile(&[]).unwrap();
    assert_eq!(module.instance_count, 0);
}

#[test]
fn attach_instance() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instance = Instance::new();
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
}

#[test]
fn detach_instance() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instance1 = Instance::new();
    let mut instance2 = Instance::new();
    instance1.attach(&mut module);
    instance2.attach(&mut module);
    assert_eq!(module.instance_count, 2);
    instance1.detach();
    assert_eq!(module.instance_count, 1);
}

#[test]
fn multiple_attachments() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instances = Vec::new();
    for _ in 0..5 {
        let mut instance = Instance::new();
        instance.attach(&mut module);
        instances.push(instance);
    }
    assert_eq!(module.instance_count, 5);
}

#[test]
#[should_panic(expected = "Module dropped with 1 attached instances")]
fn drop_with_attached_instance() {
    let mut module = Module::compile(&[]).unwrap();
    module.instance_count = 1;
    drop(module);
}

#[test]
#[should_panic(expected = "Module dropped with 3 attached instances")]
fn drop_with_multiple_attached_instances() {
    let mut module = Module::compile(&[]).unwrap();
    module.instance_count = 3;
    drop(module);
}

#[test]
fn drop_after_detach() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instance = Instance::new();
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
    drop(instance);
    assert_eq!(module.instance_count, 0);
}
