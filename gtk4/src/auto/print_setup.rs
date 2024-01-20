// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{PageSetup, PrintSettings};
use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct PrintSetup(Shared<ffi::GtkPrintSetup>);

    match fn {
        ref => |ptr| ffi::gtk_print_setup_ref(ptr),
        unref => |ptr| ffi::gtk_print_setup_unref(ptr),
        type_ => || ffi::gtk_print_setup_get_type(),
    }
}

impl PrintSetup {
    #[doc(alias = "gtk_print_setup_get_page_setup")]
    #[doc(alias = "get_page_setup")]
    pub fn page_setup(&self) -> Option<PageSetup> {
        unsafe { from_glib_none(ffi::gtk_print_setup_get_page_setup(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_print_setup_get_print_settings")]
    #[doc(alias = "get_print_settings")]
    pub fn print_settings(&self) -> Option<PrintSettings> {
        unsafe {
            from_glib_none(ffi::gtk_print_setup_get_print_settings(
                self.to_glib_none().0,
            ))
        }
    }
}