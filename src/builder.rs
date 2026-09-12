use std::ptr::NonNull;

use llvm_sys::core::{
    LLVMBuildAdd, LLVMBuildFAdd, LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFNeg, LLVMBuildFRem,
    LLVMBuildFSub, LLVMBuildMul, LLVMBuildNeg, LLVMBuildSDiv, LLVMBuildSRem, LLVMBuildSub,
    LLVMBuildUDiv, LLVMBuildURem, LLVMDisposeBuilder, LLVMPositionBuilderAtEnd,
};
use llvm_sys::prelude::LLVMBuilderRef;

use crate::{Block, Context, Error, Value};

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

    pub fn position(&self, block: &Block<'ctx>) -> Result<(), Error> {
        if !std::ptr::eq(self.context, block.context()) {
            return Err(Error::DifferentContext);
        }

        unsafe { LLVMPositionBuilderAtEnd(self.as_raw(), block.as_raw()) };
        Ok(())
    }

    pub fn add(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildAdd(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn sub(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildSub(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn mul(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildMul(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn sdiv(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildSDiv(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn udiv(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildUDiv(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn srem(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildSRem(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn urem(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildURem(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn neg(&self, value: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context, value.context()) {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildNeg(self.as_raw(), value.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn fadd(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildFAdd(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn fsub(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildFSub(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn fmul(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildFMul(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn fdiv(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildFDiv(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn frem(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe { LLVMBuildFRem(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn fneg(&self, value: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context, value.context()) {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildFNeg(self.as_raw(), value.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    fn check(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<(), Error> {
        if !std::ptr::eq(self.context, left.context()) || !std::ptr::eq(self.context, right.context()) {
            return Err(Error::DifferentContext);
        }

        Ok(())
    }

    pub(crate) fn as_raw(&self) -> LLVMBuilderRef {
        self.raw.as_ptr().cast()
    }
}

impl Drop for Builder<'_> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.as_raw()) };
    }
}
