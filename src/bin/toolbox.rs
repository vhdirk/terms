use clap::{self, Parser, Subcommand};
use libc;
use std::process::ExitCode;
use terms_util::{libc_util, toolbox};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

// This is a simple program meant to be launched with flatpak-spawn --host to
// retrieve host information Flatpak'ed apps don't have access to. The original
// idea for this program came from
// https://github.com/gnunn1/tilix/blob/master/experimental/flatpak/tilix-flatpak-toolbox.c
// https://gitlab.gnome.org/raggesilver/blackbox/-/blob/e1862c558046783ef47ba6332734d77b25370e4d/toolbox/main.c

/// Print the current env as yaml (since we have that serializer as a dependency already).
/// Printing as a structured format rather than KEY=VALUE makes parsing at the other end a whole lot
/// easier
fn get_env() -> ExitCode {
    let mut mapping = serde_yaml::Mapping::new();
    for (key, value) in std::env::vars() {
        mapping.insert(serde_yaml::Value::String(key), serde_yaml::Value::String(value));
    }

    match serde_yaml::to_string(&mapping) {
        Ok(out) => {
            println!("{}", out);
            ExitCode::SUCCESS
        },
        Err(err) => {
            eprintln!("Could not get env {}", err);
            ExitCode::FAILURE
        },
    }
}

fn get_home_directory() -> ExitCode {
    if let Some(home_dir) = dirs::home_dir() {
        println!("{}", home_dir.display());
    } else {
        println!("/")
    }
    ExitCode::SUCCESS
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
    match libc_util::tcgetpgrp(3) {
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
    Env,
    HomeDirectory,
    Shell { uid: libc::uid_t },
    ChildPid,
    ProcessOwner { pid: libc::pid_t },
    ProcessStatus { pid: libc::pid_t },
    ProcessCmdline { pid: libc::pid_t },
}

fn main() -> ExitCode {
    tracing_subscriber::registry()
        .with(fmt::layer().with_filter(EnvFilter::from_default_env()))
        .init();

    let args = Cli::parse();

    match args.command {
        Commands::HomeDirectory => get_home_directory(),
        Commands::Env => get_env(),
        Commands::Shell { uid } => get_shell(uid),
        Commands::ChildPid => get_child_pid(),
        Commands::ProcessOwner { pid } => get_process_owner(pid),
        Commands::ProcessStatus { pid } => get_process_status(pid),
        Commands::ProcessCmdline { pid } => get_process_cmdline(pid),
    }
}
