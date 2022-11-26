// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ColumnView;
use crate::ListItemFactory;
use crate::Sorter;
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
    #[doc(alias = "GtkColumnViewColumn")]
    pub struct ColumnViewColumn(Object<ffi::GtkColumnViewColumn, ffi::GtkColumnViewColumnClass>);

    match fn {
        type_ => || ffi::gtk_column_view_column_get_type(),
    }
}

impl ColumnViewColumn {
    #[doc(alias = "gtk_column_view_column_new")]
    pub fn new(
        title: Option<&str>,
        factory: Option<&impl IsA<ListItemFactory>>,
    ) -> ColumnViewColumn {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_column_view_column_new(
                title.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ColumnViewColumn`] objects.
    ///
    /// This method returns an instance of [`ColumnViewColumnBuilder`](crate::builders::ColumnViewColumnBuilder) which can be used to create [`ColumnViewColumn`] objects.
    pub fn builder() -> ColumnViewColumnBuilder {
        ColumnViewColumnBuilder::default()
    }

    #[doc(alias = "gtk_column_view_column_get_column_view")]
    #[doc(alias = "get_column_view")]
    pub fn column_view(&self) -> Option<ColumnView> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_column_view(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_column_get_expand")]
    #[doc(alias = "get_expand")]
    pub fn expands(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_column_get_expand(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_column_get_factory")]
    #[doc(alias = "get_factory")]
    pub fn factory(&self) -> Option<ListItemFactory> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_factory(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_column_get_fixed_width")]
    #[doc(alias = "get_fixed_width")]
    pub fn fixed_width(&self) -> i32 {
        unsafe { ffi::gtk_column_view_column_get_fixed_width(self.to_glib_none().0) }
    }

    #[doc(alias = "gtk_column_view_column_get_header_menu")]
    #[doc(alias = "get_header_menu")]
    pub fn header_menu(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_header_menu(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_column_view_column_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_column_view_column_get_id(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_column_view_column_get_resizable")]
    #[doc(alias = "get_resizable")]
    pub fn is_resizable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_column_get_resizable(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_column_get_sorter")]
    #[doc(alias = "get_sorter")]
    pub fn sorter(&self) -> Option<Sorter> {
        unsafe {
            from_glib_none(ffi::gtk_column_view_column_get_sorter(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_column_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_column_view_column_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_column_view_column_get_visible")]
    #[doc(alias = "get_visible")]
    pub fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_column_view_column_get_visible(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_column_view_column_set_expand")]
    pub fn set_expand(&self, expand: bool) {
        unsafe {
            ffi::gtk_column_view_column_set_expand(self.to_glib_none().0, expand.into_glib());
        }
    }

    #[doc(alias = "gtk_column_view_column_set_factory")]
    pub fn set_factory(&self, factory: Option<&impl IsA<ListItemFactory>>) {
        unsafe {
            ffi::gtk_column_view_column_set_factory(
                self.to_glib_none().0,
                factory.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_column_view_column_set_fixed_width")]
    pub fn set_fixed_width(&self, fixed_width: i32) {
        unsafe {
            ffi::gtk_column_view_column_set_fixed_width(self.to_glib_none().0, fixed_width);
        }
    }

    #[doc(alias = "gtk_column_view_column_set_header_menu")]
    pub fn set_header_menu(&self, menu: Option<&impl IsA<gio::MenuModel>>) {
        unsafe {
            ffi::gtk_column_view_column_set_header_menu(
                self.to_glib_none().0,
                menu.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "gtk_column_view_column_set_id")]
    pub fn set_id(&self, id: Option<&str>) {
        unsafe {
            ffi::gtk_column_view_column_set_id(self.to_glib_none().0, id.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_column_view_column_set_resizable")]
    pub fn set_resizable(&self, resizable: bool) {
        unsafe {
            ffi::gtk_column_view_column_set_resizable(self.to_glib_none().0, resizable.into_glib());
        }
    }

    #[doc(alias = "gtk_column_view_column_set_sorter")]
    pub fn set_sorter(&self, sorter: Option<&impl IsA<Sorter>>) {
        unsafe {
            ffi::gtk_column_view_column_set_sorter(
                self.to_glib_none().0,
                sorter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_column_view_column_set_title")]
    pub fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::gtk_column_view_column_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_column_view_column_set_visible")]
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_column_view_column_set_visible(self.to_glib_none().0, visible.into_glib());
        }
    }

    #[doc(alias = "column-view")]
    pub fn connect_column_view_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_column_view_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::column-view\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_column_view_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "expand")]
    pub fn connect_expand_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expand_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::expand\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expand_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "factory")]
    pub fn connect_factory_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_factory_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::factory\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_factory_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "fixed-width")]
    pub fn connect_fixed_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_fixed_width_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::fixed-width\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_fixed_width_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "header-menu")]
    pub fn connect_header_menu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_header_menu_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::header-menu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_header_menu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "id")]
    pub fn connect_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resizable")]
    pub fn connect_resizable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_resizable_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::resizable\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_resizable_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "sorter")]
    pub fn connect_sorter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sorter_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::sorter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sorter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    pub fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&ColumnViewColumn) + 'static>(
            this: *mut ffi::GtkColumnViewColumn,
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
                b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for ColumnViewColumn {
    fn default() -> Self {
        glib::object::Object::new::<Self>(&[])
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ColumnViewColumn`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ColumnViewColumnBuilder {
    expand: Option<bool>,
    factory: Option<ListItemFactory>,
    fixed_width: Option<i32>,
    header_menu: Option<gio::MenuModel>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    id: Option<String>,
    resizable: Option<bool>,
    sorter: Option<Sorter>,
    title: Option<String>,
    visible: Option<bool>,
}

impl ColumnViewColumnBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ColumnViewColumnBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ColumnViewColumn`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ColumnViewColumn {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        if let Some(ref factory) = self.factory {
            properties.push(("factory", factory));
        }
        if let Some(ref fixed_width) = self.fixed_width {
            properties.push(("fixed-width", fixed_width));
        }
        if let Some(ref header_menu) = self.header_menu {
            properties.push(("header-menu", header_menu));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref id) = self.id {
            properties.push(("id", id));
        }
        if let Some(ref resizable) = self.resizable {
            properties.push(("resizable", resizable));
        }
        if let Some(ref sorter) = self.sorter {
            properties.push(("sorter", sorter));
        }
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        glib::Object::new::<ColumnViewColumn>(&properties)
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    pub fn factory(mut self, factory: &impl IsA<ListItemFactory>) -> Self {
        self.factory = Some(factory.clone().upcast());
        self
    }

    pub fn fixed_width(mut self, fixed_width: i32) -> Self {
        self.fixed_width = Some(fixed_width);
        self
    }

    pub fn header_menu(mut self, header_menu: &impl IsA<gio::MenuModel>) -> Self {
        self.header_menu = Some(header_menu.clone().upcast());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = Some(resizable);
        self
    }

    pub fn sorter(mut self, sorter: &impl IsA<Sorter>) -> Self {
        self.sorter = Some(sorter.clone().upcast());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }
}

impl fmt::Display for ColumnViewColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ColumnViewColumn")
    }
}
