# TODO

The workflow requires taking the first unchecked item and completing it before advancing.

## Foundation

- [x] Create the standalone crate.
- [x] Establish Rust 2024 edition.
- [x] Add the LLVM FFI dependency.
- [x] Add explicit library errors.
- [x] Add `Context` ownership.
- [x] Add `Module` ownership.
- [x] Add `Builder` ownership.
- [x] Add crate-level tests for context, module, and builder lifetime behavior.

## Types

- [x] Add integer types.
- [x] Add floating-point types.
- [x] Add void type.
- [x] Add pointer types.
- [x] Add array types.
- [x] Add vector types.
- [x] Add struct types.
- [x] Add function types.

## Values

- [x] Add integer constants.
- [x] Add floating-point constants.
- [x] Add pointer values.
- [x] Add aggregate values.
- [x] Add function values.
- [x] Add basic blocks.

## IR

- [x] Add arithmetic operations.
- [x] Add comparison operations.
- [x] Add logical operations.
- [x] Add bitwise operations.
- [x] Add casts.
- [x] Add allocation.
- [x] Add load and store.
- [x] Add address calculation.
- [ ] Add aggregate access.
- [ ] Add calls.
- [ ] Add returns.
- [ ] Add branches.
- [ ] Add conditional branches.
- [ ] Add phi values.

## Targets

- [ ] Define target support required by Astery.
- [ ] Add target initialization.
- [ ] Add target machine support.
- [ ] Add object emission.

## Astery

- [ ] Integrate the first real Astery backend path.
- [ ] Remove the corresponding direct Inkwell dependency from Astery.
- [ ] Keep `an-inkwell` independent from Astery's compiler internals.

## Completion rule

An item changes to `[x]` only after its implementation and tests are complete, `cargo fmt` and `cargo test` pass locally, all warnings are fixed, the ownership and lifetime model has been checked, and the completed work has an intentional commit.
