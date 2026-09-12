# Design

`an-inkwell` is a narrow LLVM interface for Rust.

## Shape

The public API is organized around a small set of concrete objects:

- `Context` owns LLVM context state.
- `Module` owns an LLVM module and is tied to its context.
- `Builder` owns an LLVM IR builder and is tied to its context.
- `Type` describes an LLVM type and is tied to its context.

More types are added only when Astery's backend requires them.

## Ownership

LLVM handles are owned by Rust wrapper types where LLVM gives the library ownership. Types are context-owned handles, so `Type` borrows its `Context` instead of dropping the LLVM type itself.

Rust lifetimes express relationships that already exist in LLVM usage instead of introducing shared mutable state or runtime dispatch.

The current relationships are:

```text
Context
 ├── Module
 ├── Builder
 └── Type
```

A module, builder, or type cannot outlive its context.

## API

Names should be short, concrete, and one word where possible. Avoid underscore chains and names that describe implementation layers instead of the object itself.

The public API stays smaller than the implementation. Raw LLVM handles remain private.

`Type` provides the LLVM types required by Astery without reproducing Inkwell's type hierarchy. Integer, floating-point, pointer, array, vector, struct, and function types are constructed directly from the LLVM C API.

## LLVM boundary

LLVM C API calls are isolated inside the library. `unsafe` is used only where the LLVM API requires it, and each unsafe operation owns a specific invariant.

The library should prefer direct LLVM operations over building a second general-purpose abstraction system on top of LLVM.

## Static design

The library does not use `dyn Trait` as an architectural mechanism. Static dispatch and concrete types are the default.

There is no runtime plugin system, registry, reflection layer, or hidden global state in the core design.

## Reference

Inkwell is the primary API and implementation reference. Its useful concepts include a context that creates modules and builders and typed wrappers around LLVM objects. `an-inkwell` narrows that model instead of reproducing its full hierarchy.

Astery 26.9 is the consumer reference. Its language design determines which LLVM types, values, operations, control flow, functions, pointers, aggregates, casts, and targets eventually belong in the library.

## Naming

Names should form a clean visual vocabulary:

```text
Context
Module
Builder
Type
Value
Function
Block
Target
```

A name earns its place by describing a real concept in the library. Artificial suffixes and layered names are avoided.
