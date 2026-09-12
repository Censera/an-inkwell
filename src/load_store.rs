use llvm_sys::core::{LLVMBuildLoad2, LLVMBuildStore};

use crate::{Builder, Error, Type, Value};

impl<'ctx> Builder<'ctx> {
    pub fn load(&self, ty: &Type<'ctx>, pointer: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), ty.context())
            || !std::ptr::eq(self.context(), pointer.context())
        {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe {
            LLVMBuildLoad2(self.as_raw(), ty.as_raw(), pointer.as_raw(), c"".as_ptr())
        };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn store(&self, value: &Value<'ctx>, pointer: &Value<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), value.context())
            || !std::ptr::eq(self.context(), pointer.context())
        {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildStore(self.as_raw(), value.as_raw(), pointer.as_raw()) };
        Ok(Value::from_raw(self.context(), raw))
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use crate::{Context, Error, Type, Value};

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
    fn builds_load_and_store() {
        let context = Context::create();
        let (_module, builder, _block) = function(&context);
        let integer = Type::i32(&context);
        let pointer = builder.alloca(&integer).unwrap();
        let value = Value::integer(&integer, 42, false);

        assert!(builder.store(&value, &pointer).is_ok());
        assert!(builder.load(&integer, &pointer).unwrap().as_ir().contains("load i32"));
    }

    #[test]
    fn rejects_load_and_store_from_different_contexts() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _block) = function(&first);
        let first_type = Type::i32(&first);
        let second_type = Type::i32(&second);
        let pointer = builder.alloca(&first_type).unwrap();
        let value = Value::integer(&first_type, 1, false);

        assert!(matches!(
            builder.load(&second_type, &pointer),
            Err(Error::DifferentContext)
        ));
        assert!(matches!(
            builder.store(&Value::integer(&second_type, 1, false), &pointer),
            Err(Error::DifferentContext)
        ));
        assert!(matches!(
            builder.store(&value, &Value::integer(&second_type, 0, false)),
            Err(Error::DifferentContext)
        ));
    }
}
