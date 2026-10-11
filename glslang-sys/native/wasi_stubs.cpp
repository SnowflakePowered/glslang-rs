// Stubs for the WASI preview1 imports that the wasi-libc linked into this library on
// `wasm32-unknown-unknown` would otherwise import from the host.
#include <stdint.h>

#define WASI_STUB(name) extern "C" __attribute__((weak)) uint32_t __imported_wasi_snapshot_preview1_##name

static const uint32_t ESUCCESS = 0;
static const uint32_t EBADF = 8;
static const uint32_t ENOENT = 44;
static const uint32_t ENOSYS = 52;
static const uint32_t ESPIPE = 70;

struct ciovec {
    uint32_t buf;
    uint32_t len;
};

WASI_STUB(environ_sizes_get)(uint32_t* count, uint32_t* buf_size) {
    *count = 0;
    *buf_size = 0;
    return ESUCCESS;
}

WASI_STUB(environ_get)(uint32_t*, uint8_t*) {
    return ESUCCESS;
}

WASI_STUB(clock_time_get)(uint32_t, uint64_t, uint64_t*) {
    return ENOSYS;
}

WASI_STUB(fd_close)(uint32_t) {
    return ESUCCESS;
}

WASI_STUB(fd_fdstat_get)(uint32_t, uint8_t*) {
    return EBADF;
}

WASI_STUB(fd_fdstat_set_flags)(uint32_t, uint32_t) {
    return EBADF;
}

// EBADF ends libc's enumeration of preopened directories.
WASI_STUB(fd_prestat_get)(uint32_t, uint8_t*) {
    return EBADF;
}

WASI_STUB(fd_prestat_dir_name)(uint32_t, uint8_t*, uint32_t) {
    return EBADF;
}

// Reads always hit end of file.
WASI_STUB(fd_read)(uint32_t, const uint8_t*, uint32_t, uint32_t* nread) {
    *nread = 0;
    return ESUCCESS;
}

WASI_STUB(fd_seek)(uint32_t, int64_t, uint32_t, uint64_t*) {
    return ESPIPE;
}

// Writes are discarded but reported as complete.
WASI_STUB(fd_write)(uint32_t, const ciovec* iovs, uint32_t iovs_len, uint32_t* nwritten) {
    uint32_t total = 0;
    for (uint32_t i = 0; i < iovs_len; i++) {
        total += iovs[i].len;
    }
    *nwritten = total;
    return ESUCCESS;
}

WASI_STUB(path_open)(uint32_t, uint32_t, const uint8_t*, uint32_t, uint32_t, uint64_t, uint64_t,
                     uint32_t, uint32_t*) {
    return ENOENT;
}

extern "C" __attribute__((weak, noreturn)) void __imported_wasi_snapshot_preview1_proc_exit(uint32_t) {
    __builtin_trap();
}
