use once_cell::sync::OnceCell;
use async_std::{fs::File, prelude::*, path::PathBuf};

static IS_SANDBOXED: OnceCell<bool> = OnceCell::new();

/// Check whether the application is running inside a sandbox.
///
/// The function checks whether the file `/.flatpak-info` exists, or if the app
/// is running as a snap, or if the environment variable `GTK_USE_PORTAL` is set
/// to `1`. As the return value of this function will not change during the
/// runtime of a program; it is cached for future calls.
pub async fn is_sandboxed() -> bool {
    if let Some(cached_value) = IS_SANDBOXED.get() {
        return *cached_value;
    }
    let new_value = is_flatpak().await
        || is_snap().await
        || std::env::var("GTK_USE_PORTAL")
            .map(|v| v == "1")
            .unwrap_or(false);
    IS_SANDBOXED.set(new_value).unwrap(); // Safe to unwrap here
    new_value
}



pub(crate) async fn is_flatpak() -> bool {
    PathBuf::from("/.flatpak-info").exists().await
}

pub(crate) async fn is_snap() -> bool {
    let pid = std::process::id();
    let path = format!("/proc/{pid}/cgroup");
    let mut file = match File::open(path).await {
        Ok(file) => file,
        Err(_) => return false,
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer).await {
        Ok(_) => cgroup_v2_is_snap(&buffer),
        Err(_) => false,
    }
}

fn cgroup_v2_is_snap(cgroups: &str) -> bool {
    cgroups
        .lines()
        .map(|line| {
            let (n, rest) = line.split_once(':')?;
            // Check that n is a number.
            n.parse::<u32>().ok()?;
            let unit = match rest.split_once(':') {
                Some(("", unit)) => Some(unit),
                Some(("freezer", unit)) => Some(unit),
                Some(("name=systemd", unit)) => Some(unit),
                _ => None,
            }?;
            let scope = std::path::Path::new(unit).file_name()?.to_str()?;

            Some(scope.starts_with("snap."))
        })
        .any(|x| x.unwrap_or(false))
}



	// 	// #if BLACKBOX_IS_FLATPAK
	// 	//   internal string? flatpak_root = null;

	// 	//   public string get_flatpak_root () throws GLib.Error {
	// 	//     if (flatpak_root == null) {
	// 	//       KeyFile kf = new KeyFile ();

	// 	//       kf.load_from_file ("/.flatpak-info", KeyFileFlags.NONE);
	// 	//       flatpak_root = kf.get_string ("Instance", "app-path");
	// 	//     }
	// 	//     return flatpak_root;
	// 	//   }
	// 	// #endif

	// 	pub fn host_or_flatpak_spawn(argv: &[&str]) { // Result<(Option<String>, i32), glib::Error> {

	// 		// GLib.Subprocess sp;
	// 		// GLib.SubprocessLauncher launcher;
	// 		// string[] real_argv = {};
	// 		// string? buf = null;

	// 		// status = -1;

	// 		// #if BLACKBOX_IS_FLATPAK
	// 		//     real_argv += "flatpak-spawn";
	// 		//     real_argv += "--host";
	// 		// #endif

	// 		// foreach (unowned string arg in argv) {
	// 		//   real_argv += arg;
	// 		// }

	// 		// let launcher = glib::Subp new GLib.SubprocessLauncher (
	// 		//   SubprocessFlags.STDOUT_PIPE | SubprocessFlags.STDERR_SILENCE
	// 		// );

	// 		// launcher.unsetenv ("G_MESSAGES_DEBUG");
	// 		// sp = launcher.spawnv (real_argv);

	// 		// if (sp == null) return null;

	// 		// if (!sp.communicate_utf8 (null, null, out buf, null)) return null;

	// 		// int exit_status = sp.get_exit_status ();
	// 		// status = exit_status;

	// 		// return buf;
	// 	}

	// 	/// fp_guess_shell
	// 	///
	// 	/// Copyright 2019 Christian Hergert <chergert@redhat.com>
	// 	///
	// 	/// The following function is a derivative work of the code from
	// 	/// https://gitlab.gnome.org/chergert/flatterm which is licensed under the
	// 	/// Apache License, Version 2.0 <LICENSE-APACHE or
	// 	/// https://opensource.org/licenses/MIT>, at your option. This file may not
	// 	/// be copied, modified, or distributed except according to those terms.
	// 	///
	// 	/// SPDX-License-Identifier: (MIT OR Apache-2.0)
	// 	fn guess_shell(cancellable: Option<&Cancellable>) { //  -> Result<String, glib::Error> {

	// 		// #if !BLACKBOX_IS_FLATPAK
	// 		//     return Vte.get_user_shell();
	// 		// #endif

	// 		//     string[] argv = { "flatpak-spawn", "--host", "getent", "passwd",
	// 		//       Environment.get_user_name() };

	// 		//     var launcher = new GLib.SubprocessLauncher(
	// 		//       SubprocessFlags.STDOUT_PIPE | SubprocessFlags.STDERR_SILENCE
	// 		//     );

	// 		//     launcher.unsetenv("G_MESSAGES_DEBUG");
	// 		//     var sp = launcher.spawnv(argv);

	// 		//     if (sp == null)
	// 		//       return null;

	// 		//     string? buf = null;
	// 		//     if (!sp.communicate_utf8(null, cancellable, out buf, null))
	// 		//       return null;

	// 		//     var parts = buf.split(":");

	// 		//     if (parts.length < 7) {
	// 		//       return null;
	// 		//     }

	// 		//     return parts[6].strip();
	// 	}

	// 	fn get_env(cancellable: Option<&Cancellable>) { //  -> Result<Vec<String>, glib::Error> {
	// 		                                          // #if !BLACKBOX_IS_FLATPAK
	// 		                                          //     return Environ.get();
	// 		                                          // #endif

	// 		//     string[] argv = { "flatpak-spawn", "--host", "env" };

	// 		//     var launcher = new GLib.SubprocessLauncher(
	// 		//       SubprocessFlags.STDOUT_PIPE | SubprocessFlags.STDERR_SILENCE
	// 		//     );

	// 		//     launcher.setenv("G_MESSAGES_DEBUG", "false", true);

	// 		//     var sp = launcher.spawnv(argv);

	// 		//     if (sp == null)
	// 		//       return null;

	// 		//     string? buf = null;
	// 		//     if (!sp.communicate_utf8(null, cancellable, out buf, null))
	// 		//       return null;

	// 		//     string[] arr = buf.strip().split("\n");

	// 		//     return arr;
	// 	}

	// 	async fn get_foreground_process(
	// 		terminal_fd: i32,
	// 		cancellable: Option<&Cancellable>,
	// 	) {
	// 		// #if !BLACKBOX_IS_FLATPAK
	// 		//     return Posix.tcgetpgrp (terminal_fd);
	// 		// #endif

	// 		//     try {
	// 		//       KeyFile kf = new KeyFile ();

	// 		//       kf.load_from_file ("/.flatpak-info", KeyFileFlags.NONE);
	// 		//       string host_root = kf.get_string ("Instance", "app-path");

	// 		//       var argv = new Array<string> ();

	// 		//       argv.append_val ("%s/bin/terminal-toolbox".printf (host_root));
	// 		//       argv.append_val ("tcgetpgrp");
	// 		//       argv.append_val (terminal_fd.to_string ());

	// 		//       int[] fds = new int[2];

	// 		//       // This creates two fds, where we can write to one and read from the
	// 		//       // other. We'll pass one fd to the HostCommand as stdout, which means
	// 		//       // we'll be able to read what is HostCommand prints out from the other
	// 		//       // fd we just opened.
	// 		//       Unix.open_pipe (fds, Posix.FD_CLOEXEC);

	// 		//       var read_fs = GLib.FileStream.fdopen (fds [0], "r");
	// 		//       var write_fs = GLib.FileStream.fdopen (fds [1], "w");
	// 		//       int[] pass_fds = {
	// 		//         0,
	// 		//         write_fs.fileno (), // stdout for toolbox, we can read from read_fs
	// 		//         2,
	// 		//         terminal_fd // we pass the terminal fd as (3) for toolbox
	// 		//       };

	// 		//       debug ("Send command");
	// 		//       yield send_host_command (null, argv, new Array<string> (), pass_fds, null, null, null);

	// 		//       string text = read_fs.read_line ();
	// 		//       int response;

	// 		//       if (int.try_parse (text, out response, null, 10)) {
	// 		//         return response;
	// 		//       }
	// 		//     }
	// 		//     catch (GLib.Error e) {
	// 		//       warning ("%s", e.message);
	// 		//     }

	// 		//     return -1;
	// 	}

	// 	//   public delegate void HostCommandExitedCallback (uint pid, uint status);

	// 	///
	// 	/// The following function is derivative work of
	// 	/// https://github.com/gnunn1/tilix/blob/ddf5e5c069ab7d40f973cb2554eae5b13b23a87f/source/gx/tilix/terminal/terminal.d#L2967
	// 	/// which is licensed under the Mozilla Public License 2.0. If a copy of the
	// 	/// MPL was not distributed with this file, You can obtain one at
	// 	/// http://mozilla.org/MPL/2.0/.
	// 	///
	// 	fn send_host_command(
	// 		cwd: &Path,
	// 		argv: &[&str],
	// 		envv: &[&str],
	// 		fds: &[i32],
	// 	) {
	// 		//   public static async bool send_host_command (
	// 		//     string? cwd,
	// 		//     Array<string> argv,
	// 		//     Array<string> envv,
	// 		//     int[] fds,
	// 		//     HostCommandExitedCallback? callback,
	// 		//     GLib.Cancellable? cancellable,
	// 		//     out int pid
	// 		//   ) throws GLib.Error {
	// 		//     pid = -1;

	// 		// #if !BLACKBOX_IS_FLATPAK
	// 		//     return false;
	// 		// #endif

	// 		//     uint[] handles = {};

	// 		//     GLib.UnixFDList out_fd_list;
	// 		//     GLib.UnixFDList in_fd_list = new GLib.UnixFDList ();

	// 		//     foreach (var fd in fds) {
	// 		//       handles += in_fd_list.append (fd);
	// 		//     }

	// 		//     var connection = yield new DBusConnection.for_address (
	// 		//       GLib.Environment.get_variable ("DBUS_SESSION_BUS_ADDRESS"),
	// 		//       GLib.DBusConnectionFlags.AUTHENTICATION_CLIENT
	// 		//         | GLib.DBusConnectionFlags.MESSAGE_BUS_CONNECTION,
	// 		//       null,
	// 		//       null
	// 		//     );

	// 		//     connection.exit_on_close = true;

	// 		//     uint signal_id = 0;

	// 		//     signal_id = connection.signal_subscribe (
	// 		//       "org.freedesktop.Flatpak",
	// 		//       "org.freedesktop.Flatpak.Development",
	// 		//       "HostCommandExited",
	// 		//       "/org/freedesktop/Flatpak/Development",
	// 		//       null,
	// 		//       DBusSignalFlags.NONE,
	// 		//       // This callback is only called if the command is properly spawned. It is
	// 		//       // not called if spawning the command fails.
	// 		//       (_connection, sender_name, object_path, interface_name, signal_name, parameters) => {
	// 		//         connection.signal_unsubscribe (signal_id);

	// 		//         // I'm not sure which pid this is (it might be from the process that
	// 		//         // just exited or from the dbus command call).
	// 		//         uint ppid = 0;
	// 		//         // This is the return status of the command that just exited. Any
	// 		//         // non-zero value means the shell/command exited with an error.
	// 		//         uint status = 0;

	// 		//         parameters.get ("(uu)", &ppid, &status);

	// 		//         debug ("Command exited %s %s %s %s pid: %u status %u", signal_name, sender_name, object_path, interface_name, ppid, status);

	// 		//         if (callback != null) {
	// 		//           if (cancellable?.is_cancelled ()) {
	// 		//             //  callback = null;
	// 		//           }
	// 		//           else {
	// 		//             callback (ppid, status);
	// 		//           }
	// 		//         }
	// 		//       }
	// 		//     );

	// 		//     var parameters = build_host_command_variant (cwd, argv, envv, handles);

	// 		//     Variant? reply = null;

	// 		//     try {
	// 		//       reply = yield connection.call_with_unix_fd_list (
	// 		//         "org.freedesktop.Flatpak",
	// 		//         "/org/freedesktop/Flatpak/Development",
	// 		//         "org.freedesktop.Flatpak.Development",
	// 		//         "HostCommand",
	// 		//         parameters,
	// 		//         new VariantType ("(u)"),
	// 		//         GLib.DBusCallFlags.NONE,
	// 		//         -1,
	// 		//         in_fd_list,
	// 		//         null,
	// 		//         out out_fd_list
	// 		//       );
	// 		//     }
	// 		//     catch (GLib.Error e) {
	// 		//       // If we reach this catch block the command we tried to spawn very likely
	// 		//       // failed. In the context of opening new terminals, this means we failed
	// 		//       // to spawn the user's shell or the specific command given to a tab. Most
	// 		//       // users would expect to see an error banner/alert at this point.
	// 		//       connection.signal_unsubscribe (signal_id);
	// 		//       throw e;
	// 		//     }

	// 		//     if (reply == null) {
	// 		//       warning ("No reply from flatpak dbus service");
	// 		//       connection.signal_unsubscribe (signal_id);
	// 		//       return false;
	// 		//     }
	// 		//     else {
	// 		//       // Pid from the host command we just spawned
	// 		//       uint p = 0;
	// 		//       reply.get ("(u)", &p);
	// 		//       pid = (int) p;
	// 		//     }

	// 		//     return true;
	// 	}

	// 	//   // This function builds a Variant to be passed to Flatpak's HostCommand DBus
	// 	//   // call. See the following link for more details:
	// 	//   // https://github.com/flatpak/flatpak/blob/01910ad12fd840a8667879f9a479a66e441cccdd/data/org.freedesktop.Flatpak.xml#L110
	// 	//   public static Variant build_host_command_variant (
	// 	//     string? cwd,
	// 	//     Array<string> argv,
	// 	//     Array<string> envv,
	// 	//     uint[] handles
	// 	//   ) {
	// 	//     if (cwd == null) {
	// 	//       cwd = GLib.Environment.get_home_dir ();
	// 	//     }

	// 	//     var handles_vb = new VariantBuilder (new VariantType ("a{uh}"));
	// 	//     for (uint i = 0; i < handles.length; i++) {
	// 	//       handles_vb.add_value (new Variant ("{uh}", i, (int32) handles [i]));
	// 	//     }

	// 	//     var envv_vb = new VariantBuilder (new VariantType ("a{ss}"));
	// 	//     foreach (unowned string env in envv.data) {
	// 	//       if (env == null) break;

	// 	//       string[] parts = env.split ("=");
	// 	//       if (parts.length == 2) {
	// 	//         envv_vb.add_value (new Variant ("{ss}", parts [0], parts [1]));
	// 	//       }
	// 	//     }

	// 	//     var he = handles_vb.end ();
	// 	//     var ee = envv_vb.end ();

	// 	//     return new Variant (
	// 	//       "(^ay^aay@a{uh}@a{ss}u)",
	// 	//       cwd,
	// 	//       argv.data,
	// 	//       he,
	// 	//       ee,
	// 	//       2
	// 	//     );
	// 	//   }

	// 	//   public string? get_process_cmdline (int pid) {
	// 	//     try {
	// 	//       //  ps -p PID -o args --no-headers
	// 	//       string? response = host_or_flatpak_spawn ({
	// 	//         "ps",
	// 	//         "-p",
	// 	//         pid.to_string (),
	// 	//         "-o",
	// 	//         "args",
	// 	//         "--no-headers"
	// 	//       });

	// 	//       return response.strip ();
	// 	//     }
	// 	//     catch (GLib.Error e) {
	// 	//       warning ("%s", e.message);
	// 	//     }
	// 	//     return null;
	// 	//   }

	// 	//   public int get_euid_from_pid (int pid,
	// 	//                                 GLib.Cancellable? cancellable) throws GLib.Error
	// 	//   {
	// 	//     string proc_file = @"/proc/$pid";
	// 	// #if BLACKBOX_IS_FLATPAK
	// 	//     string[] argv = {
	// 	//       "%s/bin/terminal-toolbox".printf (get_flatpak_root ()),
	// 	//       "stat",
	// 	//       proc_file
	// 	//     };

	// 	//     int status;
	// 	//     var response = host_or_flatpak_spawn (argv, out status);
	// 	//     int euid = -1;

	// 	//     if (status == 0 && int.try_parse (response.strip (), out euid, null, 10)) {
	// 	//       return euid;
	// 	//     }
	// 	//     else {
	// 	//       return -1;
	// 	//     }
	// 	// #else
	// 	//     Posix.Stat? buf = null;
	// 	//     Posix.stat (proc_file, out buf);

	// 	//     return (int) buf.st_uid;
	// 	// #endif
	// 	//   }
	// 	// }
	// }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cgroup_v2_is_snap() {
        let data =
            "0::/user.slice/user-1000.slice/user@1000.service/apps.slice/snap.something.scope\n";
        assert_eq!(cgroup_v2_is_snap(data), true);

        let data = "0::/user.slice/user-1000.slice/user@1000.service/apps.slice\n";
        assert_eq!(cgroup_v2_is_snap(data), false);

        let data = "12:pids:/user.slice/user-1000.slice/user@1000.service
11:perf_event:/
10:net_cls,net_prio:/
9:cpuset:/
8:memory:/user.slice/user-1000.slice/user@1000.service/apps.slice/apps-org.gnome.Terminal.slice/vte-spawn-228ae109-a869-4533-8988-65ea4c10b492.scope
7:rdma:/
6:devices:/user.slice
5:blkio:/user.slice
4:hugetlb:/
3:freezer:/snap.portal-test
2:cpu,cpuacct:/user.slice
1:name=systemd:/user.slice/user-1000.slice/user@1000.service/apps.slice/apps-org.gnome.Terminal.slice/vte-spawn-228ae109-a869-4533-8988-65ea4c10b492.scope
0::/user.slice/user-1000.slice/user@1000.service/apps.slice/apps-org.gnome.Terminal.slice/vte-spawn-228ae109-a869-4533-8988-65ea4c10b492.scope\n";
        assert_eq!(cgroup_v2_is_snap(data), true);
    }
}
