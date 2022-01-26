// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from ../gir-files (@ 54ae87ae2ece+)
// from ../xrd-gir-files (@ 8cffc8b155f9)
// DO NOT EDIT

use crate::Container;
use crate::Window;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "XrdWindowManager")]
    pub struct WindowManager(Object<ffi::XrdWindowManager, ffi::XrdWindowManagerClass>);

    match fn {
        type_ => || ffi::xrd_window_manager_get_type(),
    }
}

impl WindowManager {
    #[doc(alias = "xrd_window_manager_new")]
    pub fn new() -> WindowManager {
        unsafe {
            from_glib_full(ffi::xrd_window_manager_new())
        }
    }

    #[doc(alias = "xrd_window_manager_add_container")]
    pub fn add_container(&self, container: &Container) {
        unsafe {
            ffi::xrd_window_manager_add_container(self.to_glib_none().0, container.to_glib_none().0);
        }
    }

    //#[doc(alias = "xrd_window_manager_add_window")]
    //pub fn add_window(&self, window: &impl IsA<Window>, flags: /*Ignored*/WindowFlags) {
    //    unsafe { TODO: call ffi:xrd_window_manager_add_window() }
    //}

    #[doc(alias = "xrd_window_manager_arrange_reset")]
    pub fn arrange_reset(&self) {
        unsafe {
            ffi::xrd_window_manager_arrange_reset(self.to_glib_none().0);
        }
    }

    #[doc(alias = "xrd_window_manager_arrange_sphere")]
    pub fn arrange_sphere(&self, context: &impl IsA<gxr::Context>) -> bool {
        unsafe {
            from_glib(ffi::xrd_window_manager_arrange_sphere(self.to_glib_none().0, context.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "xrd_window_manager_check_grab")]
    pub fn check_grab(&self, controller: &gxr::Controller) {
        unsafe {
            ffi::xrd_window_manager_check_grab(self.to_glib_none().0, controller.to_glib_none().0);
        }
    }

    #[doc(alias = "xrd_window_manager_check_release")]
    pub fn check_release(&self, controller: &gxr::Controller) {
        unsafe {
            ffi::xrd_window_manager_check_release(self.to_glib_none().0, controller.to_glib_none().0);
        }
    }

    #[doc(alias = "xrd_window_manager_drag_start")]
    pub fn drag_start(&self, controller: &gxr::Controller) {
        unsafe {
            ffi::xrd_window_manager_drag_start(self.to_glib_none().0, controller.to_glib_none().0);
        }
    }

    #[doc(alias = "xrd_window_manager_get_buttons")]
    #[doc(alias = "get_buttons")]
    pub fn buttons(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::xrd_window_manager_get_buttons(self.to_glib_none().0))
        }
    }

    //#[doc(alias = "xrd_window_manager_get_hover_mode")]
    //#[doc(alias = "get_hover_mode")]
    //pub fn hover_mode(&self) -> /*Ignored*/HoverMode {
    //    unsafe { TODO: call ffi:xrd_window_manager_get_hover_mode() }
    //}

    #[doc(alias = "xrd_window_manager_get_windows")]
    #[doc(alias = "get_windows")]
    pub fn windows(&self) -> Vec<Window> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::xrd_window_manager_get_windows(self.to_glib_none().0))
        }
    }

    #[doc(alias = "xrd_window_manager_poll_window_events")]
    pub fn poll_window_events(&self, context: &impl IsA<gxr::Context>) {
        unsafe {
            ffi::xrd_window_manager_poll_window_events(self.to_glib_none().0, context.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "xrd_window_manager_remove_container")]
    pub fn remove_container(&self, container: &Container) {
        unsafe {
            ffi::xrd_window_manager_remove_container(self.to_glib_none().0, container.to_glib_none().0);
        }
    }

    #[doc(alias = "xrd_window_manager_remove_window")]
    pub fn remove_window(&self, window: &impl IsA<Window>) {
        unsafe {
            ffi::xrd_window_manager_remove_window(self.to_glib_none().0, window.as_ref().to_glib_none().0);
        }
    }

    //#[doc(alias = "xrd_window_manager_scale")]
    //pub fn scale(&self, grab_state: /*Ignored*/&mut gxr::GrabState, factor: f32, update_rate_ms: f32) {
    //    unsafe { TODO: call ffi:xrd_window_manager_scale() }
    //}

    //#[doc(alias = "xrd_window_manager_set_hover_mode")]
    //pub fn set_hover_mode(&self, mode: /*Ignored*/HoverMode) {
    //    unsafe { TODO: call ffi:xrd_window_manager_set_hover_mode() }
    //}

    #[doc(alias = "xrd_window_manager_update_controller")]
    pub fn update_controller(&self, controller: &gxr::Controller) {
        unsafe {
            ffi::xrd_window_manager_update_controller(self.to_glib_none().0, controller.to_glib_none().0);
        }
    }

    #[doc(alias = "no-hover-event")]
    pub fn connect_no_hover_event<F: Fn(&Self, &gdk::Event) + Send + Sync + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn no_hover_event_trampoline<F: Fn(&WindowManager, &gdk::Event) + Send + Sync + 'static>(this: *mut ffi::XrdWindowManager, object: *mut gdk::ffi::GdkEvent, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_none(object))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"no-hover-event\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(no_hover_event_trampoline::<F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl Default for WindowManager {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

unsafe impl Send for WindowManager {}
unsafe impl Sync for WindowManager {}

impl fmt::Display for WindowManager {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WindowManager")
    }
}
