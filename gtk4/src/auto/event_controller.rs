// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::PropagationLimit;
use crate::PropagationPhase;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct EventController(Object<ffi::GtkEventController, ffi::GtkEventControllerClass>);

    match fn {
        type_ => || ffi::gtk_event_controller_get_type(),
    }
}

pub const NONE_EVENT_CONTROLLER: Option<&EventController> = None;

pub trait EventControllerExt: 'static {
    #[doc(alias = "gtk_event_controller_get_current_event")]
    fn current_event(&self) -> Option<gdk::Event>;

    #[doc(alias = "gtk_event_controller_get_current_event_device")]
    fn current_event_device(&self) -> Option<gdk::Device>;

    #[doc(alias = "gtk_event_controller_get_current_event_state")]
    fn current_event_state(&self) -> gdk::ModifierType;

    #[doc(alias = "gtk_event_controller_get_current_event_time")]
    fn current_event_time(&self) -> u32;

    #[doc(alias = "gtk_event_controller_get_name")]
    fn name(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_event_controller_get_propagation_limit")]
    fn propagation_limit(&self) -> PropagationLimit;

    #[doc(alias = "gtk_event_controller_get_propagation_phase")]
    fn propagation_phase(&self) -> PropagationPhase;

    #[doc(alias = "gtk_event_controller_get_widget")]
    fn widget(&self) -> Option<Widget>;

    #[doc(alias = "gtk_event_controller_reset")]
    fn reset(&self);

    #[doc(alias = "gtk_event_controller_set_name")]
    fn set_name(&self, name: &str);

    #[doc(alias = "gtk_event_controller_set_propagation_limit")]
    fn set_propagation_limit(&self, limit: PropagationLimit);

    #[doc(alias = "gtk_event_controller_set_propagation_phase")]
    fn set_propagation_phase(&self, phase: PropagationPhase);

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_propagation_limit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_propagation_phase_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<EventController>> EventControllerExt for O {
    fn current_event(&self) -> Option<gdk::Event> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_current_event(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_event_device(&self) -> Option<gdk::Device> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_current_event_device(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_event_state(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_current_event_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn current_event_time(&self) -> u32 {
        unsafe { ffi::gtk_event_controller_get_current_event_time(self.as_ref().to_glib_none().0) }
    }

    fn name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn propagation_limit(&self) -> PropagationLimit {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_propagation_limit(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn propagation_phase(&self) -> PropagationPhase {
        unsafe {
            from_glib(ffi::gtk_event_controller_get_propagation_phase(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_event_controller_get_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn reset(&self) {
        unsafe {
            ffi::gtk_event_controller_reset(self.as_ref().to_glib_none().0);
        }
    }

    fn set_name(&self, name: &str) {
        unsafe {
            ffi::gtk_event_controller_set_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    fn set_propagation_limit(&self, limit: PropagationLimit) {
        unsafe {
            ffi::gtk_event_controller_set_propagation_limit(
                self.as_ref().to_glib_none().0,
                limit.to_glib(),
            );
        }
    }

    fn set_propagation_phase(&self, phase: PropagationPhase) {
        unsafe {
            ffi::gtk_event_controller_set_propagation_phase(
                self.as_ref().to_glib_none().0,
                phase.to_glib(),
            );
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEventController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EventController>,
        {
            let f: &F = &*(f as *const F);
            f(&EventController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_propagation_limit_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_propagation_limit_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEventController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EventController>,
        {
            let f: &F = &*(f as *const F);
            f(&EventController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::propagation-limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_propagation_limit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_propagation_phase_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_propagation_phase_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEventController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EventController>,
        {
            let f: &F = &*(f as *const F);
            f(&EventController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::propagation-phase\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_propagation_phase_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_widget_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkEventController,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<EventController>,
        {
            let f: &F = &*(f as *const F);
            f(&EventController::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for EventController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("EventController")
    }
}
