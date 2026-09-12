use llvm_sys::LLVMTypeKind;

use super::{Context, Type};

#[test]
fn creates_scalar_types() {
    let context = Context::create();

    assert_eq!(Type::void(&context).kind(), LLVMTypeKind::LLVMVoidTypeKind);
    assert_eq!(
        Type::i32(&context).kind(),
        LLVMTypeKind::LLVMIntegerTypeKind
    );
    assert_eq!(Type::f64(&context).kind(), LLVMTypeKind::LLVMDoubleTypeKind);
    assert_eq!(
        Type::pointer(&context, 0).kind(),
        LLVMTypeKind::LLVMPointerTypeKind
    );
}

#[test]
fn creates_composite_types() {
    let context = Context::create();
    let i32_type = Type::i32(&context);
    let array = Type::array(&i32_type, 4);
    let vector = Type::vector(&i32_type, 4);
    let structure = Type::structure(&context, &[i32_type], false).unwrap();
    let function = Type::function(&Type::void(&context), &[Type::i32(&context)], false).unwrap();

    assert_eq!(array.kind(), LLVMTypeKind::LLVMArrayTypeKind);
    assert_eq!(vector.kind(), LLVMTypeKind::LLVMVectorTypeKind);
    assert_eq!(structure.kind(), LLVMTypeKind::LLVMStructTypeKind);
    assert_eq!(function.kind(), LLVMTypeKind::LLVMFunctionTypeKind);
}

#[test]
fn rejects_mixed_context_struct_fields() {
    let first = Context::create();
    let second = Context::create();
    let field = Type::i32(&first);

    assert!(Type::structure(&second, &[field], false).is_err());
}

#[test]
fn rejects_mixed_context_function_params() {
    let first = Context::create();
    let second = Context::create();
    let return_type = Type::void(&first);
    let param = Type::i32(&second);

    assert!(Type::function(&return_type, &[param], false).is_err());
}
