extern crate libnotcurses_sys as ffi;

extern {
  fn libc_stdout() -> *mut ffi::_IO_FILE;
}

fn msgbox(nc: *mut ffi::notcurses, dimy: i32, dimx: i32, text: &str) {
    unsafe{
        let p = ffi::ncplane_new(nc, dimy, dimx, 0, 0, std::ptr::null_mut());
        let mut ul = ffi::NcCell { gcluster: 0, gcluster_backstop: 0, width: 0, stylemask: 0, channels: 0, };
        let mut ur = ffi::NcCell { gcluster: 0, gcluster_backstop: 0, width: 0, stylemask: 0, channels: 0, };
        let mut bl = ffi::NcCell { gcluster: 0, gcluster_backstop: 0, width: 0, stylemask: 0, channels: 0, };
        let mut br = ffi::NcCell { gcluster: 0, gcluster_backstop: 0, width: 0, stylemask: 0, channels: 0, };
        let mut hl = ffi::NcCell { gcluster: 0, gcluster_backstop: 0, width: 0, stylemask: 0, channels: 0, };
        let mut vl = ffi::NcCell { gcluster: 0, gcluster_backstop: 0, width: 0, stylemask: 0, channels: 0, };
        ffi::cells_rounded_box(p, 0, 0, &mut ul, &mut ur, &mut bl, &mut br, &mut hl, &mut vl);
        ffi::ncplane_perimeter(p, &ul, &ur, &bl, &br, &hl, &vl, 0);
        let mut sbytes = 0;
        let mut _cols = ffi::ncplane_puttext(p, 0, ffi::ncalign_e_NCALIGN_LEFT,
                            std::ffi::CString::new(text).expect("Bad string").as_ptr(),
                            &mut sbytes);
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
            margin_t: 2,
            margin_r: 2,
            margin_b: 2,
            margin_l: 2,
            flags: (ffi::NCOPTION_INHIBIT_SETLOCALE | ffi::NCOPTION_NO_ALTERNATE_SCREEN) as u64,
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
        let mut ni: ffi::NcInput = std::mem::zeroed();
        notcurses::getc_blocking(nc, &mut ni);
        ffi::notcurses_stop(nc);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial; // serialize tests using notcurses

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
            msgbox(nc, dimy, dimx, "This box ought be centered");
            ffi::notcurses_stop(nc);
        }
    }
}
