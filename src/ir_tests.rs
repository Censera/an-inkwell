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

    assert!(
        builder
            .icmp(IntPredicate::Eq, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .icmp(IntPredicate::Ne, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .icmp(IntPredicate::Ugt, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .icmp(IntPredicate::Uge, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .icmp(IntPredicate::Ult, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .icmp(IntPredicate::Ule, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .icmp(IntPredicate::Sgt, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .icmp(IntPredicate::Sge, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .icmp(IntPredicate::Slt, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .icmp(IntPredicate::Sle, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
}

#[test]
fn builds_floating_comparisons() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let float = Type::f64(&context);
    let first = Value::float(&float, 20.0);
    let second = Value::float(&float, 4.0);

    assert!(
        builder
            .fcmp(FloatPredicate::False, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Oeq, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ogt, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Oge, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Olt, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ole, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::One, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ord, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Uno, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ueq, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ugt, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Uge, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ult, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Ule, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::Une, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .fcmp(FloatPredicate::True, &first, &second)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
}

#[test]
fn builds_logical_operations() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let boolean = Type::i1(&context);
    let true_value = Value::integer(&boolean, 1, false);
    let false_value = Value::integer(&boolean, 0, false);

    assert!(
        builder
            .logical_and(&true_value, &true_value)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .logical_and(&true_value, &false_value)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .logical_or(&true_value, &false_value)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
    assert!(
        builder
            .logical_or(&false_value, &false_value)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .logical_not(&true_value)
            .unwrap()
            .as_ir()
            .contains("i1 false")
    );
    assert!(
        builder
            .logical_not(&false_value)
            .unwrap()
            .as_ir()
            .contains("i1 true")
    );
}

#[test]
fn rejects_non_boolean_logical_operands() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let integer = Type::i32(&context);
    let first = Value::integer(&integer, 1, true);
    let second = Value::integer(&integer, 2, true);

    assert!(matches!(
        builder.logical_and(&first, &second),
        Err(super::Error::NotBoolean)
    ));
    assert!(matches!(
        builder.logical_or(&first, &second),
        Err(super::Error::NotBoolean)
    ));
    assert!(matches!(
        builder.logical_not(&first),
        Err(super::Error::NotBoolean)
    ));
}

#[test]
fn builds_bitwise_operations() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let integer = Type::i8(&context);
    let first = Value::integer(&integer, 0b1010, false);
    let second = Value::integer(&integer, 0b1100, false);

    assert!(
        builder
            .bit_and(&first, &second)
            .unwrap()
            .as_ir()
            .contains("i8 8")
    );
    assert!(
        builder
            .bit_or(&first, &second)
            .unwrap()
            .as_ir()
            .contains("i8 14")
    );
    assert!(
        builder
            .bit_xor(&first, &second)
            .unwrap()
            .as_ir()
            .contains("i8 6")
    );
    assert!(builder.bit_not(&first).unwrap().as_ir().contains("i8 -11"));
    assert!(
        builder
            .shl(&first, &Value::integer(&integer, 1, false))
            .unwrap()
            .as_ir()
            .contains("i8 20")
    );
    assert!(
        builder
            .lshr(&second, &Value::integer(&integer, 2, false))
            .unwrap()
            .as_ir()
            .contains("i8 3")
    );
    assert!(
        builder
            .ashr(
                &Value::integer(&integer, 0b1111_0000, false),
                &Value::integer(&integer, 2, false)
            )
            .unwrap()
            .as_ir()
            .contains("i8 -4")
    );
}

#[test]
fn builds_cast_operations() {
    let context = Context::create();
    let (_module, builder, _block) = function(&context);
    let i8 = Type::i8(&context);
    let i32 = Type::i32(&context);
    let f32 = Type::f32(&context);
    let f64 = Type::f64(&context);
    let pointer = Type::pointer(&context, 0);
    let integer = Value::integer(&i8, 5, false);
    let signed_integer = Value::integer(&i8, 250, true);
    let float = Value::float(&f32, 5.5);

    assert!(
        builder
            .trunc(&Value::integer(&i32, 5, false), &i8)
            .unwrap()
            .as_ir()
            .contains("i8 5")
    );
    assert!(
        builder
            .zext(&integer, &i32)
            .unwrap()
            .as_ir()
            .contains("i32 5")
    );
    assert!(
        builder
            .sext(&signed_integer, &i32)
            .unwrap()
            .as_ir()
            .contains("i32 -6")
    );
    assert!(
        builder
            .fptoui(&float, &i32)
            .unwrap()
            .as_ir()
            .contains("i32 5")
    );
    assert!(
        builder
            .fptosi(&float, &i32)
            .unwrap()
            .as_ir()
            .contains("i32 5")
    );
    assert!(
        builder
            .uitofp(&integer, &f32)
            .unwrap()
            .as_ir()
            .contains("float 5.000000e+00")
    );
    assert!(
        builder
            .sitofp(&signed_integer, &f32)
            .unwrap()
            .as_ir()
            .contains("float -6.000000e+00")
    );
    assert!(
        builder
            .fptrunc(&Value::float(&f64, 5.5), &f32)
            .unwrap()
            .as_ir()
            .contains("float 5.500000e+00")
    );
    assert!(
        builder
            .fpext(&float, &f64)
            .unwrap()
            .as_ir()
            .contains("double 5.500000e+00")
    );
    assert!(
        builder
            .ptrtoint(&Value::null(&pointer).unwrap(), &i32)
            .unwrap()
            .as_ir()
            .contains("i32 0")
    );
    assert!(
        builder
            .inttoptr(&Value::integer(&i32, 0, false), &pointer)
            .unwrap()
            .as_ir()
            .contains("ptr null")
    );
}

#[test]
fn rejects_casts_from_different_contexts() {
    let first = Context::create();
    let second = Context::create();
    let (_module, builder, _block) = function(&first);
    let first_type = Type::i32(&first);
    let second_type = Type::i64(&second);
    let value = Value::integer(&first_type, 1, false);

    assert!(matches!(
        builder.zext(&value, &second_type),
        Err(super::Error::DifferentContext)
    ));
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

    assert!(
        builder
            .icmp(IntPredicate::Eq, &first_value, &second_value)
            .is_err()
    );
}
