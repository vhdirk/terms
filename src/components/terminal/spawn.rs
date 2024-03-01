use std::{
    cell::Cell,
    collections::HashMap,
    os::fd::BorrowedFd,
    os::fd::{AsRawFd, FromRawFd, RawFd},
    path::PathBuf,
    pin::Pin,
    rc::Rc,
    time::Duration,
};

use ashpd::flatpak;
use async_std::{io::ReadExt, stream::StreamExt};
use async_trait::async_trait;

use glib::clone;
use libc::FD_CLOEXEC;
use std::future::Future;
use terms_util::{libc_util, toolbox};

use tracing::*;
use vte::{self, TerminalExt, TerminalExtManual};

use crate::error::TermsError;

const FLATPAK_INFO: &str = "/.flatpak-info";
const TOOLBOX: &str = "terms-toolbox";

pub fn get_spawner() -> Box<dyn Spawner> {
    if PathBuf::from(FLATPAK_INFO).exists() {
        Box::new(FlatpakSpawner::new())
    } else {
        Box::new(NativeSpawner::new())
    }
}
pub trait PtyNewSyncFuture {
    async fn pty_new_sync_future(&self, flags: vte::PtyFlags) -> Result<vte::Pty, glib::Error>;
}

impl PtyNewSyncFuture for vte::Terminal {
    async fn pty_new_sync_future(&self, flags: vte::PtyFlags) -> Result<vte::Pty, glib::Error> {
        Box::pin(gio::GioFuture::new(self, move |term, cancellable, send| {
            let result = term.pty_new_sync(flags, Some(cancellable));
            send.resolve(result);
        }))
        .await
    }
}

pub trait TrimOption {
    fn trim_option(self) -> Self;
}

impl TrimOption for Option<String> {
    fn trim_option(self) -> Self {
        self.and_then(|content| {
            let trimmed = content.trim();
            if trimmed.is_empty() {
                return None;
            }

            Some(trimmed.to_string())
        })
    }
}

pub struct SpawnHandle {
    pub pid: libc::pid_t,
    pub child_exit: Pin<Box<dyn Future<Output = i32>>>,
}

#[derive(Debug)]
pub struct NativeSpawner {}

#[derive(Debug)]
pub struct FlatpakSpawner {}

#[async_trait(?Send)]
pub trait Spawner: std::fmt::Debug {
    /// Get the preferred shell of the user
    async fn shell(&self) -> Option<String>;

    async fn env(&self) -> Result<HashMap<String, String>, TermsError>;

    async fn working_dir(&self) -> Result<PathBuf, TermsError>;

    async fn spawn(
        &self,
        term: &vte::Terminal,
        flags: vte::PtyFlags,
        working_dir: PathBuf,
        argv: Vec<PathBuf>,
        envv: HashMap<String, String>,
        timeout: Duration,
    ) -> Result<SpawnHandle, TermsError>;

    /// Determines if a child process is running in the terminal, and returns the pid
    async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, TermsError>;

    async fn process_status(&self, pid: libc::pid_t) -> Result<String, TermsError>;

    async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, TermsError>;
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
            Ok(shell) => Some(shell).trim_option(),
            Err(err) => {
                warn!("Could not get user shell {}", err);
                None
            },
        }
    }

    async fn env(&self) -> Result<HashMap<String, String>, TermsError> {
        Ok(HashMap::from_iter(std::env::vars()))
    }

    async fn working_dir(&self) -> Result<PathBuf, TermsError> {
        match std::env::current_dir() {
            Ok(current_dir) => Ok(current_dir),
            Err(err) => {
                error!("Could not use current dir {}", err);
                Ok(glib::home_dir())
            },
        }
    }

    async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, TermsError> {
        let fd = pty.fd().as_raw_fd();
        Ok(libc_util::tcgetpgrp(fd)?)
    }

    async fn process_status(&self, pid: libc::pid_t) -> Result<String, TermsError> {
        let stat = toolbox::process_status_async(pid).await?;
        Ok(stat)
    }

    async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, TermsError> {
        let stat = toolbox::process_cmdline_async(pid).await?;
        Ok(stat)
    }

    async fn spawn(
        &self,
        term: &vte::Terminal,
        flags: vte::PtyFlags,
        working_dir: PathBuf,
        argv: Vec<PathBuf>,
        envv: HashMap<String, String>,
        timeout: Duration,
    ) -> Result<SpawnHandle, TermsError> {
        let args: Vec<&str> = argv.iter().map(|path| path.to_str().unwrap_or_default()).collect();
        let env_list: Vec<String> = envv.iter().map(|(key, value)| format!("{}={}", key, value)).collect();
        let envs: Vec<&str> = env_list.iter().map(|value| value.as_str()).collect();

        let exit_handler = Box::pin(gio::GioFuture::new(term, move |term, _cancellable, send| {
            let send = Rc::new(Cell::new(Some(send)));
            let _exit_handler_id = term.connect_child_exited(clone!(@strong send => move |_, exit_status| {
                if let Some(send) = send.take() {
                    send.resolve(exit_status);
                }
            }));
        }));

        match term
            .spawn_future(
                flags,
                Some(&working_dir.to_string_lossy()),
                &args,
                &envs,
                glib::SpawnFlags::DEFAULT,
                || {},
                timeout.as_millis() as i32,
            )
            .await
        {
            Ok(pid) => Ok(SpawnHandle {
                pid: pid.0,
                child_exit: exit_handler,
            }),
            Err(err) => Err(TermsError::from(err)),
        }
    }
}

#[async_trait(?Send)]
impl Spawner for FlatpakSpawner {
    async fn shell(&self) -> Option<String> {
        let uid = unsafe { libc::getuid() };
        let shell = Self::run_host_toolbox_command("shell", Some(&uid), HashMap::new(), HashMap::new())
            .await
            .map_err(|err| error!("Could not get user shell {}", err))
            .ok();

        shell.trim_option()
    }

    async fn env(&self) -> Result<HashMap<String, String>, TermsError> {
        let out = Self::run_host_toolbox_command("env", None::<bool>, HashMap::new(), HashMap::new()).await?;

        let deser_map: serde_yaml::Mapping = serde_yaml::from_str(&out)?;
        let mut varmap = HashMap::new();
        for (key, value) in deser_map {
            if let (Some(key), Some(value)) = (key.as_str(), value.as_str()) {
                varmap.insert(key.to_string(), value.to_string());
            }
        }

        Ok(varmap)
    }

    async fn working_dir(&self) -> Result<PathBuf, TermsError> {
        let out = Self::run_host_toolbox_command("home-directory", None::<bool>, HashMap::new(), HashMap::new()).await?;
        Ok(PathBuf::from(out.trim().to_string()))
    }

    async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, TermsError> {
        let fds = HashMap::from([(3, pty.fd())]);
        let out = Self::run_host_toolbox_command("child-pid", None::<bool>, fds, HashMap::new()).await?;
        Ok(out.parse::<libc::pid_t>()?)
    }

    async fn process_status(&self, pid: libc::pid_t) -> Result<String, TermsError> {
        let out = Self::run_host_toolbox_command("process-status", Some(pid), HashMap::new(), HashMap::new()).await?;
        Ok(out)
    }

    async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, TermsError> {
        let out = Self::run_host_toolbox_command("process-cmdline", Some(pid), HashMap::new(), HashMap::new()).await?;
        Ok(out)
    }

    async fn spawn(
        &self,
        term: &vte::Terminal,
        flags: vte::PtyFlags,
        working_dir: PathBuf,
        argv: Vec<PathBuf>,
        envv: HashMap<String, String>,
        _timeout: Duration,
    ) -> Result<SpawnHandle, TermsError> {
        // Open a new PTY master
        let pty = term.pty_new_sync_future(flags | vte::PtyFlags::NO_CTTY).await.map_err(|err| {
            warn!("Failed to create pseudoterminal device {:?}", err);
            err
        })?;

        let pty_master_fd = pty.fd();

        // Allow a slave to be generated for it
        libc_util::grantpt(pty_master_fd.as_raw_fd()).map_err(|err| {
            warn!("Failed granting access to slave pseudoterminal device {:?}", err);
            err
        })?;

        libc_util::unlockpt(pty_master_fd.as_raw_fd()).map_err(|err| {
            warn!("Failed unlocking slave pseudoterminal device {:?}", err);
            err
        })?;

        // Get the name of the slave
        let slave_name = libc_util::ptsname_r(pty_master_fd.as_raw_fd()).map_err(|err| {
            warn!("Failed to get slave pseudoterminal device name {:?}", err);
            err
        })?;

        let mut pty_slaves = HashMap::<u32, BorrowedFd>::new();

        let slave_fd: RawFd = libc_util::open(PathBuf::from(&slave_name), libc::O_RDWR | libc::O_CLOEXEC, 0).map_err(|err| {
            warn!("Failed opening slave pseudoterminal device {:?}", err);
            err
        })?;
        pty_slaves.insert(0, unsafe { BorrowedFd::borrow_raw(slave_fd) });

        // TODO: what's the point of these extra slave pty's?
        for i in 1..3 {
            let dup = libc_util::dup(slave_fd).map_err(|err| {
                warn!("Failed to duplicate slave pseudoterminal device {:?}", err);
                err
            })?;
            pty_slaves.insert(i, unsafe { BorrowedFd::borrow_raw(dup) });
        }

        let dev_proxy = flatpak::Development::new().await?;
        let mut spawn_exit = dev_proxy.receive_spawn_exited().await?;

        let envs: HashMap<&str, &str> = envv.iter().map(|(key, value)| (key.as_str(), value.as_str())).collect();

        info!(
            "Spawning pty command: cwd: {:?} argv: {:?}, fds: {:?}, envs:{:?}",
            &working_dir, &argv, &pty_slaves, &envs
        );

        let pid = dev_proxy
            .host_command(working_dir, &argv, pty_slaves, envs, flatpak::HostCommandFlags::WatchBus.into())
            .await?;

        let exit_status = async move {
            loop {
                if let Some((child_pid, exit_status)) = spawn_exit.next().await {
                    info!("got exit for child pid: {}: {}", child_pid, exit_status);
                    if child_pid == pid {
                        break exit_status as i32;
                    }
                } else {
                    break 1;
                }
            }
        };

        term.set_pty(Some(&pty));

        Ok(SpawnHandle {
            pid: pid as libc::pid_t,
            child_exit: Box::pin(exit_status),
        })
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

    async fn host_root() -> Result<PathBuf, TermsError> {
        let contents = async_std::fs::read(&PathBuf::from(FLATPAK_INFO)).await?;
        let keyfile = glib::KeyFile::new();
        keyfile.load_from_bytes(&glib::Bytes::from(&contents), glib::KeyFileFlags::NONE)?;
        let host_root = keyfile.string("Instance", "app-path")?;
        Ok(PathBuf::from(host_root).join("bin"))
    }

    async fn toolbox_path() -> Result<PathBuf, TermsError> {
        // first try if toolbox is found as sibling
        // if let Some(toolbox) = std::env::current_exe()
        //     .ok()
        //     .as_ref()
        //     .and_then(|current_exe| current_exe.parent())
        //     .map(|parent_dir| parent_dir.join(TOOLBOX))
        //     .filter(|toolbox| toolbox.exists())
        // {
        //     return Ok(toolbox);
        // }

        Ok(Self::host_root().await?.join(TOOLBOX))
    }

    /// A thin wrapper over sendHostCommand that asks the terms-toolbox for information
    /// about the host system.
    async fn run_host_toolbox_command<'fd>(
        command: &str,
        command_arg: Option<impl ToString>,
        mut fds: HashMap<u32, BorrowedFd<'fd>>,
        envs: HashMap<&str, &str>,
    ) -> Result<String, TermsError> {
        let toolbox_path = Self::toolbox_path().await?;

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
        let (read_fd, write_fd): (RawFd, RawFd) = glib::unix_open_pipe(FD_CLOEXEC)?;

        let mut spawn_exit = dev_proxy.receive_spawn_exited().await?;

        fds.insert(1, unsafe { BorrowedFd::borrow_raw(write_fd) });

        info!("Spawning toolbox command: argv: {:?}, fds: {:?}, envs:{:?}", &argv, &fds, &envs);
        let pid = dev_proxy
            .host_command("/", &argv, fds, envs, flatpak::HostCommandFlags::WatchBus.into())
            .await?;
        info!("Spawned toolbox command with pid: {}", pid);

        // this shouldn't take long
        // TODO: what if it, for some reason, _does_ take long
        let exit_status = loop {
            if let Some((child_pid, exit_status)) = spawn_exit.next().await {
                info!("got exit for child pid: {}: {}", child_pid, exit_status);
                if child_pid == pid {
                    break exit_status;
                }
            } else {
                break 1;
            }
        };

        // make sure write fd is closed. We don't care about error
        unsafe { async_std::fs::File::from_raw_fd(write_fd) };

        info!("Toolbox command exited with status: {}", exit_status);
        if exit_status != 0 {
            // TODO: can we read from stderr?
            return Err(TermsError::Unknown(format!("Toolbox command exited with status {}", exit_status)));
        }

        let mut stdout_read = unsafe { async_std::fs::File::from_raw_fd(read_fd) };

        let mut out = String::new();
        let _ = stdout_read.read_to_string(&mut out).await?;
        out = out.trim().to_string();
        info!("Toolbox command returned output: {}", out);

        if exit_status != 0 {
            Err(TermsError::Unknown(out))
        } else {
            Ok(out)
        }
    }
}
