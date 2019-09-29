use llvm_sys::core::LLVMContextCreate;
use llvm_sys::core::LLVMCreateBuilder;
use llvm_sys::core::LLVMCreateMessage;
use llvm_sys::core::LLVMDisposeBuilder;
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
    fn struct_type(&self, elems: &[&Type<'a>]) -> Type<'a>;
    fn packed_struct_type(&self, elems: &[&Type<'a>]) -> Type<'a>;
}

impl<'a, 'b: 'a> LLVM<'a> {
    /// Run `func` with an `LLVM` handle.
    pub fn with_llvm<F, T, U>(func: F, arg: T) -> U
        where F: FnOnce(&mut LLVM<'a>, T) -> U {
        let mut llvm = LLVM(PhantomData);

        func(&mut llvm, arg)
    }

    pub fn ctx(&mut self) -> Context<'b> {
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

    pub fn create_msg(&mut self, s: &str) -> Message<'b> {
        unsafe {
            let cstr = CString::new(s);

            Message { msg: LLVMCreateMessage(cstr.unwrap().into_raw()),
                      phantom: PhantomData }
        }
    }
}

impl<'a, 'b> ContextOps<'b> for LLVM<'a> {
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
    fn struct_type(&self, elems: &[&Type<'b>]) -> Type<'b> {
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
    fn packed_struct_type(&self, elems: &[&Type<'b>]) -> Type<'b> {
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
        msg.to_string() }, 0);

    assert_eq!("test", res)
}
