use llvm_sys::core::LLVMBuildAlloca;

use crate::{Builder, Error, Type, Value};

impl<'ctx> Builder<'ctx> {
    pub fn alloca(&self, ty: &Type<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), ty.context()) {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildAlloca(self.as_raw(), ty.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(ty.context(), raw))
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use crate::{Context, Error, Type};

    fn function(context: &Context) -> (crate::Module<'_>, Builder<'_>, crate::Block<'_>) {
        let module = context.module("test").unwrap();
        let builder = context.builder().unwrap();
        let function_type = Type::function(&Type::void(context), &[], false).unwrap();
        let function = module.function("main", &function_type).unwrap();
        let block = function.block("entry").unwrap();
        builder.position(&block).unwrap();
        (module, builder, block)
    }

    #[test]
    fn builds_allocation() {
        let context = Context::create();
        let (_module, builder, _block) = function(&context);
        let integer = Type::i32(&context);

        assert!(
            builder
                .alloca(&integer)
                .unwrap()
                .as_ir()
                .contains("alloca i32")
        );
    }

    #[test]
    fn rejects_allocation_from_different_context() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _block) = function(&first);
        let integer = Type::i32(&second);

        assert!(matches!(
            builder.alloca(&integer),
            Err(Error::DifferentContext)
        ));
    }
}
