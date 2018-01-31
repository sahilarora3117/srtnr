// This file was generated by gir (d933f9a) from gir-files (469db10)
// DO NOT EDIT

use Buildable;
use Container;
use MenuShell;
use PackDirection;
use Widget;
use ffi;
use gio;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct MenuBar(Object<ffi::GtkMenuBar, ffi::GtkMenuBarClass>): MenuShell, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_menu_bar_get_type(),
    }
}

impl MenuBar {
    pub fn new() -> MenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_bar_new()).downcast_unchecked()
        }
    }

    pub fn new_from_model<P: IsA<gio::MenuModel>>(model: &P) -> MenuBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_bar_new_from_model(model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

impl Default for MenuBar {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MenuBarExt {
    fn get_child_pack_direction(&self) -> PackDirection;

    fn get_pack_direction(&self) -> PackDirection;

    fn set_child_pack_direction(&self, child_pack_dir: PackDirection);

    fn set_pack_direction(&self, pack_dir: PackDirection);

    fn connect_property_child_pack_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pack_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MenuBar> + IsA<glib::object::Object>> MenuBarExt for O {
    fn get_child_pack_direction(&self) -> PackDirection {
        unsafe {
            from_glib(ffi::gtk_menu_bar_get_child_pack_direction(self.to_glib_none().0))
        }
    }

    fn get_pack_direction(&self) -> PackDirection {
        unsafe {
            from_glib(ffi::gtk_menu_bar_get_pack_direction(self.to_glib_none().0))
        }
    }

    fn set_child_pack_direction(&self, child_pack_dir: PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_child_pack_direction(self.to_glib_none().0, child_pack_dir.to_glib());
        }
    }

    fn set_pack_direction(&self, pack_dir: PackDirection) {
        unsafe {
            ffi::gtk_menu_bar_set_pack_direction(self.to_glib_none().0, pack_dir.to_glib());
        }
    }

    fn connect_property_child_pack_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::child-pack-direction",
                transmute(notify_child_pack_direction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_pack_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::pack-direction",
                transmute(notify_pack_direction_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_child_pack_direction_trampoline<P>(this: *mut ffi::GtkMenuBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuBar::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_pack_direction_trampoline<P>(this: *mut ffi::GtkMenuBar, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MenuBar> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MenuBar::from_glib_borrow(this).downcast_unchecked())
}