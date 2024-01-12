use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[allow(unused)]
    pub struct PCRE2Flags: u32 {
        const ALLOW_EMPTY_CLASS = 0x00000001;
        const ALT_BSUX = 0x00000002;
        const AUTO_CALLOUT = 0x00000004;
        const CASELESS = 0x00000008;
        const DOLLAR_ENDONLY = 0x00000010;
        const DOTALL = 0x00000020;
        const DUPNAMES = 0x00000040;
        const EXTENDED = 0x00000080;
        const FIRSTLINE = 0x00000100;
        const MATCH_UNSET_BACKREF = 0x00000200;
        const MULTILINE = 0x00000400;
        const NEVER_UCP = 0x00000800;
        const NEVER_UTF = 0x00001000;
        const NO_AUTO_CAPTURE = 0x00002000;
        const NO_AUTO_POSSESS = 0x00004000;
        const NO_DOTSTAR_ANCHOR = 0x00008000;
        const NO_START_OPTIMIZE = 0x00010000;
        const UCP = 0x00020000;
        const UNGREEDY = 0x00040000;
        const UTF = 0x00080000;
        const ANCHORED = 0x80000000;
        const NO_UTF_CHECK = 0x40000000;
    }
}
