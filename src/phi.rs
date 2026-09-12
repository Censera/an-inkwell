use llvm_sys::core::{LLVMAddIncoming, LLVMBuildPhi};
use llvm_sys::prelude::LLVMBasicBlockRef;

use crate::{Block, Builder, Error, Type, Value};

impl<'ctx> Builder<'ctx> {
    pub fn phi(&self, ty: &Type<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), ty.context()) {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildPhi(self.as_raw(), ty.as_raw(), c"".as_ptr()) };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn add_phi_incoming(
        &self,
        phi: &Value<'ctx>,
        incoming: &[(&Value<'ctx>, &Block<'ctx>)],
    ) -> Result<(), Error> {
        if !std::ptr::eq(self.context(), phi.context()) {
            return Err(Error::DifferentContext);
        }

        for (value, block) in incoming {
            if !std::ptr::eq(self.context(), value.context())
                || !std::ptr::eq(self.context(), block.context())
            {
                return Err(Error::DifferentContext);
            }
        }

        let mut values = incoming
            .iter()
            .map(|(value, _)| value.as_raw())
            .collect::<Vec<_>>();
        let mut blocks = incoming
            .iter()
            .map(|(_, block)| block.as_raw())
            .collect::<Vec<LLVMBasicBlockRef>>();

        unsafe {
            LLVMAddIncoming(
                phi.as_raw(),
                values.as_mut_ptr(),
                blocks.as_mut_ptr(),
                values.len() as u32,
            );
        }

        Ok(())
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
        crate::Block<'_>,
        crate::Block<'_>,
    ) {
        let module = context.module("test").unwrap();
        let builder = context.builder().unwrap();
        let function_type = Type::function(&Type::i32(context), &[], false).unwrap();
        let function = module.function("main", &function_type).unwrap();
        let entry = function.block("entry").unwrap();
        let left = function.block("left").unwrap();
        let right = function.block("right").unwrap();
        builder.position(&entry).unwrap();
        (module, builder, entry, left, right)
    }

    #[test]
    fn builds_phi_with_incoming_values() {
        let context = Context::create();
        let (module, builder, _entry, left, right) = function(&context);
        let integer = Type::i32(&context);
        let phi = builder.phi(&integer).unwrap();
        let left_value = Value::integer(&integer, 1, false);
        let right_value = Value::integer(&integer, 2, false);

        builder
            .add_phi_incoming(&phi, &[(&left_value, &left), (&right_value, &right)])
            .unwrap();

        let ir = module.as_ir();
        assert!(ir.contains("%"));
        assert!(phi.as_ir().contains("phi i32"));
    }

    #[test]
    fn rejects_phi_from_different_context() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _entry, left, _right) = function(&first);
        let integer = Type::i32(&second);
        let phi = builder.phi(&integer);

        assert!(matches!(phi, Err(crate::Error::DifferentContext)));

        let integer = Type::i32(&first);
        let phi = builder.phi(&integer).unwrap();
        let other_integer = Type::i32(&second);
        let value = Value::integer(&other_integer, 1, false);
        assert!(matches!(
            builder.add_phi_incoming(&phi, &[(&value, &left)]),
            Err(crate::Error::DifferentContext)
        ));
    }
}
