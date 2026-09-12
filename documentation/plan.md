# Plan

`an-inkwell` grows from the LLVM boundary outward. The plan defines the order of capabilities; `documentation/workflow.md` defines how each capability is completed.

## Foundation

Establish the crate, explicit errors, LLVM context ownership, module ownership, builder ownership, private raw handles, and the basic test structure.

## Types

Add only the LLVM types required by Astery v26:

- Void.
- Integer.
- Floating point.
- Pointer.
- Array.
- Vector.
- Struct.
- Function.

The mapping from Astery language types to LLVM types belongs here only where code generation requires it.

## Values

Add concrete value wrappers needed to construct and manipulate LLVM IR. Distinguish values by actual LLVM behavior rather than reproducing a large hierarchy for symmetry.

## IR construction

Complete the IR surface in small, independently tested operations:

- Arithmetic.
- Comparisons.
- Logical operations.
- Bitwise operations.
- Casts.
- Allocation.
- Load and store.
- Address calculation.
- Aggregate access.
- Calls.
- Returns.
- Branches.
- Conditional branches.
- Phi values.

Each operation becomes a separate TODO item and follows the workflow from selection through validation and commit.

## Targets

Define and implement only the target support Astery actually requires:

- Target initialization.
- Target machine support.
- Object emission.

## Astery backend

Use `an-inkwell` from real Astery backend code only after the required library surface has been implemented and exercised independently.

The plan is deliberately sequential. A later requirement is evidence for a needed capability, not permission to implement its abstractions early.
