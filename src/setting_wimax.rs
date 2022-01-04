// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingWimax")]
    pub struct SettingWimax(Object<ffi::NMSettingWimax, ffi::NMSettingWimaxClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_wimax_get_type(),
    }
}

impl SettingWimax {
    /// Creates a new [`SettingWimax`][crate::SettingWimax] object with default values.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingWimax`][crate::SettingWimax] object
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_setting_wimax_new")]
    pub fn new() -> SettingWimax {
        unsafe { Setting::from_glib_full(ffi::nm_setting_wimax_new()).unsafe_cast() }
    }

    /// Returns the MAC address of a WiMAX device which this connection is locked
    /// to.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the MAC address
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_setting_wimax_get_mac_address")]
    #[doc(alias = "get_mac_address")]
    pub fn mac_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_wimax_get_mac_address(self.to_glib_none().0)) }
    }

    /// Returns the WiMAX NSP name (ex "Sprint" or "CLEAR") which identifies the
    /// specific WiMAX network this setting describes a connection to.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the WiMAX NSP name
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_setting_wimax_get_network_name")]
    #[doc(alias = "get_network_name")]
    pub fn network_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_wimax_get_network_name(
                self.to_glib_none().0,
            ))
        }
    }

    /// If specified, this connection will only apply to the WiMAX device whose
    /// MAC address matches. This property does not change the MAC address of the
    /// device (known as MAC spoofing).
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "mac-address")]
    pub fn set_mac_address(&self, mac_address: Option<&str>) {
        glib::ObjectExt::set_property(self, "mac-address", &mac_address)
    }

    /// Network Service Provider (NSP) name of the WiMAX network this connection
    /// should use.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "network-name")]
    pub fn set_network_name(&self, network_name: Option<&str>) {
        glib::ObjectExt::set_property(self, "network-name", &network_name)
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "mac-address")]
    pub fn connect_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_trampoline<F: Fn(&SettingWimax) + 'static>(
            this: *mut ffi::NMSettingWimax,
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
                b"notify::mac-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "network-name")]
    pub fn connect_network_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_network_name_trampoline<F: Fn(&SettingWimax) + 'static>(
            this: *mut ffi::NMSettingWimax,
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
                b"notify::network-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_network_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingWimax {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingWimax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingWimax")
    }
}
