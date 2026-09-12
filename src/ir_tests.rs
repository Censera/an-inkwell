use super::{Context, Type, Value};

#[test]
fn builds_integer_arithmetic() {
    let context = Context::create();
    let module = context.module("test").unwrap();
    let builder = context.builder().unwrap();
    let integer = Type::i32(&context);
    let function_type = Type::function(&Type::void(&context), &[], false).unwrap();
    let function = module.function("main", &function_type).unwrap();
    let block = function.block("entry").unwrap();

    builder.position(&block).unwrap();

    let first = Value::integer(&integer, 20, true);
    let second = Value::integer(&integer, 4, true);

    assert!(builder.add(&first, &second).unwrap().as_ir().contains("i32 24"));
    assert!(builder.sub(&first, &second).unwrap().as_ir().contains("i32 16"));
    assert!(builder.mul(&first, &second).unwrap().as_ir().contains("i32 80"));
    assert!(builder.sdiv(&first, &second).unwrap().as_ir().contains("i32 5"));
    assert!(builder.udiv(&first, &second).unwrap().as_ir().contains("i32 5"));
    assert!(builder.srem(&first, &second).unwrap().as_ir().contains("i32 0"));
    assert!(builder.urem(&first, &second).unwrap().as_ir().contains("i32 0"));
    assert!(builder.neg(&first).unwrap().as_ir().contains("i32 -20"));
}

#[test]
fn builds_floating_arithmetic() {
    let context = Context::create();
    let module = context.module("test").unwrap();
    let builder = context.builder().unwrap();
    let float = Type::f64(&context);
    let function_type = Type::function(&Type::void(&context), &[], false).unwrap();
    let function = module.function("main", &function_type).unwrap();
    let block = function.block("entry").unwrap();

    builder.position(&block).unwrap();

    let first = Value::float(&float, 20.0);
    let second = Value::float(&float, 4.0);

    assert!(builder.fadd(&first, &second).unwrap().as_ir().contains("double 2.400000e+01"));
    assert!(builder.fsub(&first, &second).unwrap().as_ir().contains("double 1.600000e+01"));
    assert!(builder.fmul(&first, &second).unwrap().as_ir().contains("double 8.000000e+01"));
    assert!(builder.fdiv(&first, &second).unwrap().as_ir().contains("double 5.000000e+00"));
    assert!(builder.frem(&first, &second).unwrap().as_ir().contains("double 0.000000e+00"));
    assert!(builder.fneg(&first).unwrap().as_ir().contains("double -2.000000e+01"));
}

#[test]
fn rejects_arithmetic_from_different_contexts() {
    let first = Context::create();
    let second = Context::create();
    let builder = first.builder().unwrap();
    let first_type = Type::i32(&first);
    let second_type = Type::i32(&second);
    let function_type = Type::function(&Type::void(&first), &[], false).unwrap();
    let module = first.module("test").unwrap();
    let function = module.function("main", &function_type).unwrap();
    let block = function.block("entry").unwrap();
    builder.position(&block).unwrap();

    let first_value = Value::integer(&first_type, 1, true);
    let second_value = Value::integer(&second_type, 2, true);

    assert!(builder.add(&first_value, &second_value).is_err());
}
