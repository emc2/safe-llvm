use llvm_sys::core::LLVMAppendBasicBlock;
use llvm_sys::core::LLVMAppendBasicBlockInContext;
use llvm_sys::core::LLVMConstArray;
use llvm_sys::core::LLVMConstAllOnes;
use llvm_sys::core::LLVMConstInt;
use llvm_sys::core::LLVMConstIntOfArbitraryPrecision;
use llvm_sys::core::LLVMConstIntOfStringAndSize;
use llvm_sys::core::LLVMConstIntGetSExtValue;
use llvm_sys::core::LLVMConstIntGetZExtValue;
use llvm_sys::core::LLVMConstNamedStruct;
use llvm_sys::core::LLVMConstReal;
use llvm_sys::core::LLVMConstRealOfStringAndSize;
use llvm_sys::core::LLVMConstRealGetDouble;
use llvm_sys::core::LLVMConstStringInContext;
use llvm_sys::core::LLVMConstStructInContext;
use llvm_sys::core::LLVMConstNull;
use llvm_sys::core::LLVMConstPointerNull;
use llvm_sys::core::LLVMConstVector;
use llvm_sys::core::LLVMCountBasicBlocks;
use llvm_sys::core::LLVMGetBasicBlocks;
use llvm_sys::core::LLVMGetEntryBasicBlock;
use llvm_sys::core::LLVMGetFirstBasicBlock;
use llvm_sys::core::LLVMGetAsString;
use llvm_sys::core::LLVMGetElementAsConstant;
use llvm_sys::core::LLVMGetFirstUse;
use llvm_sys::core::LLVMGetLastBasicBlock;
use llvm_sys::core::LLVMGetOperand;
use llvm_sys::core::LLVMGetOperandUse;
use llvm_sys::core::LLVMGetNextUse;
use llvm_sys::core::LLVMGetNumOperands;
use llvm_sys::core::LLVMGetUndef;
use llvm_sys::core::LLVMGetUsedValue;
use llvm_sys::core::LLVMGetUser;
use llvm_sys::core::LLVMGetValueName2;
use llvm_sys::core::LLVMIsAMDNode;
use llvm_sys::core::LLVMIsAMDString;
use llvm_sys::core::LLVMIsConstant;
use llvm_sys::core::LLVMIsUndef;
use llvm_sys::core::LLVMPrintValueToString;
use llvm_sys::core::LLVMReplaceAllUsesWith;
use llvm_sys::core::LLVMSetOperand;
use llvm_sys::core::LLVMSetValueName2;
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
use std::slice;

pub struct Use<'a> {
    useref: LLVMUseRef,
    phantom: PhantomData<&'a LLVMUseRef>
}

pub struct UseIter<'a> {
    next: LLVMUseRef,
    phantom: PhantomData<&'a LLVMUseRef>
}

impl<'a> Use<'a> {
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

impl<'a> Iterator for UseIter<'a> {
    type Item = Use<'a>;

    fn next(&mut self) -> Option<Use<'a>> {
        unsafe {
            if self.next.is_null() {
                None
            } else {
                let out = self.next;

                self.next = LLVMGetNextUse(out);

                Some(Use { useref: out, phantom: PhantomData })
            }
        }
    }
}

impl<'a> Value<'a> {
    pub fn null(ty: &Type<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMConstNull(ty.ty), phantom: PhantomData }
        }
    }

    pub fn null_ptr(ty: &Type<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMConstPointerNull(ty.ty), phantom: PhantomData }
        }
    }

    pub fn all_ones(ty: &Type<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMConstAllOnes(ty.ty), phantom: PhantomData }
        }
    }

    pub fn undef(ty: &Type<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetUndef(ty.ty), phantom: PhantomData }
        }
    }

    pub fn const_int(ty: &Type<'a>, val: u64, sext: bool) -> Value<'a> {
        unsafe {
            Value { value: LLVMConstInt(ty.ty, val, if sext { 1 } else { 0 }),
                    phantom: PhantomData }
        }
    }
/*
    pub fn const_int_arbitrary_prec(ty: &Type<'a>, data: &[u64]) -> Value<'a> {
        unsafe {
            Value { value: LLVMConstInt(ty.ty, val, if sext { 1 } else { 0 }),
                    phantom: PhantomData }
        }
    }
*/
    pub fn const_real(ty: &Type<'a>, val: f64, sext: bool) -> Value<'a> {
        unsafe {
            Value { value: LLVMConstReal(ty.ty, val), phantom: PhantomData }
        }
    }

    pub fn type_of(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMTypeOf(self.value), phantom: PhantomData }
        }
    }

    pub fn name(&self) -> Option<Cow<'a, str>> {
        unsafe {
            let mut len = 0;
            let strbytes = LLVMGetValueName2(self.value, &mut len);

            if !strbytes.is_null() {
                let strslice = slice::from_raw_parts(strbytes as *const u8,
                                                     len);

                Some(String::from_utf8_lossy(strslice))
            } else {
                None
            }
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe {
            let vec: Vec<u8> = name.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            LLVMSetValueName2(self.value, cstr.unwrap().as_ptr(), len);
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

    pub fn operand(&self, idx: u32) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetOperand(self.value, idx),
                    phantom: PhantomData }
        }
    }

    pub fn operand_use(&self, idx: u32) -> Use<'a> {
        unsafe {
            Use { useref: LLVMGetOperandUse(self.value, idx),
                  phantom: PhantomData }
        }
    }

    pub fn num_operands(&self) -> usize {
        unsafe {
            LLVMGetNumOperands(self.value) as usize
        }
    }

    pub fn set_operand(&mut self, idx: u32, val: &Value<'a>) {
        unsafe {
            LLVMSetOperand(self.value, idx, val.value)
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
    /// Get an `Iterator` on `Use`s of this value.
    pub fn use_iter<'b: 'a>(&self) -> UseIter<'b> {
        unsafe {
            UseIter { next: LLVMGetFirstUse(self.value), phantom: PhantomData }
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
            let len = self.num_basic_blocks() as usize;
            let mut vec = Vec::with_capacity(len);

            LLVMGetBasicBlocks(self.value, vec.as_mut_slice().as_mut_ptr());

            let mut out = Vec::with_capacity(len);

            for i in 0..len {
                out.push(BasicBlock { block: vec[i], phantom: PhantomData })
            }

            out
        }
    }

    pub fn num_basic_blocks(&self) -> u32 {
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
