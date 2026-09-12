mod builder;
mod context;
mod error;
mod module;
mod r#type;

pub use builder::Builder;
pub use context::Context;
pub use error::Error;
pub use module::Module;
pub use r#type::Type;

#[cfg(test)]
mod tests {
    use llvm_sys::{core::LLVMGetTypeKind, LLVMTypeKind};

    use super::{Context, Type};

    #[test]
    fn creates_module() {
        let context = Context::create();
        let module = context.module("test").ok();

        assert!(
            module
                .as_ref()
                .is_some_and(|module| module.as_ir().contains("test"))
        );
    }

    #[test]
    fn creates_builder() {
        let context = Context::create();
        assert!(context.builder().is_ok());
    }

    #[test]
    fn creates_types() {
        let context = Context::create();
        let i32 = Type::i32(&context);
        let array = Type::array(&i32, 4);
        let pointer = Type::pointer(&context, 0);
        let function = Type::function(&i32, &[pointer], false).unwrap();

        assert_eq!(i32.kind(), LLVMTypeKind::LLVMIntegerTypeKind);
        assert_eq!(array.kind(), LLVMTypeKind::LLVMArrayTypeKind);
        assert_eq!(pointer.kind(), LLVMTypeKind::LLVMPointerTypeKind);
        assert_eq!(function.kind(), LLVMTypeKind::LLVMFunctionTypeKind);
        assert_eq!(array.as_ir(), "[4 x i32]");
    }

    #[test]
    fn rejects_mixed_context_types() {
        let first = Context::create();
        let second = Context::create();
        let value = Type::i32(&first);
        let field = Type::i32(&second);

        assert!(matches!(
            Type::structure(&first, &[value, field], false),
            Err(super::Error::DifferentContext)
        ));
    }
}
