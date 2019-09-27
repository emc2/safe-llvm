use llvm_sys::core::LLVMBuildAggregateRet;
use llvm_sys::core::LLVMBuildBr;
//use llvm_sys::core::LLVMBuildCleanupRet;
//use llvm_sys::core::LLVMBuildCatchRet;
use llvm_sys::core::LLVMBuildCondBr;
use llvm_sys::core::LLVMBuildIndirectBr;
use llvm_sys::core::LLVMBuildInvoke;
use llvm_sys::core::LLVMBuildLandingPad;
use llvm_sys::core::LLVMBuildResume;
use llvm_sys::core::LLVMBuildRet;
use llvm_sys::core::LLVMBuildRetVoid;
use llvm_sys::core::LLVMBuildSwitch;
use llvm_sys::core::LLVMBuildUnreachable;
use llvm_sys::core::LLVMClearInsertionPosition;
use llvm_sys::core::LLVMGetCurrentDebugLocation;
use llvm_sys::core::LLVMGetInsertBlock;
use llvm_sys::core::LLVMInsertIntoBuilder;
use llvm_sys::core::LLVMInsertIntoBuilderWithName;
use llvm_sys::core::LLVMPositionBuilder;
use llvm_sys::core::LLVMPositionBuilderAtEnd;
use llvm_sys::core::LLVMPositionBuilderBefore;
use llvm_sys::core::LLVMSetCurrentDebugLocation;
use llvm_sys::core::LLVMSetInstDebugLocation;
use llvm::BasicBlock;
use llvm::Builder;
use llvm::Type;
use llvm::Value;
use std::ffi::CString;
use std::marker::PhantomData;

impl<'a> Builder<'a> {
    pub fn position(&self, block: BasicBlock<'a>, instr: Value<'a>) {
        unsafe {
            LLVMPositionBuilder(self.builder, block.block, instr.value);
        }
    }

    pub fn position_before(&self, instr: Value<'a>) {
        unsafe {
            LLVMPositionBuilderBefore(self.builder, instr.value);
        }
    }

    pub fn position_at_end(&self, block: BasicBlock<'a>) {
        unsafe {
            LLVMPositionBuilderAtEnd(self.builder, block.block);
        }
    }

    pub fn insert_block(&self) -> BasicBlock<'a> {
        unsafe {
            BasicBlock { block: LLVMGetInsertBlock(self.builder),
                         phantom: PhantomData }
        }
    }

    pub fn clear_insertion_pos(&self) {
        unsafe {
            LLVMClearInsertionPosition(self.builder);
        }
    }

    pub fn insert(&self, instr: Value<'a>) {
        unsafe {
            LLVMInsertIntoBuilder(self.builder, instr.value);
        }
    }

    pub fn insert_with_name(&self, instr: Value<'a>, name: &str) {
        unsafe {
            let cstr = CString::new(name).unwrap().into_raw();

            LLVMInsertIntoBuilderWithName(self.builder, instr.value, cstr);
        }
    }

    pub fn set_curr_debug_location(&self, loc: Value<'a>) {
        unsafe {
            LLVMSetCurrentDebugLocation(self.builder, loc.value);
        }
    }

    pub fn get_curr_debug_location(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMGetCurrentDebugLocation(self.builder),
                    phantom: PhantomData }
        }
    }

    pub fn set_inst_debug_location(&self, loc: Value<'a>) {
        unsafe {
            LLVMSetInstDebugLocation(self.builder, loc.value);
        }
    }

    pub fn build_ret_void(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildRetVoid(self.builder),
                    phantom: PhantomData }
        }
    }

    pub fn build_ret(&self, val: Value<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildRet(self.builder, val.value),
                    phantom: PhantomData }
        }
    }

    pub fn build_aggregate_ret(&self, vals: &[Value<'a>]) -> Value<'a> {
        unsafe {
            let len = vals.len();
            let mut arr = Vec::with_capacity(len);

            for i in 0..len {
                arr.push(vals[i].value);
            }

            let ptr = arr.as_mut_slice().as_mut_ptr();

            Value { value: LLVMBuildAggregateRet(self.builder, ptr, len as u32),
                    phantom: PhantomData }
        }
    }

    pub fn build_br(&self, target: BasicBlock<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildBr(self.builder, target.block),
                    phantom: PhantomData }
        }
    }

    pub fn build_cond_br(&self, cond: Value<'a>, t: BasicBlock<'a>,
                         f: BasicBlock<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildCondBr(self.builder, cond.value,
                                           t.block, f.block),
                    phantom: PhantomData }
        }
    }

    pub fn build_switch(&self, cond: Value<'a>, def: BasicBlock<'a>,
                        ncases: u32) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildSwitch(self.builder, cond.value,
                                           def.block, ncases),
                    phantom: PhantomData }
        }
    }

    pub fn build_indirect_br(&self, addr: Value<'a>, ncases: u32) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildIndirectBr(self.builder, addr.value,
                                               ncases),
                    phantom: PhantomData }
        }
    }

    pub fn build_invoke(&self, func: Value<'a>, args: &[Value<'a>],
                        then: BasicBlock<'a>, catch: BasicBlock<'a>,
                        name: &str) -> Value<'a> {
        unsafe {
            let len = args.len();
            let mut arr = Vec::with_capacity(len);

            for i in 0..len {
                arr.push(args[i].value);
            }

            let ptr = arr.as_mut_slice().as_mut_ptr();
            let cstr = CString::new(name).unwrap().into_raw();

            Value { value: LLVMBuildInvoke(self.builder, func.value, ptr,
                                           len as u32, then.block,
                                           catch.block, cstr),
                    phantom: PhantomData }
        }
    }

    pub fn build_unreachable(&self) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildUnreachable(self.builder),
                    phantom: PhantomData }
        }
    }

    pub fn build_resume(&self, val: Value<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildResume(self.builder, val.value),
                    phantom: PhantomData }
        }
    }

    pub fn build_landing_pad(&self, ty: Type<'a>, pers: Value<'a>,
                             nclauses: u32, name: &str) -> Value<'a> {
        unsafe {
            let cstr = CString::new(name).unwrap().into_raw();

            Value { value: LLVMBuildLandingPad(self.builder, ty.ty, pers.value,
                                               nclauses as u32, cstr),
                    phantom: PhantomData }
        }
    }
/*
    pub fn build_cleanup_ret(&self, val: Value<'a>,
                             cleanup: BasicBlock<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildCleanupRet(self.builder, val.value,
                                               cleanup.block),
                    phantom: PhantomData }
        }
    }

    pub fn build_catch_ret(&self, val: Value<'a>,
                           block: BasicBlock<'a>) -> Value<'a> {
        unsafe {
            Value { value: LLVMBuildCatchRet(self.builder, val.value,
                                               block.block),
                    phantom: PhantomData }
        }
    }
*/
}
