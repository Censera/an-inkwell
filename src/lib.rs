mod builder;
mod context;
mod error;
mod module;

pub use builder::Builder;
pub use context::Context;
pub use error::Error;
pub use module::Module;

#[cfg(test)]
mod tests {
    use super::Context;

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
}
