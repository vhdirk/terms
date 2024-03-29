use constcat::concat;

// Copyright (c) 2011-2017 elementary LLC. (https://elementary.io)
// From: https://github.com/elementary/terminal/blob/c3e36fb2ab64c18028ff2b4a6da5bfb2171c1c04/src/Widgets/TerminalWidget.vala
pub const USERCHARS: &str = "-[:alnum:]";
pub const USERCHARS_CLASS: &str = concat!("[", USERCHARS, "]");
pub const PASSCHARS_CLASS: &str = "[-[:alnum:]\\Q,?;.:/!%$^*&~\"#'\\E]";
pub const HOSTCHARS_CLASS: &str = "[-[:alnum:]]";
pub const HOST: &str = concat!(HOSTCHARS_CLASS, "+(\\.", HOSTCHARS_CLASS, "+)*");
pub const PORT: &str = "(?:\\:[[:digit:]]{1,5})?";
pub const PATHCHARS_CLASS: &str = "[-[:alnum:]\\Q_$.+!*,;:@&=?/~#%\\E]";
pub const PATHTERM_CLASS: &str = "[^\\Q]'.}>) \t\r\n,\"\\E]";
pub const SCHEME: &str = concat!(
    "(?:news:|telnet:|nntp:|file:\\/|https?:|ftps?:|sftp:|webcal:",
    "|irc:|sftp:|ldaps?:|nfs:|smb:|rsync:|ssh:|rlogin:|telnet:|git:",
    "|git\\+ssh:|bzr:|bzr\\+ssh:|svn:|svn\\+ssh:|hg:|mailto:|magnet:)"
);

pub const USERPASS: &str = concat!(USERCHARS_CLASS, "+(?:", PASSCHARS_CLASS, "+)?");
pub const URLPATH: &str = concat!(
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
