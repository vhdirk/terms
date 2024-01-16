use std::{
    ffi::{CString, NulError},
    os::fd::{AsRawFd, BorrowedFd, RawFd},
    path::PathBuf,
};
use thiserror::Error;
use tracing::*;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum ToolboxError {
    #[error(transparent)]
    NulError(#[from] NulError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("Unknown error")]
    Unknown(String),
}

pub fn passwd_line_filter(uid: libc::uid_t) -> impl FnMut(&Result<String, std::io::Error>) -> bool {
    let uid = uid.to_string();
    move |line| {
        if let Ok(entry) = line {
            let fields: Vec<&str> = entry.split(':').collect();
            if fields.get(3) == Some(&uid.as_str()) {
                return true;
            }
        }
        false
    }
}

pub fn shell_from_passwd_line(passwd_line: &str) -> Result<String, ToolboxError> {
    let parts: Vec<&str> = passwd_line.split(":").collect();

    if parts.len() < 7 {
        return Err(ToolboxError::Unknown(format!("Could not parse user line from passwd line {}", { passwd_line })));
    }

    Ok(parts[6].trim().to_owned())
}

pub fn user_shell(uid: libc::uid_t) -> Result<String, ToolboxError> {
    use std::io::BufRead;
    let passwd_file = std::fs::File::open("/etc/passwd")?;

    let passwd_line = match std::io::BufReader::new(passwd_file).lines().find(passwd_line_filter(uid)) {
        Some(line) => line.map_err(ToolboxError::from),
        None => Err(ToolboxError::Unknown(format!("User {} not found in passwd file", { uid }))),
    }?;

    shell_from_passwd_line(&passwd_line)
}

pub async fn user_shell_async(uid: libc::uid_t) -> Result<String, ToolboxError> {
    use async_std::{io::prelude::BufReadExt, stream::StreamExt};
    let passwd_file = async_std::fs::File::open("/etc/passwd").await?;

    let passwd_line = match async_std::io::BufReader::new(passwd_file).lines().find(passwd_line_filter(uid)).await {
        Some(line) => line.map_err(ToolboxError::from),
        None => Err(ToolboxError::Unknown(format!("User {} not found in passwd file", { uid }))),
    }?;

    shell_from_passwd_line(&passwd_line)
}

pub fn process_libc_stat(pid: libc::pid_t) -> Result<libc::stat, ToolboxError> {
    let path = PathBuf::from(format!("/proc/{}/stat", pid));
    let stat = crate::libc_util::stat(&path)?;
    Ok(stat)
}

pub fn process_owner(pid: libc::pid_t) -> Result<libc::uid_t, ToolboxError> {
    let stat = process_libc_stat(pid)?;
    Ok(stat.st_uid)
}

fn read_file(file_path: &str) -> Result<String, ToolboxError> {
    let path = std::path::PathBuf::from(file_path);
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

async fn read_file_async(file_path: &str) -> Result<String, ToolboxError> {
    let path = async_std::path::PathBuf::from(file_path);
    let content = async_std::fs::read_to_string(path).await?;
    Ok(content)
}

pub fn process_status(pid: libc::pid_t) -> Result<String, ToolboxError> {
    read_file(&format!("/proc/{}/stat", pid))
}

pub async fn process_status_async(pid: libc::pid_t) -> Result<String, ToolboxError> {
    read_file_async(&format!("/proc/{}/stat", pid)).await
}

pub fn process_cmdline(pid: libc::pid_t) -> Result<String, ToolboxError> {
    read_file(&format!("/proc/{}/cmdline", pid))
}

pub async fn process_cmdline_async(pid: libc::pid_t) -> Result<String, ToolboxError> {
    read_file_async(&format!("/proc/{}/cmdline", pid)).await
}

pub fn working_dir() -> std::path::PathBuf {
    // get the current dir
    let current_dir = std::env::current_dir();

    if current_dir.is_ok() {
        return current_dir.unwrap();
    } else {
        error!("Could not use current dir {}", current_dir.unwrap_err());
    }

    glib::home_dir()
}
