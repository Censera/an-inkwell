use std::ffi::NulError;

#[derive(Debug)]
pub enum Error {
    Name(NulError),
    ModuleCreation,
    BuilderCreation,
    DifferentContext,
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
            Self::DifferentContext => formatter.write_str("LLVM types belong to different contexts"),
        }
    }
}

impl std::error::Error for Error {}
