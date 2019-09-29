use llvm_sys::core::LLVMContextCreate;
use llvm_sys::core::LLVMCreateBuilder;
use llvm_sys::core::LLVMCreatePassManager;
use llvm_sys::core::LLVMCreateMessage;
use llvm_sys::core::LLVMDisposeBuilder;
use llvm_sys::core::LLVMDisposePassManager;
use llvm_sys::core::LLVMDisposeMessage;
use llvm_sys::core::LLVMDoubleType;
use llvm_sys::core::LLVMFP128Type;
use llvm_sys::core::LLVMFloatType;
use llvm_sys::core::LLVMGetMDKindID;
use llvm_sys::core::LLVMGetGlobalContext;
use llvm_sys::core::LLVMHalfType;
use llvm_sys::core::LLVMInt1Type;
use llvm_sys::core::LLVMInt8Type;
use llvm_sys::core::LLVMInt16Type;
use llvm_sys::core::LLVMInt32Type;
use llvm_sys::core::LLVMInt64Type;
use llvm_sys::core::LLVMInt128Type;
use llvm_sys::core::LLVMIntType;
use llvm_sys::core::LLVMStructType;
use llvm_sys::core::LLVMLabelType;
use llvm_sys::core::LLVMModuleCreateWithName;
use llvm_sys::core::LLVMPPCFP128Type;
use llvm_sys::core::LLVMShutdown;
use llvm_sys::core::LLVMStructCreateNamed;
use llvm_sys::core::LLVMVoidType;
use llvm_sys::core::LLVMX86FP80Type;
use llvm_sys::core::LLVMX86MMXType;
use llvm_sys::prelude::LLVMBuilderRef;
use llvm_sys::prelude::LLVMBasicBlockRef;
use llvm_sys::prelude::LLVMContextRef;
use llvm_sys::prelude::LLVMModuleRef;
use llvm_sys::prelude::LLVMPassManagerRef;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::marker::PhantomData;
use std::mem;
use std::ops::Drop;
use std::str;

pub mod builder;
pub mod basic_block;
pub mod context;
pub mod module;
pub mod types;
pub mod value;

/// Type representing basic LLVM functions.  This is provided as a
/// reference, provided by `with_llvm` in order to control use of the
/// `LLVMShutdown` function.
pub struct LLVM<'a>(PhantomData<&'a ()>);

pub struct Builder<'a> {
    builder: LLVMBuilderRef,
    phantom: PhantomData<&'a LLVMBuilderRef>
}

pub struct BasicBlock<'a> {
    block: LLVMBasicBlockRef,
    phantom: PhantomData<&'a LLVMBasicBlockRef>
}

pub struct Context<'a> {
    ctx: LLVMContextRef,
    dispose: bool,
    phantom: PhantomData<&'a LLVMContextRef>
}

pub struct PassManager<'a> {
    pass_mgr: LLVMPassManagerRef,
    phantom: PhantomData<&'a LLVMPassManagerRef>
}

pub struct Message<'a> {
    msg: *mut ::libc::c_char,
    phantom: PhantomData<&'a *mut ::libc::c_char>
}

pub struct MDKindID<'a> {
    id: ::libc::c_uint,
    phantom: PhantomData<&'a ::libc::c_uint>
}

pub struct Module<'a> {
    module: LLVMModuleRef,
    phantom: PhantomData<&'a LLVMModuleRef>
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Type<'a> {
    ty: LLVMTypeRef,
    phantom: PhantomData<&'a LLVMTypeRef>
}

pub struct Value<'a> {
    value: LLVMValueRef,
    phantom: PhantomData<&'a LLVMTypeRef>
}

pub trait ContextOps<'a> {
    fn md_kind_id(&mut self, name: &str) -> MDKindID<'a>;
    fn module(&mut self, name: &str) -> Module<'a>;
    fn named_struct(&mut self, name: &str) -> Type<'a>;

    /// Create an LLVM Builder object, for creating instructions.
    fn builder(&mut self) -> Builder<'a>;

    fn int1_type(&self) -> Type<'a>;
    fn int8_type(&self) -> Type<'a>;
    fn int16_type(&self) -> Type<'a>;
    fn int32_type(&self) -> Type<'a>;
    fn int64_type(&self) -> Type<'a>;
    fn int128_type(&self) -> Type<'a>;
    fn int_type(&self, len: u32) -> Type<'a>;
    fn half_type(&self) -> Type<'a>;
    fn float_type(&self) -> Type<'a>;
    fn double_type(&self) -> Type<'a>;
    fn f128_type(&self) -> Type<'a>;
    fn x86_fp80_type(&self) -> Type<'a>;
    fn ppc_fp128_type(&self) -> Type<'a>;
    fn void_type(&self) -> Type<'a>;
    fn label_type(&self) -> Type<'a>;
    fn x86_mmx_type(&self) -> Type<'a>;
    fn struct_type(&self, elems: &[Type<'a>]) -> Type<'a>;
    fn packed_struct_type(&self, elems: &[Type<'a>]) -> Type<'a>;
}

impl<'a> LLVM<'a> {
    /// Run `func` with an `LLVM` handle.
    pub fn with_llvm<F, T, U>(func: F, arg: T) -> U
        where F: FnOnce(&mut LLVM<'a>, T) -> U {
        let mut llvm = LLVM(PhantomData);

        func(&mut llvm, arg)
    }

    pub fn ctx<'b: 'a>(&mut self) -> Context<'b> {
        unsafe {
            Context { ctx: LLVMContextCreate(), dispose: true,
                      phantom: PhantomData }
        }
    }

    pub fn global_ctx(&mut self) -> Context<'a> {
        unsafe {
            Context { ctx: LLVMGetGlobalContext(), dispose: false,
                      phantom: PhantomData }
        }
    }

    pub fn create_msg<'b: 'a>(&mut self, s: &str) -> Message<'b> {
        unsafe {
            let cstr = CString::new(s);

            Message { msg: LLVMCreateMessage(cstr.unwrap().into_raw()),
                      phantom: PhantomData }
        }
    }

    pub fn create_pass_mgr<'b: 'a>(&self) -> PassManager<'b> {
        unsafe {
            PassManager { pass_mgr: LLVMCreatePassManager(),
                          phantom: PhantomData }
        }
    }
}

impl<'a, 'b: 'a> ContextOps<'b> for LLVM<'a> {
    fn md_kind_id(&mut self, name: &str) -> MDKindID<'b> {
        unsafe {
            let vec: Vec<u8> = name.into();
            let len = vec.len();
            let cstr = CString::new(vec);

            MDKindID { id: LLVMGetMDKindID(cstr.unwrap().as_ptr(), len as u32),
                       phantom: PhantomData }
        }
    }

    fn module(&mut self, name: &str) -> Module<'b> {
        unsafe {
            let cstr = CString::new(name);

            Module { module: LLVMModuleCreateWithName(cstr.unwrap().into_raw()),
                     phantom: PhantomData }
        }
    }

    fn named_struct(&mut self, name: &str) -> Type<'b> {
        unsafe {
            let cstr = CString::new(name);

            Type { ty: LLVMStructCreateNamed(self.global_ctx().ctx,
                                             cstr.unwrap().into_raw()),
                   phantom: PhantomData }
        }
    }

    fn builder(&mut self) -> Builder<'b> {
        unsafe {
            Builder { builder: LLVMCreateBuilder(), phantom: PhantomData }
        }
    }

    fn int1_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt1Type(), phantom: PhantomData }
        }
    }

    fn int8_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt8Type(), phantom: PhantomData }
        }
    }

    fn int16_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt16Type(), phantom: PhantomData }
        }
    }

    fn int32_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt32Type(), phantom: PhantomData }
        }
    }

    fn int64_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt64Type(), phantom: PhantomData }
        }
    }

    fn int128_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMInt128Type(), phantom: PhantomData }
        }
    }

    fn int_type(&self, len: u32) -> Type<'b> {
        unsafe {
            Type { ty: LLVMIntType(len), phantom: PhantomData }
        }
    }

    fn half_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMHalfType(), phantom: PhantomData }
        }
    }

    fn float_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMFloatType(), phantom: PhantomData }
        }
    }

    fn double_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMDoubleType(), phantom: PhantomData }
        }
    }

    fn f128_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMFP128Type(), phantom: PhantomData }
        }
    }

    fn x86_fp80_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMX86FP80Type(), phantom: PhantomData }
        }
    }

    fn ppc_fp128_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMPPCFP128Type(), phantom: PhantomData }
        }
    }

    fn void_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMVoidType(), phantom: PhantomData }
        }
    }

    fn label_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMLabelType(), phantom: PhantomData }
        }
    }

    fn x86_mmx_type(&self) -> Type<'b> {
        unsafe {
            Type { ty: LLVMX86MMXType(), phantom: PhantomData }
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

            Type { ty: LLVMStructType(ptr, len as u32, 0),
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

            Type { ty: LLVMStructType(ptr, len as u32, 1),
                   phantom: PhantomData }
        }
    }
}

impl<'a> Message<'a> {
    pub fn to_str(&self) -> Cow<'a, str> {
        unsafe {
            CStr::from_ptr(self.msg).to_string_lossy()
        }
    }

    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }
}

impl<'a> Display for Message<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_str())
    }
}

impl<'a> Drop for Builder<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeBuilder(self.builder)
        }
    }
}

/*
impl<'a> Drop for LLVM<'a> {
    fn drop(&mut self) {
        unsafe {
            //LLVMShutdown();
        }
    }
}
*/
impl<'a> Drop for PassManager<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposePassManager(self.pass_mgr)
        }
    }
}

impl<'a> Drop for Message<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMDisposeMessage(self.msg)
        }
    }
}

impl<'a> From<Message<'a>> for Cow<'a, str> {
    fn from(m: Message<'a>) -> Cow<'a, str> {
        m.to_str()
    }
}

#[test]
fn test_with_llvm() {
    let res = LLVM::with_llvm(|_llvm, x| x, 0);

    assert_eq!(0, res)
}

#[test]
fn test_ctx() {
    let res = LLVM::with_llvm(
        |llvm, x| {
            let _ctx = llvm.ctx();

            x
        }, 0);

    assert_eq!(0, res)
}

#[test]
fn test_global_ctx() {
    let res = LLVM::with_llvm(
        |llvm, x| {
            let _ctx = llvm.global_ctx();

            x
        }, 0);

    assert_eq!(0, res)
}

#[test]
fn test_message() {
    let res = LLVM::with_llvm(|llvm, _x| {
        let msg = llvm.create_msg("test");
        msg.to_string() }, 0);

    assert_eq!("test", res)
}

#[test]
fn test_int1() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int1_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int8() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int8_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int16() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int16_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int32() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int32_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int64() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int64_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int128() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int128_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_int() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.int_type(256);

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_half() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.half_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_float() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.float_type();

            assert_eq!(true, ty.is_sized());
        }, 0);
}

#[test]
fn test_double() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.double_type();

            assert_eq!(true, ty.is_sized());

        }, 0);
}

#[test]
fn test_f128() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.f128_type();

            assert_eq!(true, ty.is_sized());

        }, 0);
}

#[test]
fn test_x86_fp80() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.x86_fp80_type();

            assert_eq!(true, ty.is_sized());

        }, 0);
}

#[test]
fn test_ppc_fp128() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.ppc_fp128_type();

            assert_eq!(true, ty.is_sized());

        }, 0);
}

#[test]
fn test_void() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.void_type();

            assert_eq!(false, ty.is_sized());

        }, 0);
}

#[test]
fn test_label() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.label_type();

            assert_eq!(false, ty.is_sized());

        }, 0);
}

#[test]
fn test_x86_mmx() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ty = llvm.x86_mmx_type();

            assert_eq!(true, ty.is_sized());

        }, 0);
}
