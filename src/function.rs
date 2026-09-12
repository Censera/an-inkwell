use std::ptr::NonNull;

use llvm_sys::core::{LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMGetValueName2};
use llvm_sys::prelude::LLVMValueRef;

use crate::{Block, Context, Error, Module, Type};

pub struct Function<'ctx> {
    context: &'ctx Context,
    raw: NonNull<std::ffi::c_void>,
}

impl<'ctx> Function<'ctx> {
    pub(crate) fn from_raw(context: &'ctx Context, raw: LLVMValueRef) -> Result<Self, Error> {
        let Some(raw) = NonNull::new(raw.cast()) else {
            return Err(Error::FunctionCreation);
        };

        Ok(Self { context, raw })
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn name(&self) -> String {
        let mut length = 0;
        let value = unsafe { LLVMGetValueName2(self.as_raw(), &mut length) };
        if value.is_null() || length == 0 {
            return String::new();
        }

        let bytes = unsafe { std::slice::from_raw_parts(value.cast::<u8>(), length) };
        String::from_utf8_lossy(bytes).into_owned()
    }

    pub fn block(&self, name: &str) -> Result<Block<'ctx>, Error> {
        let name = std::ffi::CString::new(name)?;
        let raw = unsafe {
            LLVMAppendBasicBlockInContext(self.context.as_raw(), self.as_raw(), name.as_ptr())
        };
        Block::from_raw(self.context, raw)
    }

    pub(crate) fn from_module(
        module: &Module<'ctx>,
        name: &str,
        function_type: &Type<'ctx>,
    ) -> Result<Self, Error> {
        let context = module.context();
        if !std::ptr::eq(context, function_type.context()) {
            return Err(Error::DifferentContext);
        }

        let name = std::ffi::CString::new(name)?;
        let raw = unsafe { LLVMAddFunction(module.as_raw(), name.as_ptr(), function_type.as_raw()) };
        Self::from_raw(context, raw)
    }

    pub(crate) fn as_raw(&self) -> LLVMValueRef {
        self.raw.as_ptr().cast()
    }
}
