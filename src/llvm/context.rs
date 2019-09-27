use llvm_sys::core::LLVMCreateBuilderInContext;
use llvm_sys::core::LLVMInt1TypeInContext;
use llvm_sys::core::LLVMInt8TypeInContext;
use llvm_sys::core::LLVMInt16TypeInContext;
use llvm_sys::core::LLVMInt32TypeInContext;
use llvm_sys::core::LLVMInt64TypeInContext;
use llvm_sys::core::LLVMInt128TypeInContext;
use llvm_sys::core::LLVMIntTypeInContext;
use llvm_sys::core::LLVMHalfTypeInContext;
use llvm_sys::core::LLVMFloatTypeInContext;
use llvm_sys::core::LLVMDoubleTypeInContext;
use llvm_sys::core::LLVMFP128TypeInContext;
use llvm_sys::core::LLVMX86FP80TypeInContext;
use llvm_sys::core::LLVMPPCFP128TypeInContext;
use llvm_sys::core::LLVMVoidTypeInContext;
use llvm_sys::core::LLVMLabelTypeInContext;
use llvm_sys::core::LLVMX86MMXTypeInContext;
use llvm_sys::core::LLVMTokenTypeInContext;
use llvm_sys::core::LLVMMetadataTypeInContext;
use llvm::Builder;
use llvm::Context;
use llvm::ContextOps;
use llvm::MDKindID;
use llvm::Module;
use llvm::Type;
use llvm_sys::core::LLVMContextDispose;
use std::marker::PhantomData;
use std::ops::Drop;

impl<'a> Context<'a> {
    pub fn token_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMTokenTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    pub fn metadata_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMMetadataTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }
}

impl<'a> ContextOps<'a> for Context<'a> {
    /*
    fn get_md_kind_id(&mut self, name: &str) -> MDKindID<'a> {

    }

    fn module_create(&mut self, name: &str) -> Module<'a> {

    }
     */
    fn builder(&mut self) -> Builder<'a> {
        unsafe {
            Builder { builder: LLVMCreateBuilderInContext(self.ctx),
                      phantom: PhantomData }
        }
    }

    fn int1_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt1TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int8_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt8TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int16_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt16TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int32_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt32TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int64_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt64TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int128_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt128TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int_type(&self, len: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMIntTypeInContext(self.ctx, len),
                   phantom: PhantomData }
        }
    }

    fn half_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMHalfTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn float_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMFloatTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn double_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMDoubleTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn f128_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMFP128TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn x86_fp80_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMX86FP80TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn ppc_fp128_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMPPCFP128TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn void_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMVoidTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn label_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMLabelTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn x86_mmx_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMX86MMXTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }
}

impl<'a> Drop for Context<'a> {
    fn drop(&mut self) {
        unsafe {
            if self.dispose {
                LLVMContextDispose(self.ctx)
            }
        }
    }
}
