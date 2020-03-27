fn main() {
    use clap::{load_yaml, App};
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    if matches.is_present("msgbox") {
        // do a messagebox
    }else{
        eprintln!("Needed a widget type");
        std::process::exit(1);
    }

    let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
    let opts: notcurses_options = libnotcurses-sys::notcurses_options {
        inhibit_alternate_screen: true,
        loglevel: 0,
        termtype: std::ptr::null(),
        retain_cursor: false,
        suppress_banner: false,
        no_winch_sighandler: false,
        no_quit_sighandlers: false,
        renderfp: std::ptr::null_mut(),
        margin_t: 0,
        margin_r: 0,
        margin_b: 0,
        margin_l: 0,
    };
    let nc = notcurses_init(&opts, libc_stdout());
    notcurses_stop(nc);
}
