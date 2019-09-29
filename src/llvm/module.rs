use llvm::Module;
use llvm::Type;
use llvm::Value;
use llvm_sys::analysis::LLVMVerifierFailureAction;
use llvm_sys::analysis::LLVMVerifyModule;
use llvm_sys::core::LLVMAddFunction;
use llvm_sys::core::LLVMAppendModuleInlineAsm;
use llvm_sys::core::LLVMCloneModule;
use llvm_sys::core::LLVMDisposeModule;
use llvm_sys::core::LLVMGetDataLayoutStr;
use llvm_sys::core::LLVMGetNamedFunction;
use llvm_sys::core::LLVMGetTarget;
use llvm_sys::core::LLVMGetTypeByName;
use llvm_sys::core::LLVMGetModuleIdentifier;
use llvm_sys::core::LLVMGetModuleInlineAsm;
use llvm_sys::core::LLVMGetFirstFunction;
use llvm_sys::core::LLVMGetNextFunction;
use llvm_sys::core::LLVMGetSourceFileName;
use llvm_sys::core::LLVMPrintModuleToString;
use llvm_sys::core::LLVMSetDataLayout;
use llvm_sys::core::LLVMSetModuleIdentifier;
use llvm_sys::core::LLVMSetModuleInlineAsm2;
use llvm_sys::core::LLVMSetSourceFileName;
use llvm_sys::core::LLVMSetTarget;
use llvm_sys::prelude::LLVMModuleRef;
use llvm_sys::prelude::LLVMValueRef;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Drop;
use std::marker::PhantomData;
use std::ptr::null_mut;
use std::result::Result;
use std::slice;

pub enum ModuleFlagBehavior {
    /// Emits an error if two values disagree, otherwise the resulting
    /// value is that of the operands.
    Error,
    /// Emits a warning if two values disagree. The result value will
    /// be the operand for the flag from the first module being
    /// linked.
    Warning,
    /// Adds a requirement that another module flag be present and
    /// have a specified value after linking is performed. The value
    /// must be a metadata pair, where the first element of the pair
    /// is the ID of the module flag to be restricted, and the second
    /// element of the pair is the value the module flag should be
    /// restricted to. This behavior can be used to restrict the
    /// allowable results (via triggering of an error) of linking IDs
    /// with the **Override** behavior.
    Require,
    /// Uses the specified value, regardless of the behavior or value
    /// of the other module. If both modules specify **Override**, but
    /// the values differ, an error will be emitted.
    Override,
    /// Appends the two values, which are required to be metadata nodes.
    Append,
    /// Appends the two values, which are required to be metadata
    /// nodes. However, duplicate entries in the second list are
    /// dropped during the append operation.
    AppendUnique
}

pub struct FunctionIter<'a> {
    next: LLVMValueRef,
    phantom: PhantomData<&'a LLVMValueRef>
}

impl<'a> Module<'a> {
    /// Get the name of the module as a `Cow` string.
    pub fn id(&self) -> Option<Cow<'a, str>> {
        unsafe {
            let mut len = 0;
            let strbytes = LLVMGetModuleIdentifier(self.module, &mut len);

            if !strbytes.is_null() {
                let strslice = slice::from_raw_parts(strbytes as *const u8,
                                                     len);

                Some(String::from_utf8_lossy(strslice))
            } else {
                None
            }
        }
    }

    pub fn verify(&self) -> Result<(), Cow<'a, str>> {
        unsafe {
            let mut ptr = null_mut();

            if LLVMVerifyModule(self.module,
                                LLVMVerifierFailureAction::
                                LLVMReturnStatusAction,
                                &mut ptr) == 0 {
                Ok(())
            } else {
                let cstr = CString::from_raw(ptr);
                let strslice = slice::from_raw_parts(ptr as *const u8,
                                                     cstr.as_bytes().len());

                Err(String::from_utf8_lossy(strslice))
            }
        }
    }

    /// Set the name of the module to `name`.
    pub fn set_id(&mut self, name: &str) {
        unsafe {
            let vec: Vec<u8> = name.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            LLVMSetModuleIdentifier(self.module, cstr.unwrap().as_ptr(), len)
        }
    }

    /// Get the source file name as a `Cow` string.
    pub fn src_file_name(&self) -> Option<Cow<'a, str>> {
        unsafe {
            let mut len = 0;
            let strbytes = LLVMGetSourceFileName(self.module, &mut len);

            if !strbytes.is_null() {
                let strslice = slice::from_raw_parts(strbytes as *const u8,
                                                     len);

                Some(String::from_utf8_lossy(strslice))
            } else {
                None
            }
        }
    }

    /// Set the source file name of the module to `fname`.
    pub fn set_src_file_name(&mut self, fname: &str) {
        unsafe {
            let vec: Vec<u8> = fname.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            LLVMSetSourceFileName(self.module, cstr.unwrap().as_ptr(), len)
        }
    }

    /// Get the data layout string.
    pub fn data_layout(&self) -> Option<Cow<'a, str>> {
        unsafe {
            let strbytes = LLVMGetDataLayoutStr(self.module);

            if !strbytes.is_null() {
                Some(CStr::from_ptr(strbytes).to_string_lossy())
            } else {
                None
            }
        }
    }

    /// Set the data layout string to `layout`.
    pub fn set_data_layout(&mut self, layout: &str) {
        unsafe {
            let cstr = CString::new(layout);

            LLVMSetDataLayout(self.module, cstr.unwrap().into_raw())
        }
    }

    /// Get the inline assembly for this module.
    pub fn inline_asm(&self) -> Option<Cow<'a, str>> {
        unsafe {
            let mut len = 0;
            let strbytes = LLVMGetModuleInlineAsm(self.module, &mut len);

            if !strbytes.is_null() {
                let strslice = slice::from_raw_parts(strbytes as *const u8,
                                                     len);

                Some(String::from_utf8_lossy(strslice))
            } else {
                None
            }
        }
    }

    /// Set the inline assembly of the module to `asm`.
    pub fn set_inline_asm(&mut self, asm: &str) {
        unsafe {
            let vec: Vec<u8> = asm.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            LLVMSetModuleInlineAsm2(self.module, cstr.unwrap().as_ptr(), len)
        }
    }

    /// Append the inline assembly `asm` to the module.
    pub fn append_inline_asm(&mut self, asm: &str) {
        unsafe {
            let vec: Vec<u8> = asm.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            LLVMAppendModuleInlineAsm(self.module, cstr.unwrap().as_ptr(),
                                      len)
        }
    }

    /// Get the target.
    pub fn target(&'a self) -> Option<Cow<'a, str>> {
        unsafe {
            let strbytes = LLVMGetTarget(self.module);

            if !strbytes.is_null() {
                Some(CStr::from_ptr(strbytes).to_string_lossy())
            } else {
                None
            }
        }
    }

    /// Set the target to `target`.
    pub fn set_target(&'a mut self, target: &str) {
        unsafe {
            let cstr = CString::new(target);

            LLVMSetDataLayout(self.module, cstr.unwrap().into_raw())
        }
    }

    /// Get the type with name `name` in the module.
    pub fn type_by_name<'b: 'a>(&self, name: &str) -> Type<'b> {
        unsafe {
            let cstr = CString::new(name);

            Type { ty: LLVMGetTypeByName(self.module, cstr.unwrap().into_raw()),
                   phantom: PhantomData }
        }
    }

    /// Get the target.
    pub fn to_str(&'a self) -> Cow<'a, str> {
        unsafe {
            CStr::from_ptr(LLVMPrintModuleToString(self.module))
                           .to_string_lossy()
        }
    }

    /// Add a new function with name `name` and type `ty`.
    pub fn add_function<'b: 'a>(&mut self, name: &str,
                                ty: &Type<'a>) -> Value<'b> {
        unsafe {
            let cstr = CString::new(name);

            Value { value: LLVMAddFunction(self.module,
                                           cstr.unwrap().into_raw(),
                                           ty.ty),
                   phantom: PhantomData }
        }
    }

    /// Get the function `name` in the module.
    pub fn named_function<'b: 'a>(&mut self, name: &str) -> Value<'b> {
        unsafe {
            let cstr = CString::new(name);

            Value { value: LLVMGetNamedFunction(self.module,
                                                cstr.unwrap().into_raw()),
                   phantom: PhantomData }
        }
    }

    /// Get an `Iterator` on functions in the module.
    pub fn function_iter<'b: 'a>(&self) -> FunctionIter<'b> {
        unsafe {
            FunctionIter { next: LLVMGetFirstFunction(self.module),
                           phantom: PhantomData }
        }
    }
}

impl<'a> Iterator for FunctionIter<'a> {
    type Item = Value<'a>;

    fn next(&mut self) -> Option<Value<'a>> {
        unsafe {
            if self.next.is_null() {
                None
            } else {
                let out = self.next;

                self.next = LLVMGetNextFunction(out);

                Some(Value { value: out, phantom: PhantomData })
            }
        }
    }
}

impl<'a> Clone for Module<'a> {
    fn clone(&self) -> Module<'a> {
        unsafe {
            Module { module: LLVMCloneModule(self.module),
                     phantom: PhantomData }
        }
    }
}

impl<'a> Display for Module<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeModule(self.module)
        }
    }
}
