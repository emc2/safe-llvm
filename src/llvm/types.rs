use llvm_sys::core::LLVMArrayType;
use llvm_sys::core::LLVMCountParamTypes;
use llvm_sys::core::LLVMCountStructElementTypes;
use llvm_sys::core::LLVMGetArrayLength;
use llvm_sys::core::LLVMContextCreate;
use llvm_sys::core::LLVMFunctionType;
use llvm_sys::core::LLVMGetElementType;
use llvm_sys::core::LLVMGetTypeContext;
use llvm_sys::core::LLVMGetParamTypes;
use llvm_sys::core::LLVMGetPointerAddressSpace;
use llvm_sys::core::LLVMGetReturnType;
use llvm_sys::core::LLVMGetStructElementTypes;
use llvm_sys::core::LLVMGetVectorSize;
use llvm_sys::core::LLVMIsFunctionVarArg;
use llvm_sys::core::LLVMIsOpaqueStruct;
use llvm_sys::core::LLVMIsPackedStruct;
use llvm_sys::core::LLVMPointerType;
use llvm_sys::core::LLVMPrintTypeToString;
use llvm_sys::core::LLVMStructCreateNamed;
use llvm_sys::core::LLVMStructGetTypeAtIndex;
use llvm_sys::core::LLVMStructSetBody;
use llvm_sys::core::LLVMTypeIsSized;
use llvm_sys::core::LLVMVectorType;
use llvm::Context;
use llvm::Type;
use std::borrow::Cow;
use std::ffi::CStr;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::marker::PhantomData;
use std::mem;
use std::ptr::null_mut;

impl<'a> Type<'a> {
    /// Check if this type has a static size.
    pub fn is_sized(&self) -> bool {
        unsafe {
            LLVMTypeIsSized(self.ty) != 0
        }
    }

    /// Get the `Context` in which this type exists.
    pub fn context(&self) -> Context<'a> {
        unsafe {
            Context { ctx: LLVMGetTypeContext(self.ty), dispose: false,
                      phantom: PhantomData }
        }
    }

    /// Get a `str` representation of this `Type`.
    pub fn to_str(&self) -> Cow<'a, str> {
        unsafe {
            CStr::from_ptr(LLVMPrintTypeToString(self.ty)).to_string_lossy()
        }
    }

    /// Get a `String` representation of this `Type`.
    pub fn to_string(&self) -> String {
        String::from(self.to_str())
    }

    /// Create an array type with this type as the elements.
    pub fn array(&self, count: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMArrayType(self.ty, count), phantom: PhantomData }
        }
    }

    /// Get the array size.
    pub fn array_len(&self) -> u32 {
        unsafe {
            LLVMGetArrayLength(self.ty)
        }
    }

    /// Get a function's return type.
    pub fn elem_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMGetElementType(self.ty), phantom: PhantomData }
        }
    }

    /// Create a function type.
    fn func_type(args: &[Type<'a>], ret: Type<'a>) -> Type<'a> {
        let mut vec = Vec::with_capacity(args.len());

        for arg in args {
            vec.push(arg.ty);
        }

        unsafe {
            let ptr = vec.as_mut_ptr();

            mem::forget(vec);

            Type { ty: LLVMFunctionType(ret.ty, ptr, args.len() as u32, 0),
                   phantom: PhantomData }
        }
    }

    /// Create a function type.
    fn func_type_vararg(args: &[Type<'a>], ret: Type<'a>) -> Type<'a> {
        let mut vec = Vec::with_capacity(args.len());

        for arg in args {
            vec.push(arg.ty);
        }

        unsafe {
            let ptr = vec.as_mut_ptr();

            mem::forget(vec);

            Type { ty: LLVMFunctionType(ret.ty, ptr, args.len() as u32, 1),
                   phantom: PhantomData }
        }
    }

    /// Check if a function type has variable arguments.
    pub fn is_vararg(&self) -> bool {
        unsafe {
            LLVMIsFunctionVarArg(self.ty) != 0
        }
    }

    /// Get a function's return type.
    pub fn return_type(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMGetReturnType(self.ty), phantom: PhantomData }
        }
    }

    /// Get a function's return type.
    pub fn num_params(&self) -> u32 {
        unsafe {
            LLVMCountParamTypes(self.ty)
        }
    }

    /// Get the param types for a function type.
    pub fn param_types(&self) -> Vec<Type<'a>> {
        unsafe {
            let len = self.num_params() as usize;
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(null_mut());
            }

            LLVMGetParamTypes(self.ty, vec.as_mut_slice().as_mut_ptr());

            let mut out = Vec::with_capacity(len);

            for i in 0..len {
                out.push(Type { ty: vec[i], phantom: PhantomData })
            }

            out
        }
    }

    /// Get the number of structure elements.
    pub fn num_struct_elems(&self) -> u32 {
        unsafe {
            LLVMCountStructElementTypes(self.ty)
        }
    }

    /// Check if this type is an opaque struct.
    pub fn is_opaque_struct(&self) -> bool {
        unsafe {
            LLVMIsOpaqueStruct(self.ty) != 0
        }
    }

    /// Check if this type is a packed struct.
    pub fn is_packed_struct(&self) -> bool {
        unsafe {
            LLVMIsPackedStruct(self.ty) != 0
        }
    }

    /// Get the element types for a struct type.
    pub fn struct_elem_types(&self) -> Vec<Type<'a>> {
        unsafe {
            let len = self.num_struct_elems() as usize;
            let mut vec = Vec::with_capacity(len);

            for _ in 0..len {
                vec.push(null_mut());
            }

            LLVMGetStructElementTypes(self.ty, vec.as_mut_slice().as_mut_ptr());

            let mut out = Vec::with_capacity(len);

            for i in 0..len {
                out.push(Type { ty: vec[i], phantom: PhantomData })
            }

            out
        }
    }

    /// Set the body of a struct.
    pub fn struct_set_body(&mut self, elems: &[Type<'a>]) {
        let len = elems.len();
        let mut vec = Vec::with_capacity(len);

        for elem in elems {
            vec.push(elem.ty);
        }

        unsafe {
            let ptr = vec.as_mut_ptr();

            mem::forget(vec);

            LLVMStructSetBody(self.ty, ptr, len as u32, 0)
        }
    }

    /// Set the body of a struct.
    pub fn struct_set_body_packed(&mut self, elems: &[Type<'a>]) {
        let len = elems.len();
        let mut vec = Vec::with_capacity(len);

        for elem in elems {
            vec.push(elem.ty);
        }

        unsafe {
            let ptr = vec.as_mut_ptr();

            mem::forget(vec);

            LLVMStructSetBody(self.ty, ptr, len as u32, 1)
        }
    }

    pub fn struct_type_at_idx(&self, idx: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMStructGetTypeAtIndex(self.ty, idx),
                   phantom: PhantomData }
        }
    }

    /// Create a pointer type with this type as the base type.
    pub fn ptr(&self) -> Type<'a> {
        unsafe {
            Type { ty: LLVMPointerType(self.ty, 0), phantom: PhantomData }
        }
    }

    /// Get the pointer address space.
    pub fn ptr_addrspc(&self) -> u32 {
        unsafe {
            LLVMGetPointerAddressSpace(self.ty)
        }
    }

    /// Create a pointer type with this type as the base type.
    pub fn ptr_with_addrspc(&self, addrspc: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMPointerType(self.ty, addrspc), phantom: PhantomData }
        }
    }

    /// Create an array type with this type as the elements.
    pub fn vector(&self, count: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMVectorType(self.ty, count), phantom: PhantomData }
        }
    }

    /// Get the vector size.
    pub fn vector_size(&self) -> u32 {
        unsafe {
            LLVMGetVectorSize(self.ty)
        }
    }
}

impl<'a> Display for Type<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_str())
    }
}

impl<'a> Debug for Type<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
use llvm::LLVM;
#[cfg(test)]
use llvm::ContextOps;

#[test]
fn test_array_int8() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let elemty = ctx.int8_type();
            let arrty = elemty.array(8);

            assert_eq!(8, arrty.array_len());
            assert_eq!(elemty, arrty.elem_type());
            assert_eq!(true, arrty.is_sized());
        }, 0);
}

#[test]
fn test_vector_int8() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let elemty = ctx.int8_type();
            let vecty = elemty.vector(8);

            assert_eq!(8, vecty.vector_size());
            assert_eq!(elemty, vecty.elem_type());
            assert_eq!(true, vecty.is_sized());
        }, 0);
}

#[test]
fn test_ptr_int8() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let elemty = ctx.int8_type();
            let ptrty = elemty.ptr();

            assert_eq!(elemty, ptrty.elem_type());
            assert_eq!(true, ptrty.is_sized());
        }, 0);
}

#[test]
fn test_ptr_int8_with_addrspc() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let elemty = ctx.int8_type();
            let ptrty = elemty.ptr_with_addrspc(2);

            assert_eq!(2, ptrty.ptr_addrspc());
            assert_eq!(elemty, ptrty.elem_type());
            assert_eq!(true, ptrty.is_sized());
        }, 0);
}

#[test]
fn test_func() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let arg1ty = ctx.int8_type();
            let arg2ty = ctx.float_type();
            let retty = arg1ty.ptr();
            let params = [arg1ty, arg2ty];
            let functy = Type::func_type(&params, retty);

            assert_eq!(retty, functy.return_type());
            assert_eq!(2, functy.num_params());
            assert_eq!(vec![arg1ty, arg2ty], functy.param_types());
            assert_eq!(false, functy.is_sized());
            assert_eq!(false, functy.is_vararg());
        }, 0);
}

#[test]
fn test_vararg_func() {
    LLVM::with_llvm(
        |llvm, _x| {
            let ctx = llvm.ctx();
            let arg1ty = ctx.int8_type();
            let arg2ty = ctx.float_type();
            let retty = arg1ty.ptr();
            let params = [arg1ty, arg2ty];
            let functy = Type::func_type_vararg(&params, retty);

            assert_eq!(retty, functy.return_type());
            assert_eq!(2, functy.num_params());
            assert_eq!(vec![arg1ty, arg2ty], functy.param_types());
            assert_eq!(false, functy.is_sized());
            assert_eq!(true, functy.is_vararg());
        }, 0);
}
