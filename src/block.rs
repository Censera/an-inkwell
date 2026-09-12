use std::ptr::NonNull;

use llvm_sys::core::LLVMGetBasicBlockName;
use llvm_sys::prelude::LLVMBasicBlockRef;

use crate::{Context, Error};

pub struct Block<'ctx> {
    context: &'ctx Context,
    raw: NonNull<std::ffi::c_void>,
}

impl<'ctx> Block<'ctx> {
    pub(crate) fn from_raw(context: &'ctx Context, raw: LLVMBasicBlockRef) -> Result<Self, Error> {
        let Some(raw) = NonNull::new(raw.cast()) else {
            return Err(Error::BlockCreation);
        };

        Ok(Self { context, raw })
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn name(&self) -> String {
        let value = unsafe { LLVMGetBasicBlockName(self.as_raw()) };
        if value.is_null() {
            return String::new();
        }

        unsafe { std::ffi::CStr::from_ptr(value) }
            .to_string_lossy()
            .into_owned()
    }

    pub(crate) fn as_raw(&self) -> LLVMBasicBlockRef {
        self.raw.as_ptr().cast()
    }
}
