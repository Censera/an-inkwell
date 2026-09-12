use std::ptr::NonNull;

use llvm_sys::{
    core::{
        LLVMConstArray2, LLVMConstInt, LLVMConstNull, LLVMConstReal, LLVMConstStruct,
        LLVMConstVector, LLVMDisposeMessage, LLVMPrintValueToString,
    },
    prelude::LLVMValueRef,
};

use crate::{Context, Error, Type};

pub struct Value<'ctx> {
    context: &'ctx Context,
    raw: NonNull<std::ffi::c_void>,
}

impl<'ctx> Value<'ctx> {
    pub fn integer(integer: &'ctx Type<'ctx>, value: u64, signed: bool) -> Self {
        Self::from_raw(integer.context(), unsafe {
            LLVMConstInt(integer.as_raw(), value, signed as i32)
        })
    }

    pub fn float(float: &'ctx Type<'ctx>, value: f64) -> Self {
        Self::from_raw(float.context(), unsafe {
            LLVMConstReal(float.as_raw(), value)
        })
    }

    pub fn null(pointer: &'ctx Type<'ctx>) -> Result<Self, Error> {
        if pointer.kind() != llvm_sys::LLVMTypeKind::LLVMPointerTypeKind {
            return Err(Error::NotPointerType);
        }

        Ok(Self::from_raw(pointer.context(), unsafe {
            LLVMConstNull(pointer.as_raw())
        }))
    }

    pub fn array(element: &'ctx Type<'ctx>, values: &[Value<'ctx>]) -> Result<Self, Error> {
        for value in values {
            if !std::ptr::eq(element.context(), value.context) {
                return Err(Error::DifferentContext);
            }
        }

        let mut values = values.iter().map(Self::as_raw).collect::<Vec<_>>();
        let raw =
            unsafe { LLVMConstArray2(element.as_raw(), values.as_mut_ptr(), values.len() as u64) };

        Ok(Self::from_raw(element.context(), raw))
    }

    pub fn vector(values: &[Value<'ctx>]) -> Result<Self, Error> {
        let Some(first) = values.first() else {
            return Err(Error::EmptyAggregate);
        };

        for value in values {
            if !std::ptr::eq(first.context, value.context) {
                return Err(Error::DifferentContext);
            }
        }

        let mut values = values.iter().map(Self::as_raw).collect::<Vec<_>>();
        let raw = unsafe { LLVMConstVector(values.as_mut_ptr(), values.len() as u32) };
        Ok(Self::from_raw(first.context, raw))
    }

    pub fn structure(values: &[Value<'ctx>], packed: bool) -> Result<Self, Error> {
        let Some(first) = values.first() else {
            return Err(Error::EmptyAggregate);
        };

        for value in values {
            if !std::ptr::eq(first.context, value.context) {
                return Err(Error::DifferentContext);
            }
        }

        let mut values = values.iter().map(Self::as_raw).collect::<Vec<_>>();
        let raw =
            unsafe { LLVMConstStruct(values.as_mut_ptr(), values.len() as u32, packed as i32) };
        Ok(Self::from_raw(first.context, raw))
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn as_ir(&self) -> String {
        let value = unsafe { LLVMPrintValueToString(self.as_raw()) };
        if value.is_null() {
            return String::new();
        }

        let text = unsafe { std::ffi::CStr::from_ptr(value) };
        let text = text.to_string_lossy().into_owned();
        unsafe { LLVMDisposeMessage(value) };
        text
    }

    pub(crate) fn from_raw(context: &'ctx Context, raw: LLVMValueRef) -> Self {
        let raw = unsafe { NonNull::new_unchecked(raw.cast()) };
        Self { context, raw }
    }

    pub(crate) fn as_raw(&self) -> LLVMValueRef {
        self.raw.as_ptr().cast()
    }
}
