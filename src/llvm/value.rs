use llvm_sys::core::LLVMAppendBasicBlock;
use llvm_sys::core::LLVMAppendBasicBlockInContext;
use llvm_sys::core::LLVMCountBasicBlocks;
use llvm_sys::core::LLVMGetBasicBlocks;
use llvm_sys::core::LLVMGetEntryBasicBlock;
use llvm_sys::core::LLVMGetFirstBasicBlock;
use llvm_sys::core::LLVMGetFirstUse;
use llvm_sys::core::LLVMGetLastBasicBlock;
use llvm_sys::core::LLVMGetNextUse;
use llvm_sys::core::LLVMGetUsedValue;
use llvm_sys::core::LLVMGetUser;
use llvm_sys::core::LLVMIsAMDNode;
use llvm_sys::core::LLVMIsAMDString;
use llvm_sys::core::LLVMIsConstant;
use llvm_sys::core::LLVMIsUndef;
use llvm_sys::core::LLVMPrintValueToString;
use llvm_sys::core::LLVMReplaceAllUsesWith;
use llvm_sys::core::LLVMTypeOf;
use llvm_sys::core::LLVMValueIsBasicBlock;
use llvm_sys::core::LLVMValueAsBasicBlock;
use llvm_sys::prelude::LLVMUseRef;
use llvm::BasicBlock;
use llvm::Context;
use llvm::Type;
use llvm::Value;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::PhantomData;

pub struct Use<'a> {
    useref: LLVMUseRef,
    phantom: PhantomData<&'a LLVMUseRef>
}

impl<'a> Use<'a> {
    pub fn next(&self) -> Use<'a> {
        unsafe {
            Use { useref: LLVMGetNextUse(self.useref),
                  phantom: PhantomData }
        }
    }

    pub fn user(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetUser(self.useref),
                    phantom: PhantomData }
        }
    }

    pub fn value(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetUser(self.useref),
                    phantom: PhantomData }
        }
    }
}

impl<'a> Value<'a> {
    pub fn type_of(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMTypeOf(self.value), phantom: PhantomData }
        }
    }

    /// Get a `str` representation of this `Value`.
    pub fn to_str(&self) -> Cow<'a, str> {
        unsafe {
            CStr::from_ptr(LLVMPrintValueToString(self.value)).to_string_lossy()
        }
    }

    /// Get a `String` representation of this `Value`.
    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }

    pub fn replace_with(&self, new: Value<'a>) {
        unsafe {
            LLVMReplaceAllUsesWith(self.value, new.value);
        }
    }

    pub fn is_constant(&self) -> bool {
        unsafe {
            LLVMIsConstant(self.value) != 0
        }
    }

    pub fn is_undef(&self) -> bool {
        unsafe {
            LLVMIsUndef(self.value) != 0
        }
    }
/*
    pub fn is_md_node(&self) -> bool {
        unsafe {
            LLVMIsAMDNode(self.value) != 0
        }
    }

    pub fn is_md_string(&self) -> bool {
        unsafe {
            LLVMIsAMDString(self.value) != 0
        }
    }
*/
    pub fn first_use(&self) -> Use<'a> {
        unsafe {
            Use { useref: LLVMGetFirstUse(self.value),
                  phantom: PhantomData }
        }
    }

    pub fn is_basic_block(&self) -> bool {
        unsafe {
            LLVMValueIsBasicBlock(self.value) != 0
        }
    }

    pub fn as_basic_block(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMValueAsBasicBlock(self.value),
                         phantom: PhantomData }
        }
    }

    pub fn basic_blocks(&self) -> Vec<BasicBlock<'a>> {
        unsafe {
            let len = self.count_basic_blocks() as usize;
            let mut vec = Vec::with_capacity(len);

            LLVMGetBasicBlocks(self.value, vec.as_mut_slice().as_mut_ptr());

            let mut out = Vec::with_capacity(len);

            for i in 0..len {
                out.push(BasicBlock { block: vec[i], phantom: PhantomData })
            }

            out
        }
    }

    pub fn count_basic_blocks(&self) -> u32 {
        unsafe {
            LLVMCountBasicBlocks(self.value)
        }
    }

    pub fn entry_basic_block(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMGetEntryBasicBlock(self.value),
                         phantom: PhantomData }
        }
    }

    pub fn first_basic_block(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMGetFirstBasicBlock(self.value),
                         phantom: PhantomData }
        }
    }

    pub fn last_basic_block(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMGetLastBasicBlock(self.value),
                         phantom: PhantomData }
        }
    }

    pub fn append_block(&self, name: &str) -> BasicBlock<'a> {
        unsafe {
            let str = CString::new(name).unwrap().into_raw();

            BasicBlock { block: LLVMAppendBasicBlock(self.value, str),
                         phantom: PhantomData }
        }
    }

    pub fn append_block_in_ctx(&self, c: Context<'a>,
                               name: &str) -> BasicBlock<'a> {
        unsafe {
            let str = CString::new(name).unwrap().into_raw();

            BasicBlock { block: LLVMAppendBasicBlockInContext(c.ctx, self.value,
                                                              str),
                         phantom: PhantomData }
        }
    }
}
