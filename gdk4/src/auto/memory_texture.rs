// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::MemoryFormat;
use crate::Paintable;
use crate::Texture;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct MemoryTexture(Object<ffi::GdkMemoryTexture, ffi::GdkMemoryTextureClass>) @extends Texture, @implements Paintable;

    match fn {
        type_ => || ffi::gdk_memory_texture_get_type(),
    }
}

impl MemoryTexture {
    #[doc(alias = "gdk_memory_texture_new")]
    pub fn new(
        width: i32,
        height: i32,
        format: MemoryFormat,
        bytes: &glib::Bytes,
        stride: usize,
    ) -> MemoryTexture {
        assert_initialized_main_thread!();
        unsafe {
            Texture::from_glib_full(ffi::gdk_memory_texture_new(
                width,
                height,
                format.to_glib(),
                bytes.to_glib_none().0,
                stride,
            ))
            .unsafe_cast()
        }
    }
}

impl fmt::Display for MemoryTexture {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MemoryTexture")
    }
}
