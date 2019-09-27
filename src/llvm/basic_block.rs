use llvm_sys::core::LLVMBasicBlockAsValue;
use llvm_sys::core::LLVMGetBasicBlockName;
use llvm_sys::core::LLVMGetBasicBlockParent;
use llvm_sys::core::LLVMGetBasicBlockTerminator;
use llvm_sys::core::LLVMGetFirstInstruction;
use llvm_sys::core::LLVMGetLastInstruction;
use llvm_sys::core::LLVMGetNextBasicBlock;
use llvm_sys::core::LLVMGetPreviousBasicBlock;
use llvm_sys::core::LLVMInsertBasicBlock;
use llvm_sys::core::LLVMInsertBasicBlockInContext;
use llvm_sys::core::LLVMMoveBasicBlockAfter;
use llvm_sys::core::LLVMMoveBasicBlockBefore;
use llvm_sys::core::LLVMRemoveBasicBlockFromParent;
use llvm::BasicBlock;
use llvm::Context;
use llvm::Value;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::PhantomData;

impl<'a> BasicBlock<'a> {
    pub fn as_basic_block(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMBasicBlockAsValue(self.block),
                    phantom: PhantomData }
        }
    }

    pub fn name(&self) -> Cow<'a, str> {
        unsafe {
            CStr::from_ptr(LLVMGetBasicBlockName(self.block)).to_string_lossy()
        }
    }

    pub fn parent(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetBasicBlockParent(self.block),
                    phantom: PhantomData }
        }
    }

    pub fn terminator(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetBasicBlockTerminator(self.block),
                    phantom: PhantomData }
        }
    }

    pub fn next(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMGetNextBasicBlock(self.block),
                         phantom: PhantomData }
        }
    }

    pub fn prev(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMGetPreviousBasicBlock(self.block),
                         phantom: PhantomData }
        }
    }

    pub fn insert_block(&self, name: &str) -> BasicBlock<'a> {
        unsafe {
            let str = CString::new(name).unwrap().into_raw();

            BasicBlock { block: LLVMInsertBasicBlock(self.block, str),
                         phantom: PhantomData }
        }
    }

    pub fn insert_block_in_ctx(&self, c: Context<'a>,
                               name: &str) -> BasicBlock<'a> {
        unsafe {
            let str = CString::new(name).unwrap().into_raw();

            BasicBlock { block: LLVMInsertBasicBlockInContext(c.ctx, self.block,
                                                              str),
                         phantom: PhantomData }
        }
    }

    pub fn remove_from_parent(&self) {
        unsafe {
            LLVMRemoveBasicBlockFromParent(self.block);
        }
    }

    pub fn move_before(&self, other: BasicBlock<'a>) {
        unsafe {
            LLVMMoveBasicBlockBefore(self.block, other.block);
        }
    }

    pub fn move_after(&self, other: BasicBlock<'a>) {
        unsafe {
            LLVMMoveBasicBlockAfter(self.block, other.block);
        }
    }

    pub fn first_instr(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetFirstInstruction(self.block),
                    phantom: PhantomData }
        }
    }

    pub fn last_instr(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetLastInstruction(self.block),
                    phantom: PhantomData }
        }
    }
}
