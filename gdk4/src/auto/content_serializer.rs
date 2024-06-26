// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkContentSerializer")]
    pub struct ContentSerializer(Object<ffi::GdkContentSerializer>);

    match fn {
        type_ => || ffi::gdk_content_serializer_get_type(),
    }
}

impl ContentSerializer {
    #[doc(alias = "gdk_content_serializer_get_cancellable")]
    #[doc(alias = "get_cancellable")]
    pub fn cancellable(&self) -> Option<gio::Cancellable> {
        unsafe {
            from_glib_none(ffi::gdk_content_serializer_get_cancellable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_serializer_get_gtype")]
    #[doc(alias = "get_gtype")]
    pub fn type_(&self) -> glib::types::Type {
        unsafe { from_glib(ffi::gdk_content_serializer_get_gtype(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_content_serializer_get_mime_type")]
    #[doc(alias = "get_mime_type")]
    pub fn mime_type(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::gdk_content_serializer_get_mime_type(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_serializer_get_output_stream")]
    #[doc(alias = "get_output_stream")]
    pub fn output_stream(&self) -> gio::OutputStream {
        unsafe {
            from_glib_none(ffi::gdk_content_serializer_get_output_stream(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_content_serializer_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self) -> glib::Value {
        unsafe { from_glib_none(ffi::gdk_content_serializer_get_value(self.to_glib_none().0)) }
    }

    #[doc(alias = "gdk_content_serializer_return_success")]
    pub fn return_success(&self) {
        unsafe {
            ffi::gdk_content_serializer_return_success(self.to_glib_none().0);
        }
    }
}
