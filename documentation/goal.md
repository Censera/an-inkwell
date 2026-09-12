# Goal

`an-inkwell` is Astery's Narrower Inkwell.

It is a standalone Rust library that exposes the part of LLVM needed by Astery through a deliberately smaller API than Inkwell.

The library is not an Inkwell fork and does not aim for API compatibility with Inkwell.

## Scope

The library exists to give Astery a direct, explicit LLVM interface for its compiler backend.

Astery v26 is statically typed and includes integers, floating point values, booleans, characters, strings, pointers, arrays, vectors, tuples, structs, enums, functions, casts, control flow, function attributes, variadic functions, macros, and C embedding. These language features define the eventual LLVM surface that the library must support.

Astery is intended to be compiled and statically typed, with LLVM serving the compiler rather than defining the language itself.

## Non-goals

- Reimplement all of Inkwell.
- Preserve Inkwell's API.
- Support LLVM features without a compiler need.
- Build speculative abstractions for future users.
- Hide LLVM concepts behind a large framework.
- Introduce dynamic dispatch where static types are sufficient.

## Result

A Rust crate that is small enough to understand directly, narrow enough to evolve with Astery, and useful as an independent published library.
