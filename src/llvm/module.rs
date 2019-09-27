use llvm_sys::core::LLVMDisposeModule;
use llvm_sys::core::LLVMGetDataLayoutStr;
use llvm_sys::core::LLVMGetModuleIdentifier;
//use llvm_sys::core::LLVMGetSourceFileName;
use llvm_sys::prelude::LLVMModuleRef;
use std::borrow::Cow;
use std::ops::Drop;
use std::slice;

pub struct Module(LLVMModuleRef);

impl<'a> Module {
    /// Get the name of the module as a `Cow` string.
    pub fn get_identifier(&'a self) -> Cow<'a, str> {
        unsafe {
            let mut len = 0;
            let strbytes = LLVMGetModuleIdentifier(self.0, &mut len);
            let strslice = slice::from_raw_parts(strbytes as *const u8, len);

            String::from_utf8_lossy(strslice)
        }
    }
/*
    /// Get the source file name as a `Cow` string.
    pub fn get_source_file_name(&'a self) -> Cow<'a, str> {
        unsafe {
            let mut len = 0;
            let strbytes = LLVMGetSourceFileName(self.0, &mut len);
            let strslice = slice::from_raw_parts(strbytes as *const u8, len);

            String::from_utf8_lossy(strslice)
        }
    }
*/
}

impl Drop for Module {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.0)
        }
    }
}
