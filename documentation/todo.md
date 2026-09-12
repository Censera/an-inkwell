# TODO

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

- [ ] Add arithmetic operations.
- [ ] Add comparison operations.
- [ ] Add logical operations.
- [ ] Add bitwise operations.
- [ ] Add casts.
- [ ] Add allocation.
- [ ] Add load and store.
- [ ] Add address calculation.
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
