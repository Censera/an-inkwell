use llvm_sys::core::LLVMBuildGEP2;
use llvm_sys::prelude::LLVMValueRef;

use crate::{Builder, Error, Type, Value};

impl<'ctx> Builder<'ctx> {
    pub fn gep(
        &self,
        element_type: &Type<'ctx>,
        pointer: &Value<'ctx>,
        indices: &[Value<'ctx>],
    ) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), element_type.context())
            || !std::ptr::eq(self.context(), pointer.context())
        {
            return Err(Error::DifferentContext);
        }

        for index in indices {
            if !std::ptr::eq(self.context(), index.context()) {
                return Err(Error::DifferentContext);
            }
        }

        let mut indices = indices
            .iter()
            .map(Value::as_raw)
            .collect::<Vec<LLVMValueRef>>();
        let raw = unsafe {
            LLVMBuildGEP2(
                self.as_raw(),
                element_type.as_raw(),
                pointer.as_raw(),
                indices.as_mut_ptr(),
                indices.len() as u32,
                c"".as_ptr(),
            )
        };
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
    fn builds_address_calculation() {
        let context = Context::create();
        let (_module, builder, _block) = function(&context);
        let integer = Type::i32(&context);
        let pointer = builder.alloca(&integer).unwrap();
        let index = Value::integer(&integer, 0, false);

        assert!(builder
            .gep(&integer, &pointer, &[index])
            .unwrap()
            .as_ir()
            .contains("getelementptr i32"));
    }

    #[test]
    fn rejects_address_calculation_from_different_contexts() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _block) = function(&first);
        let first_type = Type::i32(&first);
        let second_type = Type::i32(&second);
        let pointer = builder.alloca(&first_type).unwrap();
        let index = Value::integer(&second_type, 0, false);

        assert!(matches!(
            builder.gep(&first_type, &pointer, &[index]),
            Err(Error::DifferentContext)
        ));
        assert!(matches!(
            builder.gep(&second_type, &pointer, &[]),
            Err(Error::DifferentContext)
        ));
    }
}
