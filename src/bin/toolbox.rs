use clap::{self, Parser, Subcommand};
use libc;
use std::{ffi::CString, process::ExitCode};

use terms_toolbox as toolbox;

// This is a simple program meant to be launched with flatpak-spawn --host to
// retrieve host information Flatpak'ed apps don't have access to. The original
// idea for this program came from
// https://github.com/gnunn1/tilix/blob/master/experimental/flatpak/tilix-flatpak-toolbox.c
// https://gitlab.gnome.org/raggesilver/blackbox/-/blob/e1862c558046783ef47ba6332734d77b25370e4d/toolbox/main.c

fn exec_getent_passwd(uid: libc::uid_t) -> ExitCode {
    let getent = CString::new("getent").expect("Could not create getent arg");
    let getent_arg = getent.clone();
    let passwd = CString::new("passwd").expect("Could not create passwd arg");
    let user = CString::new(uid.to_string()).expect("Could not create user arg");

    let ret = unsafe { libc::execlp(getent.as_ptr(), getent_arg.as_ptr(), passwd.as_ptr(), user.as_ptr(), 0) };

    if ret != -1 {
        unreachable!("uh oh, this is weird");
    }

    eprintln!("Could not exec getent {}", ret);
    return ExitCode::FAILURE;
}

fn get_shell(uid: libc::uid_t) -> ExitCode {
    match toolbox::user_shell(uid) {
        Ok(shell) => {
            println!("{}", shell);
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("error getting user shell {}", err);
            ExitCode::FAILURE
        },
    }
}

fn get_child_pid() -> ExitCode {
    // Caller should have saved terminal to fd 3.
    // This is 3 because we create an array of fds that will be passed to this
    // program via a Flatpak DBus call, and the vte pty we need is in the 4th
    // slot in that array.
    match toolbox::child_pid(3) {
        Ok(pid) => {
            println!("{}", pid);
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("error getting child pid {}", err);
            ExitCode::FAILURE
        },
    }
}

fn get_process_owner(pid: libc::pid_t) -> ExitCode {
    match toolbox::process_owner(pid) {
        Ok(pid) => {
            println!("{}", pid);
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("error getting child pid {}", err);
            ExitCode::FAILURE
        },
    }
}

fn get_process_status(pid: libc::pid_t) -> ExitCode {
    match toolbox::process_status(pid) {
        Ok(stat) => {
            println!("{}", stat);
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("error getting child status {}", err);
            ExitCode::FAILURE
        },
    }
}

fn get_process_cmdline(pid: libc::pid_t) -> ExitCode {
    match toolbox::process_cmdline(pid) {
        Ok(stat) => {
            println!("{}", stat);
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("error getting child cmdline {}", err);
            ExitCode::FAILURE
        },
    }
}

#[derive(Debug, Parser)]
#[command(about = "Terms companion for flatpak")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Passwd { uid: libc::uid_t },
    Shell { uid: libc::uid_t },
    ChildPid,
    ProcessOwner { pid: libc::pid_t },
    ProcessStatus { pid: libc::pid_t },
    ProcessCmdline { pid: libc::pid_t },
}

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command {
        Commands::Passwd { uid } => exec_getent_passwd(uid),
        Commands::Shell { uid } => get_shell(uid),
        Commands::ChildPid => get_child_pid(),
        Commands::ProcessOwner { pid } => get_process_owner(pid),
        Commands::ProcessStatus { pid } => get_process_status(pid),
        Commands::ProcessCmdline { pid } => get_process_cmdline(pid),
    }
}
