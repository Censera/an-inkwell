use std::ffi::NulError;

#[derive(Debug)]
pub enum Error {
    Name(NulError),
    ModuleCreation,
    BuilderCreation,
    DifferentContext,
    NotPointerType,
    NotBoolean,
    EmptyAggregate,
    FunctionCreation,
    BlockCreation,
    TargetCreation(String),
    UnsupportedTarget,
    TargetMachineCreation,
    TargetDataLayoutCreation,
    ObjectEmission(String),
}

impl From<NulError> for Error {
    fn from(error: NulError) -> Self {
        Self::Name(error)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name(error) => write!(formatter, "LLVM name contains an interior NUL: {error}"),
            Self::ModuleCreation => formatter.write_str("LLVM failed to create a module"),
            Self::BuilderCreation => formatter.write_str("LLVM failed to create a builder"),
            Self::DifferentContext => {
                formatter.write_str("LLVM values or types belong to different contexts")
            }
            Self::NotPointerType => formatter.write_str("LLVM null value requires a pointer type"),
            Self::NotBoolean => {
                formatter.write_str("LLVM logical operation requires boolean values")
            }
            Self::EmptyAggregate => {
                formatter.write_str("LLVM aggregate value requires at least one value")
            }
            Self::FunctionCreation => formatter.write_str("LLVM failed to create a function"),
            Self::BlockCreation => formatter.write_str("LLVM failed to create a basic block"),
            Self::TargetCreation(error) => write!(formatter, "LLVM target lookup failed: {error}"),
            Self::UnsupportedTarget => formatter.write_str("LLVM target is outside the supported architectures"),
            Self::TargetMachineCreation => formatter.write_str("LLVM failed to create a target machine"),
            Self::TargetDataLayoutCreation => formatter.write_str("LLVM failed to create target data layout"),
            Self::ObjectEmission(error) => write!(formatter, "LLVM object emission failed: {error}"),
        }
    }
}

impl std::error::Error for Error {}
