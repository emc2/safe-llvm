use llvm_sys::core::LLVMArrayType;
use llvm_sys::core::LLVMCountParamTypes;
use llvm_sys::core::LLVMCountStructElementTypes;
use llvm_sys::core::LLVMGetArrayLength;
use llvm_sys::core::LLVMContextCreate;
use llvm_sys::core::LLVMFunctionType;
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
use llvm_sys::core::LLVMStructGetTypeAtIndex;
use llvm_sys::core::LLVMTypeIsSized;
use llvm_sys::core::LLVMVectorType;
use llvm::Context;
use llvm::Type;
use std::borrow::Cow;
use std::ffi::CStr;
use std::marker::PhantomData;

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
    pub fn array_type(&self, count: u32) -> Type<'a> {
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
/*
    /// Create a function type.
    fn func_type(args: &'a [Type<'a>], ret: Type<'a>) -> Type<'a> {
        unsafe {
            Type { ty: LLVMFunctionType(ret.ty, args.as_mut_ptr(),
                                        args.len() as u32, 0),
                   phantom: PhantomData }
        }
    }

    /// Create a function type.
    fn func_type_vararg(args: &'a [Type<'a>], ret: Type<'a>) -> Type<'a> {
        unsafe {
            Type { ty: LLVMFunctionType(ret.ty, args.as_mut_ptr(),
                                        args.len() as u32, 1),
                   phantom: PhantomData }
        }
    }
     */
    /// Check if a function type has variable arguments.
    pub fn is_function_vararg(&self) -> bool {
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
    pub fn count_params(&self) -> u32 {
        unsafe {
            LLVMCountParamTypes(self.ty)
        }
    }

    /// Get a function's return type.
    pub fn count_struct_elems(&self) -> u32 {
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

    /// Get the param types for a function type.
    pub fn param_types(&self) -> Vec<Type<'a>> {
        unsafe {
            let len = self.count_params() as usize;
            let mut vec = Vec::with_capacity(len);

            LLVMGetParamTypes(self.ty, vec.as_mut_slice().as_mut_ptr());

            let mut out = Vec::with_capacity(len);

            for i in 0..len {
                out.push(Type { ty: vec[i], phantom: PhantomData })
            }

            out
        }
    }

    /// Create a pointer type with this type as the base type.
    pub fn ptr_type(&self) -> Type<'a> {
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
    pub fn ptr_type_with_addrspc(&self, addrspc: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMPointerType(self.ty, addrspc), phantom: PhantomData }
        }
    }

    /// Get the element types for a struct type.
    pub fn struct_elem_types(&self) -> Vec<Type<'a>> {
        unsafe {
            let len = self.count_params() as usize;
            let mut vec = Vec::with_capacity(len);

            LLVMGetStructElementTypes(self.ty, vec.as_mut_slice().as_mut_ptr());

            let mut out = Vec::with_capacity(len);

            for i in 0..len {
                out.push(Type { ty: vec[i], phantom: PhantomData })
            }

            out
        }
    }

    pub fn struct_type_at_idx(&self, idx: u32) -> Type<'a> {
        unsafe {
            Type { ty: LLVMStructGetTypeAtIndex(self.ty, idx),
                   phantom: PhantomData }
        }
    }

    /// Get the vector size.
    pub fn vector_size(&self) -> u32 {
        unsafe {
            LLVMGetVectorSize(self.ty)
        }
    }
}
