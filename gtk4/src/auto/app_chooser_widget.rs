// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Accessible;
use crate::AccessibleRole;
use crate::Align;
use crate::AppChooser;
use crate::Buildable;
use crate::ConstraintTarget;
use crate::LayoutManager;
use crate::Overflow;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct AppChooserWidget(Object<ffi::GtkAppChooserWidget>) @extends Widget, @implements Accessible, Buildable, ConstraintTarget, AppChooser;

    match fn {
        type_ => || ffi::gtk_app_chooser_widget_get_type(),
    }
}

impl AppChooserWidget {
    #[doc(alias = "gtk_app_chooser_widget_new")]
    pub fn new(content_type: &str) -> AppChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_widget_new(
                content_type.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_get_default_text")]
    pub fn default_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_widget_get_default_text(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_get_show_all")]
    pub fn shows_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_all(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_get_show_default")]
    pub fn shows_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_default(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_get_show_fallback")]
    pub fn shows_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_fallback(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_get_show_other")]
    pub fn shows_other(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_other(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_get_show_recommended")]
    pub fn shows_recommended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_recommended(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_set_default_text")]
    pub fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(
                self.to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_set_show_all")]
    pub fn set_show_all(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_all(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_set_show_default")]
    pub fn set_show_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_default(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_set_show_fallback")]
    pub fn set_show_fallback(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_fallback(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_set_show_other")]
    pub fn set_show_other(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_other(self.to_glib_none().0, setting.to_glib());
        }
    }

    #[doc(alias = "gtk_app_chooser_widget_set_show_recommended")]
    pub fn set_show_recommended(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_recommended(
                self.to_glib_none().0,
                setting.to_glib(),
            );
        }
    }

    pub fn connect_application_activated<F: Fn(&AppChooserWidget, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn application_activated_trampoline<
            F: Fn(&AppChooserWidget, &gio::AppInfo) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            application: *mut gio::ffi::GAppInfo,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(application))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"application-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    application_activated_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_application_selected<F: Fn(&AppChooserWidget, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn application_selected_trampoline<
            F: Fn(&AppChooserWidget, &gio::AppInfo) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            application: *mut gio::ffi::GAppInfo,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(application))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"application-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    application_selected_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_default_text_notify<F: Fn(&AppChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_text_trampoline<F: Fn(&AppChooserWidget) + 'static>(
            this: *mut ffi::GtkAppChooserWidget,
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
                b"notify::default-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_text_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_all_notify<F: Fn(&AppChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_all_trampoline<F: Fn(&AppChooserWidget) + 'static>(
            this: *mut ffi::GtkAppChooserWidget,
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
                b"notify::show-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_all_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_default_notify<F: Fn(&AppChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_default_trampoline<F: Fn(&AppChooserWidget) + 'static>(
            this: *mut ffi::GtkAppChooserWidget,
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
                b"notify::show-default\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_default_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_fallback_notify<F: Fn(&AppChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_fallback_trampoline<F: Fn(&AppChooserWidget) + 'static>(
            this: *mut ffi::GtkAppChooserWidget,
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
                b"notify::show-fallback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_fallback_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_other_notify<F: Fn(&AppChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_other_trampoline<F: Fn(&AppChooserWidget) + 'static>(
            this: *mut ffi::GtkAppChooserWidget,
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
                b"notify::show-other\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_other_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_show_recommended_notify<F: Fn(&AppChooserWidget) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_recommended_trampoline<
            F: Fn(&AppChooserWidget) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
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
                b"notify::show-recommended\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_recommended_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct AppChooserWidgetBuilder {
    default_text: Option<String>,
    show_all: Option<bool>,
    show_default: Option<bool>,
    show_fallback: Option<bool>,
    show_other: Option<bool>,
    show_recommended: Option<bool>,
    can_focus: Option<bool>,
    can_target: Option<bool>,
    css_classes: Option<Vec<String>>,
    css_name: Option<String>,
    cursor: Option<gdk::Cursor>,
    focus_on_click: Option<bool>,
    focusable: Option<bool>,
    halign: Option<Align>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    layout_manager: Option<LayoutManager>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    opacity: Option<f64>,
    overflow: Option<Overflow>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    accessible_role: Option<AccessibleRole>,
    content_type: Option<String>,
}

impl AppChooserWidgetBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> AppChooserWidget {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref default_text) = self.default_text {
            properties.push(("default-text", default_text));
        }
        if let Some(ref show_all) = self.show_all {
            properties.push(("show-all", show_all));
        }
        if let Some(ref show_default) = self.show_default {
            properties.push(("show-default", show_default));
        }
        if let Some(ref show_fallback) = self.show_fallback {
            properties.push(("show-fallback", show_fallback));
        }
        if let Some(ref show_other) = self.show_other {
            properties.push(("show-other", show_other));
        }
        if let Some(ref show_recommended) = self.show_recommended {
            properties.push(("show-recommended", show_recommended));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref can_target) = self.can_target {
            properties.push(("can-target", can_target));
        }
        if let Some(ref css_classes) = self.css_classes {
            properties.push(("css-classes", css_classes));
        }
        if let Some(ref css_name) = self.css_name {
            properties.push(("css-name", css_name));
        }
        if let Some(ref cursor) = self.cursor {
            properties.push(("cursor", cursor));
        }
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref focusable) = self.focusable {
            properties.push(("focusable", focusable));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref layout_manager) = self.layout_manager {
            properties.push(("layout-manager", layout_manager));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref overflow) = self.overflow {
            properties.push(("overflow", overflow));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref accessible_role) = self.accessible_role {
            properties.push(("accessible-role", accessible_role));
        }
        if let Some(ref content_type) = self.content_type {
            properties.push(("content-type", content_type));
        }
        let ret = glib::Object::new::<AppChooserWidget>(&properties).expect("object new");
        ret
    }

    pub fn default_text(mut self, default_text: &str) -> Self {
        self.default_text = Some(default_text.to_string());
        self
    }

    pub fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = Some(show_all);
        self
    }

    pub fn show_default(mut self, show_default: bool) -> Self {
        self.show_default = Some(show_default);
        self
    }

    pub fn show_fallback(mut self, show_fallback: bool) -> Self {
        self.show_fallback = Some(show_fallback);
        self
    }

    pub fn show_other(mut self, show_other: bool) -> Self {
        self.show_other = Some(show_other);
        self
    }

    pub fn show_recommended(mut self, show_recommended: bool) -> Self {
        self.show_recommended = Some(show_recommended);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn can_target(mut self, can_target: bool) -> Self {
        self.can_target = Some(can_target);
        self
    }

    pub fn css_classes(mut self, css_classes: Vec<String>) -> Self {
        self.css_classes = Some(css_classes);
        self
    }

    pub fn css_name(mut self, css_name: &str) -> Self {
        self.css_name = Some(css_name.to_string());
        self
    }

    pub fn cursor(mut self, cursor: &gdk::Cursor) -> Self {
        self.cursor = Some(cursor.clone());
        self
    }

    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn focusable(mut self, focusable: bool) -> Self {
        self.focusable = Some(focusable);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn layout_manager<P: IsA<LayoutManager>>(mut self, layout_manager: &P) -> Self {
        self.layout_manager = Some(layout_manager.clone().upcast());
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn overflow(mut self, overflow: Overflow) -> Self {
        self.overflow = Some(overflow);
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn accessible_role(mut self, accessible_role: AccessibleRole) -> Self {
        self.accessible_role = Some(accessible_role);
        self
    }

    pub fn content_type(mut self, content_type: &str) -> Self {
        self.content_type = Some(content_type.to_string());
        self
    }
}

impl fmt::Display for AppChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooserWidget")
    }
}
