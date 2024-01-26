use glib::subclass::prelude::*;

mod flatpak;
mod native;
mod process_manager;

use process_manager as imp;

glib::wrapper! {
    pub struct ProcessManager(ObjectSubclass<imp::ProcessManager>);
}

impl ProcessManager {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    // async fn shell(&self) -> Option<String>;

    // async fn env(&self) -> Result<HashMap<String, String>, TermsError>;

    // async fn working_dir(&self) -> Result<PathBuf, TermsError>;

    // async fn spawn(
    //     &self,
    //     term: &vte::Terminal,
    //     flags: vte::PtyFlags,
    //     working_dir: PathBuf,
    //     argv: Vec<PathBuf>,
    //     envv: HashMap<String, String>,
    //     timeout: Duration,
    // ) -> Result<SpawnHandle, TermsError>;

    // /// Determines if a child process is running in the terminal, and returns the pid
    // async fn foreground_pid(&self, pty: &vte::Pty) -> Result<libc::pid_t, TermsError>;

    // async fn process_status(&self, pid: libc::pid_t) -> Result<String, TermsError>;

    // async fn process_cmdline(&self, pid: libc::pid_t) -> Result<String, TermsError>;
}
