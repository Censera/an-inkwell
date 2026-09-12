use std::ptr::NonNull;

use llvm_sys::core::{
    LLVMBuildAdd, LLVMBuildAnd, LLVMBuildFAdd, LLVMBuildFCmp, LLVMBuildFDiv, LLVMBuildFMul,
    LLVMBuildFNeg, LLVMBuildFRem, LLVMBuildFSub, LLVMBuildICmp, LLVMBuildMul, LLVMBuildNeg,
    LLVMBuildNot, LLVMBuildOr, LLVMBuildSDiv, LLVMBuildSRem, LLVMBuildSub, LLVMBuildUDiv,
    LLVMBuildURem, LLVMDisposeBuilder, LLVMGetIntTypeWidth, LLVMGetTypeKind,
    LLVMPositionBuilderAtEnd, LLVMTypeOf,
};
use llvm_sys::prelude::LLVMBuilderRef;
use llvm_sys::{LLVMIntPredicate, LLVMRealPredicate, LLVMTypeKind};

use crate::{Block, Context, Error, Value};

#[derive(Clone, Copy, Debug)]
pub enum IntPredicate {
    Eq,
    Ne,
    Ugt,
    Uge,
    Ult,
    Ule,
    Sgt,
    Sge,
    Slt,
    Sle,
}

impl IntPredicate {
    fn raw(self) -> LLVMIntPredicate {
        match self {
            Self::Eq => LLVMIntPredicate::LLVMIntEQ,
            Self::Ne => LLVMIntPredicate::LLVMIntNE,
            Self::Ugt => LLVMIntPredicate::LLVMIntUGT,
            Self::Uge => LLVMIntPredicate::LLVMIntUGE,
            Self::Ult => LLVMIntPredicate::LLVMIntULT,
            Self::Ule => LLVMIntPredicate::LLVMIntULE,
            Self::Sgt => LLVMIntPredicate::LLVMIntSGT,
            Self::Sge => LLVMIntPredicate::LLVMIntSGE,
            Self::Slt => LLVMIntPredicate::LLVMIntSLT,
            Self::Sle => LLVMIntPredicate::LLVMIntSLE,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum FloatPredicate {
    False,
    Oeq,
    Ogt,
    Oge,
    Olt,
    Ole,
    One,
    Ord,
    Uno,
    Ueq,
    Ugt,
    Uge,
    Ult,
    Ule,
    Une,
    True,
}

impl FloatPredicate {
    fn raw(self) -> LLVMRealPredicate {
        match self {
            Self::False => LLVMRealPredicate::LLVMRealPredicateFalse,
            Self::Oeq => LLVMRealPredicate::LLVMRealOEQ,
            Self::Ogt => LLVMRealPredicate::LLVMRealOGT,
            Self::Oge => LLVMRealPredicate::LLVMRealOGE,
            Self::Olt => LLVMRealPredicate::LLVMRealOLT,
            Self::Ole => LLVMRealPredicate::LLVMRealOLE,
            Self::One => LLVMRealPredicate::LLVMRealONE,
            Self::Ord => LLVMRealPredicate::LLVMRealORD,
            Self::Uno => LLVMRealPredicate::LLVMRealUNO,
            Self::Ueq => LLVMRealPredicate::LLVMRealUEQ,
            Self::Ugt => LLVMRealPredicate::LLVMRealUGT,
            Self::Uge => LLVMRealPredicate::LLVMRealUGE,
            Self::Ult => LLVMRealPredicate::LLVMRealULT,
            Self::Ule => LLVMRealPredicate::LLVMRealULE,
            Self::Une => LLVMRealPredicate::LLVMRealUNE,
            Self::True => LLVMRealPredicate::LLVMRealPredicateTrue,
        }
    }
}

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

    pub fn icmp(
        &self,
        predicate: IntPredicate,
        left: &Value<'ctx>,
        right: &Value<'ctx>,
    ) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe {
            LLVMBuildICmp(
                self.as_raw(),
                predicate.raw(),
                left.as_raw(),
                right.as_raw(),
                c"".as_ptr(),
            )
        };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn fcmp(
        &self,
        predicate: FloatPredicate,
        left: &Value<'ctx>,
        right: &Value<'ctx>,
    ) -> Result<Value<'ctx>, Error> {
        self.check(left, right)?;
        let raw = unsafe {
            LLVMBuildFCmp(
                self.as_raw(),
                predicate.raw(),
                left.as_raw(),
                right.as_raw(),
                c"".as_ptr(),
            )
        };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn logical_and(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check_boolean(left, right)?;
        let raw = unsafe { LLVMBuildAnd(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn logical_or(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        self.check_boolean(left, right)?;
        let raw = unsafe { LLVMBuildOr(self.as_raw(), left.as_raw(), right.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    pub fn logical_not(&self, value: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context, value.context()) {
            return Err(Error::DifferentContext);
        }
        if !is_boolean(value) {
            return Err(Error::NotBoolean);
        }

        let raw = unsafe { LLVMBuildNot(self.as_raw(), value.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context, raw))
    }

    fn check(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<(), Error> {
        if !std::ptr::eq(self.context, left.context()) || !std::ptr::eq(self.context, right.context()) {
            return Err(Error::DifferentContext);
        }

        Ok(())
    }

    fn check_boolean(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<(), Error> {
        self.check(left, right)?;
        if !is_boolean(left) || !is_boolean(right) {
            return Err(Error::NotBoolean);
        }

        Ok(())
    }

    pub(crate) fn as_raw(&self) -> LLVMBuilderRef {
        self.raw.as_ptr().cast()
    }
}

fn is_boolean(value: &Value<'_>) -> bool {
    let ty = unsafe { LLVMTypeOf(value.as_raw()) };
    unsafe {
        LLVMGetTypeKind(ty) == LLVMTypeKind::LLVMIntegerTypeKind && LLVMGetIntTypeWidth(ty) == 1
    }
}

impl Drop for Builder<'_> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeBuilder(self.as_raw()) };
    }
}
