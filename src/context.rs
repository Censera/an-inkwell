use std::ffi::CString;
use std::ptr::NonNull;

use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext,
    LLVMModuleCreateWithNameInContext,
};
use llvm_sys::prelude::LLVMContextRef;

use crate::{Builder, Error, Module};

pub struct Context {
    raw: NonNull<std::ffi::c_void>,
}

impl Context {
    pub fn create() -> Self {
        // LLVMContextCreate returns a live context owned by this wrapper.
        let raw = unsafe { LLVMContextCreate() };
        let raw = unsafe { NonNull::new_unchecked(raw.cast()) };
        Self { raw }
    }

    pub fn module(&self, name: &str) -> Result<Module<'_>, Error> {
        let name = CString::new(name)?;
        // The context outlives the module and the C string remains valid for the call.
        let raw = unsafe { LLVMModuleCreateWithNameInContext(name.as_ptr(), self.as_raw()) };
        Module::from_raw(self, raw)
    }

    pub fn builder(&self) -> Result<Builder<'_>, Error> {
        // The context outlives the builder created from it.
        let raw = unsafe { LLVMCreateBuilderInContext(self.as_raw()) };
        Builder::from_raw(self, raw)
    }

    pub(crate) fn as_raw(&self) -> LLVMContextRef {
        self.raw.as_ptr().cast()
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        // The context handle belongs to this wrapper.
        unsafe { LLVMContextDispose(self.as_raw()) };
    }
}
