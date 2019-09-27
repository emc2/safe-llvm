use llvm_sys::core::LLVMContextCreate;
use llvm_sys::core::LLVMCreateBuilder;
use llvm_sys::core::LLVMCreateMessage;
use llvm_sys::core::LLVMDisposeBuilder;
use llvm_sys::core::LLVMDisposeMessage;
use llvm_sys::core::LLVMDoubleType;
use llvm_sys::core::LLVMFP128Type;
use llvm_sys::core::LLVMFloatType;
use llvm_sys::core::LLVMGetGlobalContext;
use llvm_sys::core::LLVMHalfType;
use llvm_sys::core::LLVMInt1Type;
use llvm_sys::core::LLVMInt8Type;
use llvm_sys::core::LLVMInt16Type;
use llvm_sys::core::LLVMInt32Type;
use llvm_sys::core::LLVMInt64Type;
use llvm_sys::core::LLVMInt128Type;
use llvm_sys::core::LLVMIntType;
use llvm_sys::core::LLVMLabelType;
use llvm_sys::core::LLVMPPCFP128Type;
use llvm_sys::core::LLVMShutdown;
use llvm_sys::core::LLVMVoidType;
use llvm_sys::core::LLVMX86FP80Type;
use llvm_sys::core::LLVMX86MMXType;
use llvm_sys::prelude::LLVMBuilderRef;
use llvm_sys::prelude::LLVMBasicBlockRef;
use llvm_sys::prelude::LLVMContextRef;
use llvm_sys::prelude::LLVMModuleRef;
use llvm_sys::prelude::LLVMTypeRef;
use llvm_sys::prelude::LLVMValueRef;
use std::borrow::Cow;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::marker::PhantomData;
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

pub struct Message<'a> {
    msg: *mut ::libc::c_char,
    phantom: PhantomData<&'a *mut ::libc::c_char>
}

pub struct MDKindID<'a> {
    msg: ::libc::c_uint,
    phantom: PhantomData<&'a ::libc::c_uint>
}

pub struct Module<'a> {
    msg: LLVMModuleRef,
    phantom: PhantomData<&'a LLVMModuleRef>
}

pub struct Type<'a> {
    ty: LLVMTypeRef,
    phantom: PhantomData<&'a LLVMTypeRef>
}

pub struct Value<'a> {
    value: LLVMValueRef,
    phantom: PhantomData<&'a LLVMTypeRef>
}

pub trait ContextOps<'a> {
//    fn get_md_kind_id(&mut self, name: &str) -> MDKindID<'a>;
    //    fn module_create(&mut self, name: &str) -> Module<'a>;

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
}

impl<'a> LLVM<'a> {
    /// Run `func` with an `LLVM` handle.
    pub fn with_llvm<T, U>(func: fn(&mut LLVM<'a>, T) -> U, arg: T) -> U {
        let mut llvm = LLVM(PhantomData);

        func(&mut llvm, arg)
    }

    pub fn ctx<'b : 'a>(&mut self) -> Context<'b> {
        unsafe {
            Context { ctx: LLVMContextCreate(), dispose: true,
                      phantom: PhantomData }
        }
    }

    pub fn global_ctx<'b : 'a>(&mut self) -> Context<'b> {
        unsafe {
            Context { ctx: LLVMGetGlobalContext(), dispose: false,
                      phantom: PhantomData }
        }
    }

    pub fn create_msg<'b : 'a>(&mut self, s: &str) -> Message<'b> {
        unsafe {
            let cstr = CString::new(s);

            Message { msg: LLVMCreateMessage(cstr.unwrap().into_raw()),
                      phantom: PhantomData }
        }
    }
}

impl<'a> ContextOps<'a> for LLVM<'a> {
    /*
    fn get_md_kind_id(&mut self, name: &str) -> MDKindID<'a> {

    }

    fn module_create(&mut self, name: &str) -> Module<'a> {

    }
     */
    fn builder(&mut self) -> Builder<'a> {
        unsafe {
            Builder { builder: LLVMCreateBuilder(), phantom: PhantomData }
        }
    }

    fn int1_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt1Type(), phantom: PhantomData }
        }
    }

    fn int8_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt8Type(), phantom: PhantomData }
        }
    }

    fn int16_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt16Type(), phantom: PhantomData }
        }
    }

    fn int32_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt32Type(), phantom: PhantomData }
        }
    }

    fn int64_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt64Type(), phantom: PhantomData }
        }
    }

    fn int128_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMInt128Type(), phantom: PhantomData }
        }
    }

    fn int_type(&self, len: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMIntType(len), phantom: PhantomData }
        }
    }

    fn half_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMHalfType(), phantom: PhantomData }
        }
    }

    fn float_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMFloatType(), phantom: PhantomData }
        }
    }

    fn double_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMDoubleType(), phantom: PhantomData }
        }
    }

    fn f128_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMFP128Type(), phantom: PhantomData }
        }
    }

    fn x86_fp80_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMX86FP80Type(), phantom: PhantomData }
        }
    }

    fn ppc_fp128_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMPPCFP128Type(), phantom: PhantomData }
        }
    }

    fn void_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMVoidType(), phantom: PhantomData }
        }
    }

    fn label_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMLabelType(), phantom: PhantomData }
        }
    }

    fn x86_mmx_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMX86MMXType(), phantom: PhantomData }
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

impl<'a> Drop for LLVM<'a> {
    fn drop(&mut self) {
        unsafe {
            LLVMShutdown();
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
    let res = LLVM::with_llvm(|llvm, x| { let _ctx = llvm.ctx(); x }, 0);

    assert_eq!(0, res)
}

#[test]
fn test_global_ctx() {
    let res = LLVM::with_llvm(|llvm, x| { let _ctx = llvm.global_ctx(); x }, 0);

    assert_eq!(0, res)
}

#[test]
fn test_message() {
    let res = LLVM::with_llvm(|llvm, _x| {
        let msg = llvm.create_msg("test");
        msg.to_string() },
                              0);

    assert_eq!("test", res)
}
