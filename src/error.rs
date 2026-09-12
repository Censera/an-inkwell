use std::ffi::NulError;

#[derive(Debug)]
pub enum Error {
    Name(NulError),
    ModuleCreation,
    BuilderCreation,
    DifferentContext,
    NotPointerType,
    EmptyAggregate,
    FunctionCreation,
    BlockCreation,
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
            Self::DifferentContext => formatter.write_str("LLVM values or types belong to different contexts"),
            Self::NotPointerType => formatter.write_str("LLVM null value requires a pointer type"),
            Self::EmptyAggregate => formatter.write_str("LLVM aggregate value requires at least one value"),
            Self::FunctionCreation => formatter.write_str("LLVM failed to create a function"),
            Self::BlockCreation => formatter.write_str("LLVM failed to create a basic block"),
        }
    }
}

impl std::error::Error for Error {}
