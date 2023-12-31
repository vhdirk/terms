use constcat::concat;
use gettextrs::gettext;

pub const PCRE_MULTILINE: u32 = 1024;

// Copyright (c) 2011-2017 elementary LLC. (https://elementary.io)
// From: https://github.com/elementary/terminal/blob/c3e36fb2ab64c18028ff2b4a6da5bfb2171c1c04/src/Widgets/TerminalWidget.vala
pub const USERCHARS: &'static str = "-[:alnum:]";
pub const USERCHARS_CLASS: &'static str = concat!("[", USERCHARS, "]");
pub const PASSCHARS_CLASS: &'static str = "[-[:alnum:]\\Q,?;.:/!%$^*&~\"#'\\E]";
pub const HOSTCHARS_CLASS: &'static str = "[-[:alnum:]]";
pub const HOST: &'static str = concat!(HOSTCHARS_CLASS, "+(\\.", HOSTCHARS_CLASS, "+)*");
pub const PORT: &'static str = "(?:\\:[[:digit:]]{1,5})?";
pub const PATHCHARS_CLASS: &'static str = "[-[:alnum:]\\Q_$.+!*,;:@&=?/~#%\\E]";
pub const PATHTERM_CLASS: &'static str = "[^\\Q]'.}>) \t\r\n,\"\\E]";
pub const SCHEME: &'static str = concat!(
    "(?:news:|telnet:|nntp:|file:\\/|https?:|ftps?:|sftp:|webcal:",
    "|irc:|sftp:|ldaps?:|nfs:|smb:|rsync:|ssh:|rlogin:|telnet:|git:",
    "|git\\+ssh:|bzr:|bzr\\+ssh:|svn:|svn\\+ssh:|hg:|mailto:|magnet:)"
);

pub const USERPASS: &'static str = concat!(USERCHARS_CLASS, "+(?:", PASSCHARS_CLASS, "+)?");
pub const URLPATH: &'static str = concat!(
    "(?:(/",
    PATHCHARS_CLASS,
    "+(?:[(]",
    PATHCHARS_CLASS,
    "*[)])*",
    PATHCHARS_CLASS,
    "*)*",
    PATHTERM_CLASS,
    ")?"
);

pub const URL_REGEX_STRINGS: [&str; 5] = [
    concat!(SCHEME, "//(?:", USERPASS, "\\@)?", HOST, PORT, URLPATH),
    concat!("(?:www|ftp)", HOSTCHARS_CLASS, "*\\.", HOST, PORT, URLPATH),
    concat!(
        "(?:callto:|h323:|sip:)",
        USERCHARS_CLASS,
        "[",
        USERCHARS,
        ".]*(?:",
        PORT,
        "/[a-z0-9]+)?\\@",
        HOST
    ),
    concat!("(?:mailto:)?", USERCHARS_CLASS, "[", USERCHARS, ".]*\\@", HOSTCHARS_CLASS, "+\\.", HOST),
    "(?:news:|man:|info:)[[:alnum:]\\Q^_{|}~!\"#$%&'()*+,./;:=?`\\E]+",
];

// pub const MENU_BUTTON_ALTERNATIVE: &'static str =
// 	&gettext("You can still access the menu by right-clicking any terminal.");

// pub const COPYING_NOT_IMPLEMENTED_WARNING_FMT: &'static str = &gettext("%s uses an early Gtk 4 port of VTE as a terminal widget. While a lot of progress has been made on this port, copying has yet to be implemented. This means there's currently no way to copy text in %s.");

// pub const INFINITE_SCROLLBACK_WARNING: &'static str = &gettext("Warning: unlimited scrollback saves content to disk, which may cause your system to run out of storage.");

//   public string get_user_schemes_dir () {
//     return Path.build_path(
//       Path.DIR_SEPARATOR_S, Environment.get_user_data_dir (), "blackbox", "schemes"
//     );
//   }

//   public string get_user_keybindings_path () {
//     return Path.build_path(
//       Path.DIR_SEPARATOR_S, Environment.get_user_data_dir (), "blackbox", "user-keymap.json"
//     );
//   }

//   public string get_app_schemes_dir () {
//     return Path.build_path (
//       Path.DIR_SEPARATOR_S, DATADIR, "blackbox", "schemes"
//     );
//   }
