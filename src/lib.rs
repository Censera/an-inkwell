mod address;
mod allocation;
mod block;
mod builder;
mod context;
mod error;
mod function;
mod load_store;
#[cfg(test)]
mod ir_tests;
mod module;
mod r#type;
mod value;

pub use block::Block;
pub use builder::{Builder, FloatPredicate, IntPredicate};
pub use context::Context;
pub use error::Error;
pub use function::Function;
pub use module::Module;
pub use r#type::Type;
pub use value::Value;

#[cfg(test)]
mod tests {
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
        let builder = context.builder();
        assert!(builder.is_ok());
    }

    #[test]
    fn creates_types_from_one_context() {
        let context = Context::create();
        let integer = Type::i32(&context);
        let structure = Type::structure(&context, &[integer], false);
        assert!(structure.is_ok());
    }
}

#[cfg(test)]
mod type_tests;

#[cfg(test)]
mod values_tests;
