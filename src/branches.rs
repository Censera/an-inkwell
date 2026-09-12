use llvm_sys::core::{LLVMBuildBr, LLVMBuildCondBr};

use crate::{Block, Builder, Error, Value};

impl<'ctx> Builder<'ctx> {
    pub fn br(&self, destination: &Block<'ctx>) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), destination.context()) {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe { LLVMBuildBr(self.as_raw(), destination.as_raw()) };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn cond_br(
        &self,
        condition: &Value<'ctx>,
        then_block: &Block<'ctx>,
        else_block: &Block<'ctx>,
    ) -> Result<Value<'ctx>, Error> {
        if !std::ptr::eq(self.context(), condition.context())
            || !std::ptr::eq(self.context(), then_block.context())
            || !std::ptr::eq(self.context(), else_block.context())
        {
            return Err(Error::DifferentContext);
        }

        let raw = unsafe {
            LLVMBuildCondBr(
                self.as_raw(),
                condition.as_raw(),
                then_block.as_raw(),
                else_block.as_raw(),
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
        crate::Block<'_>,
        crate::Block<'_>,
    ) {
        let module = context.module("test").unwrap();
        let builder = context.builder().unwrap();
        let void = Type::void(context);
        let function_type = Type::function(&void, &[], false).unwrap();
        let function = module.function("main", &function_type).unwrap();
        let entry = function.block("entry").unwrap();
        let then_block = function.block("then").unwrap();
        let else_block = function.block("else").unwrap();
        builder.position(&entry).unwrap();
        (module, builder, entry, then_block, else_block)
    }

    #[test]
    fn builds_branch() {
        let context = Context::create();
        let (_module, builder, _entry, then_block, _else_block) = function(&context);

        let branch = builder.br(&then_block).unwrap();
        assert!(branch.as_ir().contains("br label %then"));
    }

    #[test]
    fn builds_conditional_branch() {
        let context = Context::create();
        let (_module, builder, _entry, then_block, else_block) = function(&context);
        let boolean = Type::i1(&context);
        let condition = Value::integer(&boolean, 1, false);

        let branch = builder.cond_br(&condition, &then_block, &else_block).unwrap();
        assert!(branch
            .as_ir()
            .contains("br i1 true, label %then, label %else"));
    }

    #[test]
    fn rejects_branch_from_different_context() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _entry, then_block, _else_block) = function(&first);
        let module = second.module("test").unwrap();
        let void = Type::void(&second);
        let function_type = Type::function(&void, &[], false).unwrap();
        let function = module.function("main", &function_type).unwrap();
        let other_block = function.block("other").unwrap();

        assert!(matches!(
            builder.br(&other_block),
            Err(crate::Error::DifferentContext)
        ));
        assert!(builder.br(&then_block).is_ok());
    }
}
