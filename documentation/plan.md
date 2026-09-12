# Plan

The library grows from the LLVM boundary outward.

## 1. Foundation

- Establish the published crate.
- Establish explicit error handling.
- Own LLVM contexts, modules, and builders.
- Keep raw handles private.
- Keep the public surface small.

## 2. Types

Add only the LLVM types required by Astery v26:

- Void.
- Integer.
- Floating point.
- Pointer.
- Array.
- Vector.
- Struct.
- Function.

Astery's language type set includes signed and unsigned integers, floating point values, booleans, characters, strings, non-null pointers, optional pointers, arrays, vectors, tuples, structs, and enums. The mapping from those language types to LLVM types belongs here only where code generation requires it.

## 3. Values

Add concrete value wrappers needed to construct and manipulate LLVM IR.

The design should distinguish values by actual LLVM behavior rather than reproduce a large enum hierarchy solely for API symmetry.

## 4. IR construction

Add the operations Astery needs for:

- Constants.
- Arithmetic.
- Comparisons.
- Logical and bitwise operations.
- Casts.
- Loads and stores.
- Allocation.
- Address calculation.
- Aggregate access.
- Function calls.
- Returns.
- Branches.
- Conditional branches.
- Basic blocks.
- Phi values.

## 5. Functions and aggregates

Support function declarations and definitions, parameters, return values, structs, arrays, vectors, and the operations needed by Astery's aggregate types.

## 6. Targets

Add target initialization and target-machine functionality when Astery's compiler requires object generation or target-specific configuration.

## 7. Backend use

Replace Astery's direct Inkwell dependency with `an-inkwell` only after the required surface exists and is exercised by real Astery code.

The plan is intentionally sequential. A later layer does not justify implementing an earlier abstraction prematurely.
