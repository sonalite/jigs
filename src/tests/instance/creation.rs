use crate::{instance::Instance, module::Module};

#[test]
fn create_instance() {
    let instance = Instance::new();
    assert!(!instance.attached());
}

#[test]
fn create_instance_default() {
    let instance = Instance::default();
    assert!(!instance.attached());
}

#[test]
fn attach_to_module() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instance = Instance::new();
    instance.attach(&mut module);
    assert!(instance.attached());
    assert_eq!(module.instance_count, 1);
}

#[test]
fn detach_from_module() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instance = Instance::new();
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
    instance.detach();
    assert!(!instance.attached());
    assert_eq!(module.instance_count, 0);
}

#[test]
fn auto_detach_on_drop() {
    let mut module = Module::compile(&[]).unwrap();
    {
        let mut instance = Instance::new();
        instance.attach(&mut module);
        assert_eq!(module.instance_count, 1);
    }
    assert_eq!(module.instance_count, 0);
}

#[test]
fn multiple_instances_same_module() {
    let mut module = Module::compile(&[]).unwrap();
    let mut instance1 = Instance::new();
    let mut instance2 = Instance::new();
    instance1.attach(&mut module);
    instance2.attach(&mut module);
    assert_eq!(module.instance_count, 2);
    instance1.detach();
    assert_eq!(module.instance_count, 1);
    instance2.detach();
    assert_eq!(module.instance_count, 0);
}

#[test]
fn reattach_to_different_module() {
    let mut module1 = Module::compile(&[]).unwrap();
    let mut module2 = Module::compile(&[]).unwrap();
    let mut instance = Instance::new();

    instance.attach(&mut module1);
    assert_eq!(module1.instance_count, 1);
    assert_eq!(module2.instance_count, 0);

    instance.attach(&mut module2);
    assert_eq!(module1.instance_count, 0);
    assert_eq!(module2.instance_count, 1);
}

#[test]
fn detach_unattached() {
    let mut instance = Instance::new();
    instance.detach(); // Should not panic
    assert!(!instance.attached());
}
