extern crate notcurses;
extern crate libnotcurses_sys as ffi;

extern {
  fn libc_stdout() -> *mut ffi::_IO_FILE;
}

fn msgbox(nc: *mut ffi::notcurses, dimy: i32, dimx: i32, text: &str) {
    unsafe{
        let p = ffi::ncplane_new(nc, dimy, dimx, 0, 0, std::ptr::null_mut());
        notcurses::ncplane_putstr(p, text);
    }
    notcurses::render(nc).expect("failed rendering");
}

fn main() {
    use clap::{load_yaml, App};
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    unsafe{
        let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
        let opts: ffi::notcurses_options = ffi::notcurses_options {
            inhibit_alternate_screen: true,
            loglevel: 0,
            termtype: std::ptr::null(),
            retain_cursor: false,
            suppress_banner: false,
            no_winch_sighandler: false,
            no_quit_sighandlers: false,
            renderfp: std::ptr::null_mut(),
            margin_t: 8,
            margin_r: 8,
            margin_b: 8,
            margin_l: 8,
        };
        let nc = ffi::notcurses_init(&opts, libc_stdout());
        let stdplane = ffi::notcurses_stdplane(nc);
        let mut dimy = 0;
        let mut dimx = 0;
        ffi::ncplane_dim_yx(stdplane, &mut dimy, &mut dimx);
        if matches.is_present("msgbox") {
            msgbox(nc, dimy, dimx, matches.value_of("text").unwrap());
        }else{
            eprintln!("\nNeeded a widget type");
            ffi::notcurses_stop(nc);
            std::process::exit(1);
        }
        let mut ni: ffi::ncinput = std::mem::zeroed();
        notcurses::getc_blocking(nc, &mut ni);
        ffi::notcurses_stop(nc);
    }
}
