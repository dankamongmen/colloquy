extern crate notcurses;
extern crate libnotcurses_sys as ffi;

extern {
  fn libc_stdout() -> *mut ffi::_IO_FILE;
}

fn msgbox(nc: *mut ffi::notcurses, dimy: i32, dimx: i32, text: &str) {
    unsafe{
        let p = ffi::ncplane_new(nc, dimy, dimx, 0, 0, std::ptr::null_mut());
        ffi::ncplane_double_box(p, 0, 0, dimy - 1, dimx - 1, 0);
        ffi::ncplane_putstr(p, text);
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
            loglevel: 0,
            termtype: std::ptr::null(),
            renderfp: std::ptr::null_mut(),
            margin_t: 8,
            margin_r: 8,
            margin_b: 8,
            margin_l: 8,
            flags: (ffi::NCOPTION_INHIBIT_SETLOCALE |
                    ffi::NCOPTION_NO_ALTERNATE_SCREEN) as u64,
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

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test_derive::serial; // serialize tests using notcurses

    #[test]
    #[serial]
    fn test_msgbox() {
        unsafe {
            let _ = libc::setlocale(libc::LC_ALL, std::ffi::CString::new("").unwrap().as_ptr());
            let nc = ffi::notcurses_init(std::ptr::null(), libc_stdout());
            assert_ne!(std::ptr::null(), nc);
            let mut dimy = 0;
            let mut dimx = 0;
            let _stdplane = notcurses::stddim_yx(nc, &mut dimy, &mut dimx);
            msgbox(nc, dimy, dimx, "This ought be centered");
            ffi::notcurses_stop(nc);
        }
    }
}
