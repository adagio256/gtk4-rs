// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct ATContext(Object<ffi::GtkATContext, ffi::GtkATContextClass>);

    match fn {
        type_ => || ffi::gtk_at_context_get_type(),
    }
}

impl ATContext {
    #[doc(alias = "gtk_at_context_create")]
    pub fn create<P: IsA<Accessible>>(
        accessible_role: AccessibleRole,
        accessible: &P,
        display: &gdk::Display,
    ) -> Option<ATContext> {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_at_context_create(
                accessible_role.to_glib(),
                accessible.as_ref().to_glib_none().0,
                display.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_at_context_get_accessible")]
    pub fn accessible(&self) -> Option<Accessible> {
        unsafe { from_glib_none(ffi::gtk_at_context_get_accessible(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_at_context_get_accessible_role")]
    pub fn accessible_role(&self) -> AccessibleRole {
        unsafe {
            from_glib(ffi::gtk_at_context_get_accessible_role(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "set_property_accessible_role")]
    pub fn set_accessible_role(&self, accessible_role: AccessibleRole) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"accessible-role\0".as_ptr() as *const _,
                glib::Value::from(&accessible_role).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "get_property_display")]
    pub fn display(&self) -> Option<gdk::Display> {
        unsafe {
            let mut value = glib::Value::from_type(<gdk::Display as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"display\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `display` getter")
        }
    }

    #[doc(alias = "set_property_display")]
    pub fn set_display(&self, display: Option<&gdk::Display>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"display\0".as_ptr() as *const _,
                glib::Value::from(display).to_glib_none().0,
            );
        }
    }

    pub fn connect_state_change<F: Fn(&ATContext) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_change_trampoline<F: Fn(&ATContext) + 'static>(
            this: *mut ffi::GtkATContext,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-change\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_change_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_accessible_role_notify<F: Fn(&ATContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_accessible_role_trampoline<F: Fn(&ATContext) + 'static>(
            this: *mut ffi::GtkATContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accessible-role\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accessible_role_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_display_notify<F: Fn(&ATContext) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_display_trampoline<F: Fn(&ATContext) + 'static>(
            this: *mut ffi::GtkATContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::display\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_display_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ATContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ATContext")
    }
}
