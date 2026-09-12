# Target support

Astery initially targets the common 64-bit desktop/server architectures:

- x86-64 (`x86_64`)
- AArch64 (`aarch64`)

Both targets are required for native code generation and object emission.

The initial target scope does not include 32-bit x86, 32-bit ARM, RISC-V, PowerPC, or other architectures. Those can be added only when Astery has a concrete requirement for them.

LLVM target initialization, target machine creation, and object emission are separate implementation steps.
