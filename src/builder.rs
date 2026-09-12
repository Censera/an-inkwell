use std::ptr::NonNull;

use llvm_sys::core::LLVMDisposeBuilder;
use llvm_sys::prelude::LLVMBuilderRef;

use crate::{Context, Error};

pub struct Builder<'ctx> {
    context: &'ctx Context,
    raw: NonNull<std::ffi::c_void>,
}

impl<'ctx> Builder<'ctx> {
    pub(crate) fn from_raw(context: &'ctx Context, raw: LLVMBuilderRef) -> Result<Self, Error> {
        let Some(raw) = NonNull::new(raw.cast()) else {
            return Err(Error::BuilderCreation);
        };

        Ok(Self { context, raw })
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub(crate) fn as_raw(&self) -> LLVMBuilderRef {
        self.raw.as_ptr().cast()
    }
}

impl Drop for Builder<'_> {
    fn drop(&mut self) {
        // The handle is owned by this wrapper and is released exactly once.
        unsafe { LLVMDisposeBuilder(self.as_raw()) };
    }
}
