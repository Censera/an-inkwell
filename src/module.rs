use std::ptr::NonNull;

use llvm_sys::core::{LLVMDisposeMessage, LLVMDisposeModule, LLVMPrintModuleToString};
use llvm_sys::prelude::LLVMModuleRef;

use crate::{Context, Error, Function, Type};

pub struct Module<'ctx> {
    context: &'ctx Context,
    raw: NonNull<std::ffi::c_void>,
}

impl<'ctx> Module<'ctx> {
    pub(crate) fn from_raw(context: &'ctx Context, raw: LLVMModuleRef) -> Result<Self, Error> {
        let Some(raw) = NonNull::new(raw.cast()) else {
            return Err(Error::ModuleCreation);
        };

        Ok(Self { context, raw })
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn function(&self, name: &str, function_type: &Type<'ctx>) -> Result<Function<'ctx>, Error> {
        Function::from_module(self, name, function_type)
    }

    pub fn as_ir(&self) -> String {
        // LLVM returns an owned message that must be released with LLVMDisposeMessage.
        let value = unsafe { LLVMPrintModuleToString(self.as_raw()) };
        if value.is_null() {
            return String::new();
        }

        let text = unsafe { std::ffi::CStr::from_ptr(value) };
        let text = text.to_string_lossy().into_owned();
        unsafe { LLVMDisposeMessage(value) };
        text
    }

    pub(crate) fn as_raw(&self) -> LLVMModuleRef {
        self.raw.as_ptr().cast()
    }
}

impl Drop for Module<'_> {
    fn drop(&mut self) {
        // The module handle belongs to this wrapper.
        unsafe { LLVMDisposeModule(self.as_raw()) };
    }
}
