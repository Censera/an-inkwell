use std::ffi::CString;
use std::ptr::{null_mut, NonNull};

use llvm_sys::core::{LLVMDisposeMessage, LLVMSetTarget};
use llvm_sys::target::{
    LLVMInitializeAArch64AsmPrinter, LLVMInitializeAArch64Target,
    LLVMInitializeAArch64TargetInfo, LLVMInitializeAArch64TargetMC, LLVMInitializeX86AsmPrinter,
    LLVMInitializeX86Target, LLVMInitializeX86TargetInfo, LLVMInitializeX86TargetMC,
    LLVMSetModuleDataLayout,
};
use llvm_sys::target_machine::{
    LLVMCodeGenFileType, LLVMCodeGenLevelDefault, LLVMCodeModelDefault, LLVMCreateTargetDataLayout,
    LLVMCreateTargetMachine, LLVMDisposeTargetData, LLVMDisposeTargetMachine,
    LLVMGetTargetFromTriple, LLVMTargetMachineEmitToFile, LLVMTargetMachineRef, LLVMRelocDefault,
    LLVMTargetRef,
};

use crate::{Error, Module};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Architecture {
    X86_64,
    AArch64,
}

impl Architecture {
    fn target_name(self) -> &'static [u8] {
        match self {
            Self::X86_64 => b"x86-64\0",
            Self::AArch64 => b"aarch64\0",
        }
    }

    fn matches_target(self, target: LLVMTargetRef) -> bool {
        let name = unsafe { llvm_sys::target_machine::LLVMGetTargetName(target) };
        if name.is_null() {
            return false;
        }

        unsafe { std::ffi::CStr::from_ptr(name) }.to_bytes_with_nul() == self.target_name()
    }
}

pub fn initialize_targets() {
    unsafe {
        LLVMInitializeX86TargetInfo();
        LLVMInitializeX86Target();
        LLVMInitializeX86TargetMC();
        LLVMInitializeX86AsmPrinter();
        LLVMInitializeAArch64TargetInfo();
        LLVMInitializeAArch64Target();
        LLVMInitializeAArch64TargetMC();
        LLVMInitializeAArch64AsmPrinter();
    }
}

pub struct TargetMachine {
    raw: NonNull<std::ffi::c_void>,
    triple: CString,
}

impl TargetMachine {
    pub fn new(architecture: Architecture, triple: &str) -> Result<Self, Error> {
        initialize_targets();

        let triple = CString::new(triple)?;
        let mut target: LLVMTargetRef = null_mut();
        let mut error = null_mut();
        let failed = unsafe {
            LLVMGetTargetFromTriple(triple.as_ptr(), &mut target, &mut error)
        } != 0;

        if failed || target.is_null() {
            return Err(Error::TargetCreation(take_llvm_error(error)));
        }
        if !architecture.matches_target(target) {
            return Err(Error::UnsupportedTarget);
        }

        let raw = unsafe {
            LLVMCreateTargetMachine(
                target,
                triple.as_ptr(),
                c"".as_ptr(),
                c"".as_ptr(),
                LLVMCodeGenLevelDefault,
                LLVMRelocDefault,
                LLVMCodeModelDefault,
            )
        };

        let Some(raw) = NonNull::new(raw.cast()) else {
            return Err(Error::TargetMachineCreation);
        };

        Ok(Self { raw, triple })
    }

    pub fn emit_object(&self, module: &Module<'_>, path: &std::path::Path) -> Result<(), Error> {
        let path = CString::new(path.to_string_lossy().as_bytes())?;

        unsafe {
            LLVMSetTarget(module.as_raw(), self.triple.as_ptr());

            let data_layout = LLVMCreateTargetDataLayout(self.as_raw());
            if data_layout.is_null() {
                return Err(Error::TargetDataLayoutCreation);
            }
            LLVMSetModuleDataLayout(module.as_raw(), data_layout);
            LLVMDisposeTargetData(data_layout);

            let mut error = null_mut();
            let failed = LLVMTargetMachineEmitToFile(
                self.as_raw(),
                module.as_raw(),
                path.as_ptr(),
                LLVMCodeGenFileType::LLVMObjectFile,
                &mut error,
            ) != 0;

            if failed {
                return Err(Error::ObjectEmission(take_llvm_error(error)));
            }
        }

        Ok(())
    }

    fn as_raw(&self) -> LLVMTargetMachineRef {
        self.raw.as_ptr().cast()
    }
}

impl Drop for TargetMachine {
    fn drop(&mut self) {
        unsafe { LLVMDisposeTargetMachine(self.as_raw()) };
    }
}

fn take_llvm_error(error: *mut std::ffi::c_char) -> String {
    if error.is_null() {
        return String::from("unknown LLVM target error");
    }

    let message = unsafe { std::ffi::CStr::from_ptr(error) }
        .to_string_lossy()
        .into_owned();
    unsafe { LLVMDisposeMessage(error) };
    message
}

#[cfg(test)]
mod tests {
    use super::{initialize_targets, Architecture, TargetMachine};
    use crate::{Context, Type};

    #[test]
    fn initializes_supported_targets() {
        initialize_targets();

        let x86 = TargetMachine::new(Architecture::X86_64, "x86_64-unknown-linux-gnu");
        let aarch64 = TargetMachine::new(Architecture::AArch64, "aarch64-unknown-linux-gnu");

        assert!(x86.is_ok());
        assert!(aarch64.is_ok());
    }

    #[test]
    fn rejects_wrong_architecture() {
        let result = TargetMachine::new(Architecture::AArch64, "x86_64-unknown-linux-gnu");
        assert!(matches!(result, Err(crate::Error::UnsupportedTarget)));
    }

    #[test]
    fn emits_object_file() {
        let context = Context::create();
        let module = context.module("test").unwrap();
        let integer = Type::i32(&context);
        let function_type = Type::function(&integer, &[], false).unwrap();
        let function = module.function("main", &function_type).unwrap();
        let block = function.block("entry").unwrap();
        let builder = context.builder().unwrap();
        builder.position(&block).unwrap();
        let value = crate::Value::integer(&integer, 0, false);
        builder.ret(&value).unwrap();

        let machine = TargetMachine::new(Architecture::X86_64, "x86_64-unknown-linux-gnu").unwrap();
        let path = std::env::temp_dir().join("an-inkwell-test.o");
        machine.emit_object(&module, &path).unwrap();
        assert!(std::fs::metadata(&path).unwrap().len() > 0);
        std::fs::remove_file(path).unwrap();
    }
}
