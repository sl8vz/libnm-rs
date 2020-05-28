// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Setting;

glib_wrapper! {
    pub struct SettingBond(Object<nm_sys::NMSettingBond, nm_sys::NMSettingBondClass, SettingBondClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_bond_get_type(),
    }
}

impl SettingBond {
    pub fn new() -> SettingBond {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_bond_new()).unsafe_cast() }
    }

    pub fn validate_option(name: &str, value: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_bond_validate_option(
                name.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }
}

impl Default for SettingBond {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_BOND: Option<&SettingBond> = None;

pub trait SettingBondExt: 'static {
    fn add_option(&self, name: &str, value: &str) -> bool;

    fn get_num_options(&self) -> u32;

    fn get_option_by_name(&self, name: &str) -> Option<GString>;

    fn get_option_default(&self, name: &str) -> Option<GString>;

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn get_option_normalized(&self, name: &str) -> Option<GString>;

    fn get_valid_options(&self) -> Vec<GString>;

    fn remove_option(&self, name: &str) -> bool;

    //fn get_property_options(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 };

    //fn set_property_options(&self, options: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 });

    fn connect_property_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingBond>> SettingBondExt for O {
    fn add_option(&self, name: &str, value: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_bond_add_option(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn get_num_options(&self) -> u32 {
        unsafe { nm_sys::nm_setting_bond_get_num_options(self.as_ref().to_glib_none().0) }
    }

    fn get_option_by_name(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_bond_get_option_by_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn get_option_default(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_bond_get_option_default(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn get_option_normalized(&self, name: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_bond_get_option_normalized(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn get_valid_options(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(nm_sys::nm_setting_bond_get_valid_options(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_option(&self, name: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_bond_remove_option(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    //fn get_property_options(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"options\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `options` getter").unwrap()
    //    }
    //}

    //fn set_property_options(&self, options: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"options\0".as_ptr() as *const _, Value::from(&options).to_glib_none().0);
    //    }
    //}

    fn connect_property_options_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_options_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingBond,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingBond>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingBond::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::options\0".as_ptr() as *const _,
                Some(transmute(notify_options_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingBond {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingBond")
    }
}
