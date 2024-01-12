use clap::{self, Parser, Subcommand};
use libc;
use std::{ffi::CString, process::ExitCode};

// This is a simple program meant to be launched with flatpak-spawn --host to
// retrieve host information Flatpak'ed apps don't have access to. The original
// idea for this program came from
// https://github.com/gnunn1/tilix/blob/master/experimental/flatpak/tilix-flatpak-toolbox.c
// https://gitlab.gnome.org/raggesilver/blackbox/-/blob/e1862c558046783ef47ba6332734d77b25370e4d/toolbox/main.c

fn get_passwd(user: String) -> ExitCode {
    let getent = CString::new("getent").expect("Could not create getent arg");
    let getent_arg = getent.clone();
    let passwd = CString::new("passwd").expect("Could not create passwd arg");
    let user = CString::new(user).expect("Could not create user arg");

    let ret = unsafe { libc::execlp(getent.as_ptr(), getent_arg.as_ptr(), passwd.as_ptr(), user.as_ptr(), 0) };

    if ret != -1 {
        unreachable!("uh oh, this is weird");
    }

    eprintln!("Could not exec getent {}", ret);
    return ExitCode::FAILURE;
}

fn get_child_pid() -> ExitCode {
    // This is 3 because we create an array of fds that will be passed to this
    // program via a Flatpak DBus call, and the vte pty we need is in the 4th
    // slot in that array.

    let pid = unsafe { libc::tcgetpgrp(3) };
    if pid == -1 {
        eprintln!("error calling tcgetpgrp");
        return ExitCode::FAILURE;
    }

    println!("{}", pid);
    ExitCode::SUCCESS
}

fn get_proc_stat(pid: u64) -> ExitCode {
    let path = CString::new(format!("/proc/{}/stat", pid)).expect("Could not create pid stat path");
    let uid = unsafe {
        let mut statbuf: libc::stat = std::mem::zeroed();
        let ret = libc::stat(path.as_ptr(), &mut statbuf);

        if ret == -1 {
            eprintln!("stat failed for pid {}", pid);
            return ExitCode::FAILURE;
        }

        statbuf.st_uid
    };
    println!("{}", uid);
    ExitCode::SUCCESS
}

#[derive(Debug, Parser)]
#[command(name = "terms-toolbox")]
#[command(about = "Terms companion for flatpak")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    GetPasswd { user: String },
    GetChildPid,
    GetProcStat { pid: u64 },
}

fn main() -> ExitCode {
    let args = Cli::parse();

    match args.command {
        Commands::GetPasswd { user } => get_passwd(user),
        Commands::GetChildPid => get_child_pid(),
        Commands::GetProcStat { pid } => get_proc_stat(pid),
    }
}
