use super::{Context, FloatPredicate, IntPredicate, Type, Value};

fn function(context: &Context) -> (super::Module<'_>, super::Builder<'_>, super::Block<'_>) {
    let module = context.module("test").unwrap();
    let builder = context.builder().unwrap();
    let function_type = Type::function(&Type::void(context), &[], false).unwrap();
    let function = module.function("main", &function_type).unwrap();
    let block = function.block("entry").unwrap();
    builder.position(&block).unwrap();
    (module, builder, block)
}

#[test]
fn builds_integer_comparisons() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let integer = Type::i32(&context);
    let first = Value::integer(&integer, 20, true);
    let second = Value::integer(&integer, 4, true);

    assert!(builder.icmp(IntPredicate::Eq, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.icmp(IntPredicate::Ne, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.icmp(IntPredicate::Ugt, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.icmp(IntPredicate::Uge, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.icmp(IntPredicate::Ult, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.icmp(IntPredicate::Ule, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.icmp(IntPredicate::Sgt, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.icmp(IntPredicate::Sge, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.icmp(IntPredicate::Slt, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.icmp(IntPredicate::Sle, &first, &second).unwrap().as_ir().contains("i1 false"));
}

#[test]
fn builds_floating_comparisons() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let float = Type::f64(&context);
    let first = Value::float(&float, 20.0);
    let second = Value::float(&float, 4.0);

    assert!(builder.fcmp(FloatPredicate::False, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Oeq, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Ogt, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::Oge, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::Olt, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Ole, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::One, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::Ord, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::Uno, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Ueq, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Ugt, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::Uge, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::Ult, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Ule, &first, &second).unwrap().as_ir().contains("i1 false"));
    assert!(builder.fcmp(FloatPredicate::Une, &first, &second).unwrap().as_ir().contains("i1 true"));
    assert!(builder.fcmp(FloatPredicate::True, &first, &second).unwrap().as_ir().contains("i1 true"));
}

#[test]
fn rejects_comparisons_from_different_contexts() {
    let first = Context::create();
    let second = Context::create();
    let (_module, builder, _block) = function(&first);
    let first_type = Type::i32(&first);
    let second_type = Type::i32(&second);
    let first_value = Value::integer(&first_type, 1, true);
    let second_value = Value::integer(&second_type, 2, true);

    assert!(builder.icmp(IntPredicate::Eq, &first_value, &second_value).is_err());
}
