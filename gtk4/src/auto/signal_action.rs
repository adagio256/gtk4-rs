// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, ShortcutAction};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkSignalAction")]
    pub struct SignalAction(Object<ffi::GtkSignalAction, ffi::GtkSignalActionClass>) @extends ShortcutAction;

    match fn {
        type_ => || ffi::gtk_signal_action_get_type(),
    }
}

impl SignalAction {
    #[doc(alias = "gtk_signal_action_new")]
    pub fn new(signal_name: &str) -> SignalAction {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_signal_action_new(signal_name.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_signal_action_get_signal_name")]
    #[doc(alias = "get_signal_name")]
    #[doc(alias = "signal-name")]
    pub fn signal_name(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gtk_signal_action_get_signal_name(
                self.to_glib_none().0,
            ))
        }
    }
}
