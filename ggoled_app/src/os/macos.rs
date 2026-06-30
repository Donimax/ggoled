use super::{Media, OSFeatures};
use media_remote::NowPlayingPerl;

pub fn hide_dock_icon() {
    use std::ffi::c_void;

    unsafe extern "C" {
        fn objc_getClass(name: *const u8) -> *mut c_void;
        fn sel_registerName(name: *const u8) -> *mut c_void;
        #[link_name = "objc_msgSend"]
        fn objc_msgSend_noarg(obj: *mut c_void, sel: *mut c_void) -> *mut c_void;
        #[link_name = "objc_msgSend"]
        fn objc_msgSend_isize(obj: *mut c_void, sel: *mut c_void, arg: isize) -> bool;
    }

    unsafe {
        let cls = objc_getClass(b"NSApplication\0".as_ptr());
        let shared_sel = sel_registerName(b"sharedApplication\0".as_ptr());
        let app = objc_msgSend_noarg(cls, shared_sel);

        let policy_sel = sel_registerName(b"setActivationPolicy:\0".as_ptr());
        // NSApplicationActivationPolicyAccessory = 1 (no dock icon)
        objc_msgSend_isize(app, policy_sel, 1);
    }
}

pub struct OSImpl {
    npp: NowPlayingPerl,
}
impl OSFeatures for OSImpl {
    fn new() -> Self {
        let npp = NowPlayingPerl::new();
        Self { npp }
    }

    fn supports_media(&self) -> bool {
        true
    }
    fn get_media(&mut self) -> Option<Media> {
        let guard = self.npp.get_info();
        let info = guard.as_ref()?;
        if info.is_playing == Some(true) {
            Some(Media {
                title: info.title.clone()?,
                artist: info.artist.clone()?,
            })
        } else {
            None
        }
    }

    fn supports_idle(&self) -> bool {
        // TODO
        false
    }
    fn is_idle(&mut self) -> bool {
        // TODO
        false
    }
}
