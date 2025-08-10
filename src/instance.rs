use crate::module::Module;
use std::ptr;

/// Runtime instance for executing compiled RISC-V code
#[derive(Default)]
pub struct Instance {
    /// Pointer to the compiled module (null if detached)
    module: *mut Module,
}

impl Instance {
    /// Create a new instance
    pub fn new() -> Self {
        Instance {
            module: ptr::null_mut(),
        }
    }

    /// Attach this instance to a module
    ///
    /// # Safety
    /// The module must outlive this instance unless detached
    pub fn attach(&mut self, module: &mut Module) {
        if !self.module.is_null() {
            self.detach();
        }
        self.module = module as *mut Module;
        unsafe {
            (*self.module).instance_count += 1;
        }
    }

    /// Detach this instance from its module
    pub fn detach(&mut self) {
        if !self.module.is_null() {
            unsafe {
                (*self.module).instance_count -= 1;
            }
            self.module = ptr::null_mut();
        }
    }

    /// Check if this instance is attached to a module
    pub fn attached(&self) -> bool {
        !self.module.is_null()
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        self.detach();
    }
}
