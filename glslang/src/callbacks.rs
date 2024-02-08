use glslang_sys as sys;
use std::ffi::{CStr, CString};

/// The type of include.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum IncludeType {
    /// A system include, using angle brackets, i.e. `<header.h>`
    System,
    /// A relative local include, using quotes, i.e. `"header.h"`
    Local,
}

/// The result of a resolved include.
pub struct IncludeResult {
    /// The name of the header.
    pub name: String,
    /// The contents of the header file.
    pub data: String,
}

unsafe fn _glslang_rs_call_func(
    ctx: *mut ::core::ffi::c_void,
    ty: IncludeType,
    header_name: *const ::core::ffi::c_char,
    includer_name: *const ::core::ffi::c_char,
    include_depth: usize,
) -> *mut sys::glsl_include_result_t {
    let Ok(s) = std::panic::catch_unwind(|| unsafe {
        let Some(callback) = ctx.cast_const().cast::<IncludeCallback>().as_ref() else {
            return core::ptr::null_mut();
        };

        let header_name = CStr::from_ptr(header_name);
        let includer_name = CStr::from_ptr(includer_name);

        let (Ok(header_name), Ok(includer_name)) = (header_name.to_str(), includer_name.to_str())
        else {
            return core::ptr::null_mut();
        };

        let Some(result) = callback(ty, header_name, includer_name, include_depth) else {
            return core::ptr::null_mut();
        };

        let header_data_len = result.data.len();

        // SAFETY: String has no internal nulls.
        let header_name_leaked = CString::new(result.name).unwrap().into_raw();
        let header_data_leaked = CString::new(result.data).unwrap().into_raw();

        return Box::into_raw(Box::new(sys::glsl_include_result_t {
            header_name: header_name_leaked,
            header_data: header_data_leaked,
            header_length: header_data_len,
        }));
    }) else {
        return core::ptr::null_mut();
    };

    s
}

pub(crate) unsafe extern "C" fn _glslang_rs_sys_func(
    ctx: *mut ::core::ffi::c_void,
    header_name: *const ::core::ffi::c_char,
    includer_name: *const ::core::ffi::c_char,
    include_depth: usize,
) -> *mut sys::glsl_include_result_t {
    _glslang_rs_call_func(
        ctx,
        IncludeType::System,
        header_name,
        includer_name,
        include_depth,
    )
}

pub(crate) unsafe extern "C" fn _glslang_rs_local_func(
    ctx: *mut ::core::ffi::c_void,
    header_name: *const ::core::ffi::c_char,
    includer_name: *const ::core::ffi::c_char,
    include_depth: usize,
) -> *mut sys::glsl_include_result_t {
    _glslang_rs_call_func(
        ctx,
        IncludeType::Local,
        header_name,
        includer_name,
        include_depth,
    )
}

pub(crate) unsafe extern "C" fn _glslang_rs_drop_result(
    _ctx: *mut ::std::os::raw::c_void,
    result: *mut sys::glsl_include_result_t,
) -> ::core::ffi::c_int {
    let boxed = Box::from_raw(result);
    let header_name = CString::from_raw(boxed.header_name.cast_mut());
    let header_data = CString::from_raw(boxed.header_data.cast_mut());

    drop(header_data);
    drop(header_name);
    drop(boxed);
    return 0;
}

///  A callback to handle include files.
///
/// `fn (IncludeType, header_name, includer_name, include_depth)`
///
/// If the inclusion fails, return None.
pub type IncludeCallback = fn(IncludeType, &str, &str, usize) -> Option<IncludeResult>;
