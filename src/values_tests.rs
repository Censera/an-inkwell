use super::{Context, Type, Value};

#[test]
fn creates_scalar_values() {
    let context = Context::create();
    let integer = Type::i32(&context);
    let float = Type::f64(&context);
    let pointer = Type::pointer(&context, 0);

    assert!(Value::integer(&integer, 42, true).as_ir().contains("i32 42"));
    assert!(Value::float(&float, 3.5).as_ir().contains("double 3.500000e+00"));
    assert!(Value::null(&pointer).unwrap().as_ir().contains("null"));
}

#[test]
fn creates_aggregate_values() {
    let context = Context::create();
    let integer = Type::i32(&context);
    let first = Value::integer(&integer, 1, true);
    let second = Value::integer(&integer, 2, true);

    let array = Value::array(&integer, &[first, second]).unwrap();
    assert!(array.as_ir().contains("[i32 1, i32 2]"));

    let first = Value::integer(&integer, 1, true);
    let second = Value::integer(&integer, 2, true);
    let vector = Value::vector(&[first, second]).unwrap();
    assert!(vector.as_ir().contains("<i32 1, i32 2>"));

    let first = Value::integer(&integer, 1, true);
    let second = Value::integer(&integer, 2, true);
    let structure = Value::structure(&[first, second], false).unwrap();
    assert!(structure.as_ir().contains("{ i32 1, i32 2 }"));
}

#[test]
fn rejects_invalid_value_contexts() {
    let first = Context::create();
    let second = Context::create();
    let first_type = Type::i32(&first);
    let second_type = Type::i32(&second);
    let first_value = Value::integer(&first_type, 1, true);
    let second_value = Value::integer(&second_type, 2, true);

    assert!(Value::array(&first_type, &[first_value]).is_ok());
    assert!(Value::array(&first_type, &[second_value]).is_err());
}

#[test]
fn rejects_non_pointer_null() {
    let context = Context::create();
    assert!(Value::null(&Type::i32(&context)).is_err());
}

#[test]
fn creates_function_and_block_values() {
    let context = Context::create();
    let module = context.module("test").unwrap();
    let function_type = Type::function(&Type::void(&context), &[], false).unwrap();
    let function = module.function("main", &function_type).unwrap();
    let block = function.block("entry").unwrap();

    assert_eq!(function.name(), "main");
    assert_eq!(block.name(), "entry");
}
