use crate::{
    instance::Instance,
    memory::{Memory, PageStore},
    module::Module,
};

#[test]
fn create_instance() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let instance = Instance::new(memory);
    assert!(!instance.attached());
}

#[test]
fn attach_to_module() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(1).unwrap();
    let mut instance = Instance::new(memory);
    instance.attach(&mut module);
    assert!(instance.attached());
    assert_eq!(module.instance_count, 1);
}

#[test]
fn detach_from_module() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module = Module::new(1).unwrap();
    let mut instance = Instance::new(memory);
    instance.attach(&mut module);
    assert_eq!(module.instance_count, 1);
    instance.detach();
    assert!(!instance.attached());
    assert_eq!(module.instance_count, 0);
}

#[test]
fn auto_detach_on_drop() {
    let mut store = PageStore::new(100);
    let mut module = Module::new(1).unwrap();
    {
        let memory = Memory::new(&mut store, 50, 10);
        let mut instance = Instance::new(memory);
        instance.attach(&mut module);
        assert_eq!(module.instance_count, 1);
    }
    assert_eq!(module.instance_count, 0);
}

#[test]
fn multiple_instances_same_module() {
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
    instance2.detach();
    assert_eq!(module.instance_count, 0);
}

#[test]
fn reattach_to_different_module() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut module1 = Module::new(1).unwrap();
    let mut module2 = Module::new(1).unwrap();
    let mut instance = Instance::new(memory);

    instance.attach(&mut module1);
    assert_eq!(module1.instance_count, 1);
    assert_eq!(module2.instance_count, 0);

    instance.attach(&mut module2);
    assert_eq!(module1.instance_count, 0);
    assert_eq!(module2.instance_count, 1);
}

#[test]
fn detach_unattached() {
    let mut store = PageStore::new(100);
    let memory = Memory::new(&mut store, 50, 10);
    let mut instance = Instance::new(memory);
    instance.detach(); // Should not panic
    assert!(!instance.attached());
}
