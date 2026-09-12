use llvm_sys::core::{LLVMBuildRet, LLVMBuildRetVoid};

use crate::{Builder, Error, Value};

impl<'ctx> Builder<'ctx> {
    pub fn ret(&self, value: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), value.context()) {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildRet(self.as_raw(), value.as_raw()) };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn ret_void(&self) -> Value<'ctx> {
        let raw = unsafe { LLVMBuildRetVoid(self.as_raw()) };
        Value::from_raw(self.context(), raw)
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use crate::{Context, Type, Value};

    fn function(context: &Context) -> (crate::Module<'_>, Builder<'_>, crate::Block<'_>) {
        let module = context.module("test").unwrap();
        let builder = context.builder().unwrap();
        let function_type = Type::function(&Type::i32(context), &[], false).unwrap();
        let function = module.function("main", &function_type).unwrap();
        let block = function.block("entry").unwrap();
        builder.position(&block).unwrap();
        (module, builder, block)
    }

    #[test]
    fn builds_return() {
        let context = Context::create();
        let (module, builder, _block) = function(&context);
        let integer = Type::i32(&context);
        let value = Value::integer(&integer, 42, false);

        builder.ret(&value).unwrap();

        assert!(module.as_ir().contains("ret i32 42"));
    }

    #[test]
    fn builds_void_return() {
        let context = Context::create();
        let (module, builder, _block) = function(&context);

        builder.ret_void();

        assert!(module.as_ir().contains("ret void"));
    }

    #[test]
    fn rejects_return_from_different_context() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _block) = function(&first);
        let integer = Type::i32(&second);
        let value = Value::integer(&integer, 42, false);

        assert!(matches!(
            builder.ret(&value),
            Err(crate::Error::DifferentContext)
        ));
    }
}
