use llvm_sys::core::LLVMBuildCall2;

use crate::{Builder, Error, Function, Type, Value};

impl<'ctx> Builder<'ctx> {
    pub fn call(
        &self,
        function_type: &Type<'ctx>,
        function: &Function<'ctx>,
        arguments: &[Value<'ctx>],
    ) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), function_type.context())
            || !std::ptr::eq(self.context(), function.context())
        {
            return Err(Error::DifferentContext);
        }

        for argument in arguments {
            if !std::ptr::eq(self.context(), argument.context()) {
                return Err(Error::DifferentContext);
            }
        }

        let mut arguments = arguments.iter().map(Value::as_raw).collect::<Vec<_>>();
        let raw = unsafe {
            LLVMBuildCall2(
                self.as_raw(),
                function_type.as_raw(),
                function.as_raw(),
                arguments.as_mut_ptr(),
                arguments.len() as u32,
                c"".as_ptr(),
            )
        };

        Ok(Value::from_raw(self.context(), raw))
    }
}

#[cfg(test)]
mod tests {
    use super::Builder;
    use crate::{Context, Type, Value};

    fn function(
        context: &Context,
    ) -> (
        crate::Module<'_>,
        Builder<'_>,
        crate::Block<'_>,
        crate::Function<'_>,
        Type<'_>,
    ) {
        let module = context.module("test").unwrap();
        let builder = context.builder().unwrap();
        let integer = Type::i32(context);
        let parameter = Type::i32(context);
        let function_type = Type::function(&integer, &[parameter], false).unwrap();
        let function = module.function("add", &function_type).unwrap();
        let block = function.block("entry").unwrap();
        builder.position(&block).unwrap();
        (module, builder, block, function, function_type)
    }

    #[test]
    fn builds_call() {
        let context = Context::create();
        let (module, builder, _block, function, function_type) = function(&context);
        let integer = Type::i32(&context);
        let argument = Value::integer(&integer, 42, false);

        let call = builder
            .call(&function_type, &function, &[argument])
            .unwrap();

        assert!(call.as_ir().contains("call i32 @add"));
        assert!(module.as_ir().contains("call i32 @add"));
    }

    #[test]
    fn rejects_call_from_different_context() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _block, function, function_type) = function(&first);
        let integer = Type::i32(&second);
        let argument = Value::integer(&integer, 42, false);

        assert!(matches!(
            builder.call(&function_type, &function, &[argument]),
            Err(crate::Error::DifferentContext)
        ));
    }
}
