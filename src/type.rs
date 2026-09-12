use std::ptr::NonNull;

use llvm_sys::{
    core::{
        LLVMArrayType2, LLVMDisposeMessage, LLVMDoubleTypeInContext, LLVMFloatTypeInContext,
        LLVMFunctionType, LLVMGetTypeKind, LLVMInt1TypeInContext, LLVMInt8TypeInContext,
        LLVMInt16TypeInContext, LLVMInt32TypeInContext, LLVMInt64TypeInContext,
        LLVMIntTypeInContext, LLVMPointerTypeInContext, LLVMPrintTypeToString,
        LLVMStructTypeInContext, LLVMVectorType, LLVMVoidTypeInContext,
    },
    prelude::LLVMTypeRef,
    LLVMTypeKind,
};

use crate::{Context, Error};

pub struct Type<'ctx> {
    context: &'ctx Context,
    raw: NonNull<std::ffi::c_void>,
}

impl<'ctx> Type<'ctx> {
    pub fn void(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMVoidTypeInContext(context.as_raw()) })
    }

    pub fn i1(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMInt1TypeInContext(context.as_raw()) })
    }

    pub fn i8(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMInt8TypeInContext(context.as_raw()) })
    }

    pub fn i16(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMInt16TypeInContext(context.as_raw()) })
    }

    pub fn i32(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMInt32TypeInContext(context.as_raw()) })
    }

    pub fn i64(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMInt64TypeInContext(context.as_raw()) })
    }

    pub fn integer(context: &'ctx Context, bits: u32) -> Self {
        Self::from_raw(
            context,
            unsafe { LLVMIntTypeInContext(context.as_raw(), bits) },
        )
    }

    pub fn f32(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMFloatTypeInContext(context.as_raw()) })
    }

    pub fn f64(context: &'ctx Context) -> Self {
        Self::from_raw(context, unsafe { LLVMDoubleTypeInContext(context.as_raw()) })
    }

    pub fn pointer(context: &'ctx Context, address_space: u32) -> Self {
        Self::from_raw(
            context,
            unsafe { LLVMPointerTypeInContext(context.as_raw(), address_space) },
        )
    }

    pub fn array(element: &Self, count: u64) -> Self {
        Self::from_raw(element.context, unsafe {
            LLVMArrayType2(element.as_raw(), count)
        })
    }

    pub fn vector(element: &Self, count: u32) -> Self {
        Self::from_raw(element.context, unsafe {
            LLVMVectorType(element.as_raw(), count)
        })
    }

    pub fn structure(
        context: &'ctx Context,
        fields: &[Type<'ctx>],
        packed: bool,
    ) -> Result<Self, Error> {
        for field in fields {
            if !std::ptr::eq(context, field.context) {
                return Err(Error::DifferentContext);
            }
        }

        let mut fields = fields.iter().map(Type::as_raw).collect::<Vec<_>>();
        let raw = unsafe {
            LLVMStructTypeInContext(
                context.as_raw(),
                fields.as_mut_ptr(),
                fields.len() as u32,
                packed as i32,
            )
        };

        Ok(Self::from_raw(context, raw))
    }

    pub fn function(
        return_type: &Type<'ctx>,
        params: &[Type<'ctx>],
        variadic: bool,
    ) -> Result<Self, Error> {
        for param in params {
            if !std::ptr::eq(return_type.context, param.context) {
                return Err(Error::DifferentContext);
            }
        }

        let mut params = params.iter().map(Type::as_raw).collect::<Vec<_>>();
        let raw = unsafe {
            LLVMFunctionType(
                return_type.as_raw(),
                params.as_mut_ptr(),
                params.len() as u32,
                variadic as i32,
            )
        };

        Ok(Self::from_raw(return_type.context, raw))
    }

    pub fn context(&self) -> &Context {
        self.context
    }

    pub fn kind(&self) -> LLVMTypeKind {
        unsafe { LLVMGetTypeKind(self.as_raw()) }
    }

    pub fn as_ir(&self) -> String {
        let value = unsafe { LLVMPrintTypeToString(self.as_raw()) };
        if value.is_null() {
            return String::new();
        }

        let text = unsafe { std::ffi::CStr::from_ptr(value) };
        let text = text.to_string_lossy().into_owned();
        unsafe { LLVMDisposeMessage(value) };
        text
    }

    fn from_raw(context: &'ctx Context, raw: LLVMTypeRef) -> Self {
        let raw = unsafe { NonNull::new_unchecked(raw.cast()) };
        Self { context, raw }
    }

    pub(crate) fn as_raw(&self) -> LLVMTypeRef {
        self.raw.as_ptr().cast()
    }
}
