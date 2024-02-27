use std::{ffi::CString, io::Result, mem, os::fd::RawFd, path::Path};

pub fn map_error<T>(status: i32, item: T) -> Result<T> {
    if status == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(item)
    }
}

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString> {
    let path_str = path.as_ref().to_string_lossy().to_string();

    Ok(CString::new(path_str)?)
}

// pub fn execvp(
//     filename:  impl AsRef<CStr>,
//     args: &[impl AsRef<CStr>],
// ) -> Result<Infallible> {
//     let args_p = to_exec_array(args);

//     unsafe { libc::execvp(filename.as_ptr(), args_p.as_ptr()) };

//     Err(Errno::last())
// }

pub fn stat(path: impl AsRef<Path>) -> Result<libc::stat> {
    let mut dst = mem::MaybeUninit::uninit();

    let path_str = path_to_cstring(path)?;
    let res = unsafe { libc::stat(path_str.as_ptr(), dst.as_mut_ptr()) };

    map_error(res, unsafe { dst.assume_init() })
}

pub fn tcgetpgrp(fd: RawFd) -> Result<libc::pid_t> {
    let pid = unsafe { libc::tcgetpgrp(fd) };
    map_error(pid, pid)
}

pub fn grantpt(fd: RawFd) -> Result<()> {
    let status = unsafe { libc::grantpt(fd) };
    map_error(status, ())
}

pub fn unlockpt(fd: RawFd) -> Result<()> {
    let status = unsafe { libc::unlockpt(fd) };
    map_error(status, ())
}

pub fn ptsname_r(fd: RawFd) -> Result<String> {
    let mut name_buf = Vec::<libc::c_char>::with_capacity(64);
    let name_buf_ptr = name_buf.as_mut_ptr();
    let cname = unsafe {
        let cap = name_buf.capacity();
        if libc::ptsname_r(fd, name_buf_ptr, cap) != 0 {
            return Err(std::io::Error::last_os_error());
        }
        std::ffi::CStr::from_ptr(name_buf.as_ptr())
    };

    let name = cname.to_string_lossy().into_owned();
    Ok(name)
}

pub fn open(path: impl AsRef<Path>, flags: i32, mode: i32) -> Result<RawFd> {
    let path_str = path_to_cstring(path)?;

    let fd = unsafe { libc::open(path_str.as_ptr(), flags, mode) };
    map_error(fd, fd)
}

pub fn dup(oldfd: RawFd) -> Result<RawFd> {
    let fd = unsafe { libc::dup(oldfd) };
    map_error(fd, fd)
}
