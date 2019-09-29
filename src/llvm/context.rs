use llvm_sys::core::LLVMCreateBuilderInContext;
use llvm_sys::core::LLVMGetMDKindIDInContext;
use llvm_sys::core::LLVMModuleCreateWithNameInContext;
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
use llvm_sys::core::LLVMStructTypeInContext;
use llvm_sys::core::LLVMStructCreateNamed;
use llvm::Builder;
use llvm::Context;
use llvm::ContextOps;
use llvm::MDKindID;
use llvm::Module;
use llvm::Type;
use llvm_sys::core::LLVMContextDispose;
use std::ffi::CString;
use std::marker::PhantomData;
use std::ops::Drop;
use std::mem;

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

impl<'a, 'b: 'a> ContextOps<'b> for Context<'a> {
    fn md_kind_id(&mut self, name: &str) -> MDKindID<'b> {
        unsafe {
            let vec: Vec<u8> = name.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            MDKindID { id: LLVMGetMDKindIDInContext(self.ctx,
                                                    cstr.unwrap().as_ptr(),
                                                    len as u32),
                     phantom: PhantomData }
        }
    }

    fn module(&mut self, name: &str) -> Module<'b> {
        unsafe {
            let cstr = CString::new(name);

            Module { module: LLVMModuleCreateWithNameInContext(cstr.unwrap()
                                                               .into_raw(),
                                                               self.ctx),
                     phantom: PhantomData }
        }
    }

    fn named_struct(&mut self, name: &str) -> Type<'b> {
        unsafe {
            let vec: Vec<u8> = name.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            Type { ty: LLVMStructCreateNamed(self.ctx,
                                             cstr.unwrap().into_raw()),
                   phantom: PhantomData }
        }
    }

    fn builder(&mut self) -> Builder<'b> {
        unsafe {
            Builder { builder: LLVMCreateBuilderInContext(self.ctx),
                      phantom: PhantomData }
        }
    }

    fn int1_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt1TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int8_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt8TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int16_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt16TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int32_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt32TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int64_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt64TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int128_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt128TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn int_type(&self, len: u32) -> Type<'b> {
        unsafe {
            Type { ty: LLVMIntTypeInContext(self.ctx, len),
                   phantom: PhantomData }
        }
    }

    fn half_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMHalfTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn float_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMFloatTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn double_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMDoubleTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn f128_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMFP128TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn x86_fp80_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMX86FP80TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn ppc_fp128_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMPPCFP128TypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn void_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMVoidTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn label_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMLabelTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    fn x86_mmx_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMX86MMXTypeInContext(self.ctx),
                   phantom: PhantomData }
        }
    }

    /// Create a function type.
    fn struct_type(&self, elems: &[Type<'b>]) -> Type<'b> {
        let len = elems.len();
        let mut vec = Vec::with_capacity(len);

        for elem in elems {
            vec.push(elem.ty);
        }

        unsafe {
            let ptr = vec.as_mut_ptr();

            mem::forget(vec);

            Type { ty: LLVMStructTypeInContext(self.ctx, ptr, len as u32, 0),
                   phantom: PhantomData }
        }
    }

    /// Create a function type.
    fn packed_struct_type(&self, elems: &[Type<'b>]) -> Type<'b> {
        let len = elems.len();
        let mut vec = Vec::with_capacity(len);

        for elem in elems {
            vec.push(elem.ty);
        }

        unsafe {
            let ptr = vec.as_mut_ptr();

            mem::forget(vec);

            Type { ty: LLVMStructTypeInContext(self.ctx, ptr, len as u32, 1),
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

#[cfg(test)]
use llvm::LLVM;

#[test]
fn test_int1() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int1_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int8() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int8_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int16() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int16_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int32() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int32_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int64() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int64_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int128() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int128_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.int_type(256);

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_half() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.half_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_float() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.float_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_double() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.double_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_f128() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.f128_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_x86_fp80() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.x86_fp80_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_ppc_fp128() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.ppc_fp128_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_void() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.void_type();

            assert_eq!(false, ty.is_sized());
        }, 0);
}

#[test]
fn test_label() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.label_type();

            assert_eq!(false, ty.is_sized());
        }, 0);
}

#[test]
fn test_x86_mmx() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.x86_mmx_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_token() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.token_type();

            assert_eq!(false, ty.is_sized());
        }, 0);
}

#[test]
fn test_metadata() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let ty = ctx.metadata_type();

            assert_eq!(false, ty.is_sized());
        }, 0);
}

#[test]
fn test_struct() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let elem1ty = ctx.int8_type();
            let elem2ty = ctx.float_type();
            let elems = [elem1ty, elem2ty];
            let structty = ctx.struct_type(&elems);

            assert_eq!(2, structty.num_struct_elems());
            assert_eq!(vec![elem1ty, elem2ty], structty.struct_elem_types());
            assert_eq!(true, structty.is_sized());
            assert_eq!(false, structty.is_opaque_struct());
            assert_eq!(false, structty.is_packed_struct());
        }, 0);
}

#[test]
fn test_packed_struct() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let elem1ty = ctx.int8_type();
            let elem2ty = ctx.float_type();
            let elems = [elem1ty, elem2ty];
            let structty = ctx.packed_struct_type(&elems);

            println!("{}", structty);
            assert_eq!(2, structty.num_struct_elems());
            assert_eq!(vec![elem1ty, elem2ty], structty.struct_elem_types());
            assert_eq!(true, structty.is_sized());
            assert_eq!(false, structty.is_opaque_struct());
            assert_eq!(true, structty.is_packed_struct());
        }, 0);
}

#[test]
fn test_named_struct() {
    LLVM::with_llvm(
        |llvm, _x| {
            let mut ctx = llvm.ctx();
            let elem1ty = ctx.int8_type();
            let elem2ty = ctx.float_type();
            let elems = [elem1ty, elem2ty];
            let emptyelems: Vec<Type> = vec![];
            let mut structty = ctx.named_struct("test");

            assert_eq!(0, structty.num_struct_elems());
            assert_eq!(emptyelems, structty.struct_elem_types());
            assert_eq!(false, structty.is_sized());
            assert_eq!(true, structty.is_opaque_struct());
            assert_eq!(false, structty.is_packed_struct());

            structty.struct_set_body(&elems);
            assert_eq!(2, structty.num_struct_elems());
            assert_eq!(vec![elem1ty, elem2ty], structty.struct_elem_types());
            assert_eq!(true, structty.is_sized());
            assert_eq!(false, structty.is_opaque_struct());
            assert_eq!(false, structty.is_packed_struct());
        }, 0);
}

#[test]
fn test_named_packed_struct() {
    LLVM::with_llvm(
        |llvm, _x| {
            let mut ctx = llvm.ctx();
            let elem1ty = ctx.int8_type();
            let elem2ty = ctx.float_type();
            let elems = [elem1ty, elem2ty];
            let emptyelems: Vec<Type> = vec![];
            let mut structty = ctx.named_struct("test");

            assert_eq!(0, structty.num_struct_elems());
            assert_eq!(emptyelems, structty.struct_elem_types());
            assert_eq!(false, structty.is_sized());
            assert_eq!(true, structty.is_opaque_struct());
            assert_eq!(false, structty.is_packed_struct());

            structty.struct_set_body_packed(&elems);
            assert_eq!(2, structty.num_struct_elems());
            assert_eq!(vec![elem1ty, elem2ty], structty.struct_elem_types());
            assert_eq!(true, structty.is_sized());
            assert_eq!(false, structty.is_opaque_struct());
            assert_eq!(true, structty.is_packed_struct());
        }, 0);
}
