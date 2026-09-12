use llvm_sys::core::{
    LLVMBuildExtractElement, LLVMBuildExtractValue, LLVMBuildInsertElement, LLVMBuildInsertValue,
};

use crate::{Builder, Error, Value};

impl<'ctx> Builder<'ctx> {
    pub fn extract_value(&self, aggregate: &Value<'ctx>, index: u32) -> Result<Value<'ctx>, Error> {
        self.check_context(aggregate)?;
        let raw = unsafe { LLVMBuildExtractValue(self.as_raw(), aggregate.as_raw(), index, c"".as_ptr()) };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn insert_value(
        &self,
        aggregate: &Value<'ctx>,
        value: &Value<'ctx>,
        index: u32,
    ) -> Result<Value<'ctx>, Error> {
        self.check(aggregate, value)?;
        let raw = unsafe {
            LLVMBuildInsertValue(
                self.as_raw(),
                aggregate.as_raw(),
                value.as_raw(),
                index,
                c"".as_ptr(),
            )
        };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn extract_element(
        &self,
        vector: &Value<'ctx>,
        index: &Value<'ctx>,
    ) -> Result<Value<'ctx>, Error> {
        self.check(vector, index)?;
        let raw = unsafe {
            LLVMBuildExtractElement(self.as_raw(), vector.as_raw(), index.as_raw(), c"".as_ptr())
        };
        Ok(Value::from_raw(self.context(), raw))
    }

    pub fn insert_element(
        &self,
        vector: &Value<'ctx>,
        element: &Value<'ctx>,
        index: &Value<'ctx>,
    ) -> Result<Value<'ctx>, Error> {
        self.check(vector, element)?;
        self.check(vector, index)?;
        let raw = unsafe {
            LLVMBuildInsertElement(
                self.as_raw(),
                vector.as_raw(),
                element.as_raw(),
                index.as_raw(),
                c"".as_ptr(),
            )
        };
        Ok(Value::from_raw(self.context(), raw))
    }

    fn check_context(&self, value: &Value<'ctx>) -> Result<(), Error> {
        if !std::ptr::eq(self.context(), value.context()) {
            return Err(Error::DifferentContext);
        }
        Ok(())
    }

    fn check(&self, left: &Value<'ctx>, right: &Value<'ctx>) -> Result<(), Error> {
        self.check_context(left)?;
        self.check_context(right)?;
        Ok(())
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
    fn builds_aggregate_access() {
        let context = Context::create();
        let (_module, builder, _block) = function(&context);
        let integer = Type::i32(&context);
        let structure = Type::structure(&context, &[integer.clone(), integer.clone()], false).unwrap();
        let value = Value::structure(&structure, &[Value::integer(&integer, 10, false), Value::integer(&integer, 20, false)]).unwrap();

        assert!(builder.extract_value(&value, 1).unwrap().as_ir().contains("i32 20"));
        assert!(builder
            .insert_value(&value, &Value::integer(&integer, 30, false), 1)
            .unwrap()
            .as_ir()
            .contains("i32 30"));
    }

    #[test]
    fn builds_vector_element_access() {
        let context = Context::create();
        let (_module, builder, _block) = function(&context);
        let integer = Type::i32(&context);
        let vector = Type::vector(&integer, 2);
        let value = Value::vector(&vector, &[Value::integer(&integer, 10, false), Value::integer(&integer, 20, false)]).unwrap();
        let index = Value::integer(&integer, 1, false);

        assert!(builder
            .extract_element(&value, &index)
            .unwrap()
            .as_ir()
            .contains("i32 20"));
        assert!(builder
            .insert_element(&value, &Value::integer(&integer, 30, false), &index)
            .unwrap()
            .as_ir()
            .contains("i32 30"));
    }

    #[test]
    fn rejects_aggregate_access_from_different_contexts() {
        let first = Context::create();
        let second = Context::create();
        let (_module, builder, _block) = function(&first);
        let first_type = Type::i32(&first);
        let second_type = Type::i32(&second);
        let structure = Type::structure(&first, &[first_type.clone()], false).unwrap();
        let value = Value::structure(&structure, &[Value::integer(&first_type, 1, false)]).unwrap();
        let other = Value::integer(&second_type, 2, false);

        assert!(matches!(
            builder.extract_value(&other, 0),
            Err(Error::DifferentContext)
        ));
        assert!(matches!(
            builder.insert_value(&value, &other, 0),
            Err(Error::DifferentContext)
        ));
    }
}
