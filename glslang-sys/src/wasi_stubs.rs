//! Stubs for the WASI preview1 imports that the wasi-libc linked into glslang on
//! `wasm32-unknown-unknown` would otherwise import from the host.

const ESUCCESS: u32 = 0;
const EBADF: u32 = 8;
const ENOENT: u32 = 44;
const ENOSYS: u32 = 52;
const ESPIPE: u32 = 70;

#[repr(C)]
pub struct Ciovec {
    buf: u32,
    len: u32,
}

#[export_name = "__imported_wasi_snapshot_preview1_environ_sizes_get"]
pub unsafe extern "C" fn environ_sizes_get(count: *mut u32, buf_size: *mut u32) -> u32 {
    *count = 0;
    *buf_size = 0;
    ESUCCESS
}

#[export_name = "__imported_wasi_snapshot_preview1_environ_get"]
pub unsafe extern "C" fn environ_get(_environ: *mut u32, _buf: *mut u8) -> u32 {
    ESUCCESS
}

#[export_name = "__imported_wasi_snapshot_preview1_clock_time_get"]
pub unsafe extern "C" fn clock_time_get(_id: u32, _precision: u64, _time: *mut u64) -> u32 {
    ENOSYS
}

#[export_name = "__imported_wasi_snapshot_preview1_fd_close"]
pub unsafe extern "C" fn fd_close(_fd: u32) -> u32 {
    ESUCCESS
}

#[export_name = "__imported_wasi_snapshot_preview1_fd_fdstat_get"]
pub unsafe extern "C" fn fd_fdstat_get(_fd: u32, _stat: *mut u8) -> u32 {
    EBADF
}

#[export_name = "__imported_wasi_snapshot_preview1_fd_fdstat_set_flags"]
pub unsafe extern "C" fn fd_fdstat_set_flags(_fd: u32, _flags: u32) -> u32 {
    EBADF
}

// EBADF ends libc's enumeration of preopened directories.
#[export_name = "__imported_wasi_snapshot_preview1_fd_prestat_get"]
pub unsafe extern "C" fn fd_prestat_get(_fd: u32, _prestat: *mut u8) -> u32 {
    EBADF
}

#[export_name = "__imported_wasi_snapshot_preview1_fd_prestat_dir_name"]
pub unsafe extern "C" fn fd_prestat_dir_name(_fd: u32, _path: *mut u8, _len: u32) -> u32 {
    EBADF
}

// Reads always hit end of file.
#[export_name = "__imported_wasi_snapshot_preview1_fd_read"]
pub unsafe extern "C" fn fd_read(
    _fd: u32,
    _iovs: *const u8,
    _iovs_len: u32,
    nread: *mut u32,
) -> u32 {
    *nread = 0;
    ESUCCESS
}

#[export_name = "__imported_wasi_snapshot_preview1_fd_seek"]
pub unsafe extern "C" fn fd_seek(_fd: u32, _offset: i64, _whence: u32, _new_offset: *mut u64) -> u32 {
    ESPIPE
}

// Writes are discarded but reported as complete.
#[export_name = "__imported_wasi_snapshot_preview1_fd_write"]
pub unsafe extern "C" fn fd_write(
    _fd: u32,
    iovs: *const Ciovec,
    iovs_len: u32,
    nwritten: *mut u32,
) -> u32 {
    let mut total = 0u32;
    for i in 0..iovs_len as usize {
        total = total.wrapping_add((*iovs.add(i)).len);
    }
    *nwritten = total;
    ESUCCESS
}

#[export_name = "__imported_wasi_snapshot_preview1_path_open"]
pub unsafe extern "C" fn path_open(
    _fd: u32,
    _dirflags: u32,
    _path: *const u8,
    _path_len: u32,
    _oflags: u32,
    _rights_base: u64,
    _rights_inheriting: u64,
    _fdflags: u32,
    _opened_fd: *mut u32,
) -> u32 {
    ENOENT
}

#[export_name = "__imported_wasi_snapshot_preview1_proc_exit"]
pub extern "C" fn proc_exit(_code: u32) -> ! {
    core::arch::wasm32::unreachable()
}
