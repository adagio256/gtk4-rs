// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::WaylandSurface;
use std::fmt;

glib::wrapper! {
    pub struct WaylandPopup(Object<ffi::GdkWaylandPopup>) @extends WaylandSurface, gdk::Surface, @implements gdk::Popup;

    match fn {
        type_ => || ffi::gdk_wayland_popup_get_type(),
    }
}

impl WaylandPopup {}

impl fmt::Display for WaylandPopup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("WaylandPopup")
    }
}
