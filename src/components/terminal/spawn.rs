use std::{collections::HashMap, io, num::ParseIntError, os::fd::AsRawFd, path::PathBuf, string::FromUtf8Error};

use super::terminal::SpawnArgs;
use ashpd::flatpak;
use async_std::stream::StreamExt;
use async_trait::async_trait;
use libc::FD_CLOEXEC;
use terms_toolbox as toolbox;
use thiserror::Error;
use tracing::warn;
use vte::{self, InputStreamExtManual};
use zbus::zvariant::Fd;

const FLATPAK_INFO: &str = "/.flatpak-info";
const TOOLBOX: &str = "terms-toolbox";

pub fn get_spawner() -> Box<dyn Spawner> {
    if PathBuf::from(FLATPAK_INFO).exists() {
        Box::new(FlatpakSpawner::new())
    } else {
        Box::new(NativeSpawner::new())
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum SpawnError {
    #[error("IO error")]
    IoError(#[from] io::Error),

    #[error("Ashpd error")]
    AshpdError(#[from] ashpd::Error),

    #[error("GLib error")]
    GLibError(#[from] glib::Error),

    #[error("From UTF8 error")]
    FromUtf8Error(#[from] FromUtf8Error),

    #[error("Toolbox error")]
    ToolboxError(#[from] toolbox::ToolboxError),

    #[error("ParseInt error")]
    ParseIntError(#[from] ParseIntError),

    #[error("Unknown error")]
    Unknown(String),
}

#[derive(Debug)]
pub struct NativeSpawner {}

#[derive(Debug)]
pub struct FlatpakSpawner {}

#[async_trait(?Send)]
pub trait Spawner: std::fmt::Debug {
    /// Get the preferred shell of the user
    async fn shell(&self) -> Option<String>;

    async fn env(&self);

    async fn spawn(&self, term: &vte::Terminal, args: SpawnArgs) -> Result<i32, SpawnError>;

    /// Determines if a child process is running in the terminal, and returns the pid
    async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, SpawnError>;

    async fn process_status(&self, pid: libc::pid_t) -> Result<String, SpawnError>;

    async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, SpawnError>;
}

#[async_trait(?Send)]
impl Spawner for NativeSpawner {
    async fn shell(&self) -> Option<String> {
        let env_shell = std::env::var("SHELL");
        if env_shell.is_ok() {
            return env_shell.ok();
        }

        warn!("Could not get user shell from env var {}", env_shell.unwrap_err());
        let uid = unsafe { libc::getuid() };

        match toolbox::user_shell_async(uid).await {
            Ok(shell) => Some(shell),
            Err(err) => {
                warn!("Could not get user shell {}", err);
                None
            },
        }
    }

    async fn env(&self) {
        todo!();
    }

    async fn spawn(&self, term: &vte::Terminal, args: SpawnArgs) -> Result<i32, SpawnError> {
        todo!();
    }

    async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, SpawnError> {
        let fd = pty.fd().as_raw_fd();
        Ok(toolbox::child_pid(fd)?)
    }

    async fn process_status(&self, pid: libc::pid_t) -> Result<String, SpawnError> {
        let stat = toolbox::process_status_async(pid).await?;
        Ok(stat)
    }

    async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, SpawnError> {
        let stat = toolbox::process_cmdline_async(pid).await?;
        Ok(stat)
    }
}

#[async_trait(?Send)]
impl Spawner for FlatpakSpawner {
    async fn shell(&self) -> Option<String> {
        let uid = unsafe { libc::getuid() };
        let out = match self.run_host_toolbox_command("passwd", Some(&uid), HashMap::new(), HashMap::new()).await {
            Ok(out) => out,
            Err(err) => {
                warn!("Could not get user shell {}", err);
                return None;
            },
        };

        let parts: Vec<&str> = out.split(":").collect();

        if parts.len() < 7 {
            warn!("Could not parse getent output: {}", out);
            return None;
        }

        Some(parts[6].trim().to_owned())
    }

    async fn env(&self) {
        todo!();
    }

    async fn spawn(&self, term: &vte::Terminal, args: SpawnArgs) -> Result<i32, SpawnError> {
        todo!();
    }

    async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, SpawnError> {
        let fds = HashMap::from([(3, Fd::from(pty.fd().as_raw_fd()))]);
        let out = self.run_host_toolbox_command("child-pid", None::<bool>, fds, HashMap::new()).await?;
        Ok(out.parse::<libc::pid_t>()?)
    }

    async fn process_status(&self, pid: libc::pid_t) -> Result<String, SpawnError> {
        let out = self
            .run_host_toolbox_command("process-status", Some(pid), HashMap::new(), HashMap::new())
            .await?;
        Ok(out)
    }

    async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, SpawnError> {
        let out = self
            .run_host_toolbox_command("process-cmdline", Some(pid), HashMap::new(), HashMap::new())
            .await?;
        Ok(out)
    }
}

impl NativeSpawner {
    pub fn new() -> Self {
        Self {}
    }
}

impl FlatpakSpawner {
    pub fn new() -> Self {
        Self {}
    }

    async fn host_root(&self) -> Result<PathBuf, SpawnError> {
        let contents = async_std::fs::read(&PathBuf::from(FLATPAK_INFO)).await?;
        let keyfile = glib::KeyFile::new();
        keyfile.load_from_bytes(&glib::Bytes::from(&contents), glib::KeyFileFlags::NONE)?;
        let host_root = keyfile.string("Instance", "app-path")?;
        Ok(PathBuf::from(host_root).join("/bin"))
    }

    /// A thin wrapper over sendHostCommand that asks the terms-toolbox for information
    /// about the host system.
    async fn run_host_toolbox_command(
        &self,
        command: &str,
        command_arg: Option<impl ToString>,
        mut fds: HashMap<u32, Fd>,
        envs: HashMap<&str, &str>,
    ) -> Result<String, SpawnError> {
        let host_root = self.host_root().await?;
        let toolbox_path = PathBuf::from(TOOLBOX);

        let mut argv = vec![toolbox_path, PathBuf::from(command)];
        if let Some(arg) = command_arg {
            let argp = PathBuf::from(arg.to_string());
            argv.push(argp);
        }
        let dev_proxy = flatpak::Development::new().await?;

        // This creates two fds, where we can write to one and read from the
        // other. We'll pass one fd to the HostCommand as stdout, which means
        // we'll be able to read what is HostCommand prints out from the other
        // fd we just opened.
        let (read_fd, write_fd) = glib::unix_open_pipe(FD_CLOEXEC)?;

        let mut spawn_exit = dev_proxy.receive_spawn_exited().await?;

        fds.insert(1, write_fd.into());
        let pid = dev_proxy
            .host_command(
                host_root,
                &argv,
                fds,
                envs,
                flatpak::HostCommandFlags::ClearEnv | flatpak::HostCommandFlags::WatchBus,
            )
            .await?;

        // this shouldn't take long
        // TODO: what if it, for some reason, _does_ take long
        let exit_status = loop {
            if let Some((child_pid, exit_status)) = spawn_exit.next().await {
                if child_pid == pid {
                    break exit_status;
                }
            }
        };

        // stream takes ownership of read_fd. No need to close it later
        let input_stream = unsafe { gio::UnixInputStream::take_fd(read_fd) };

        // TODO: is 1024 bytes not enough or way to much?
        let out = match input_stream.read_future(vec![0; 1024], glib::Priority::DEFAULT).await {
            Ok((buffer, size)) => String::from_utf8(buffer[0..size].to_vec()).map_err(|err| SpawnError::from(err)),
            Err((_buffer, err)) => Err(SpawnError::from(err)),
        }?;

        // make sure write fd is closed. We don't care about error
        let write_fd_close_ret = unsafe { libc::close(write_fd) };
        if write_fd_close_ret == -1 {
            let err = std::io::Error::last_os_error();
            warn!("Error occured while closing write fd {}", err);
        }

        if exit_status != 0 {
            Err(SpawnError::Unknown(out))
        } else {
            Ok(out)
        }
    }
}
