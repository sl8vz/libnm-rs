// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
use crate::SettingSecretFlags;
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
    #[doc(alias = "NMSettingCdma")]
    pub struct SettingCdma(Object<ffi::NMSettingCdma, ffi::NMSettingCdmaClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_cdma_get_type(),
    }
}

impl SettingCdma {
    /// Creates a new [`SettingCdma`][crate::SettingCdma] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingCdma`][crate::SettingCdma] object
    #[doc(alias = "nm_setting_cdma_new")]
    pub fn new() -> SettingCdma {
        unsafe { Setting::from_glib_full(ffi::nm_setting_cdma_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingCdma::mtu` property of the setting
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    #[doc(alias = "nm_setting_cdma_get_mtu")]
    #[doc(alias = "get_mtu")]
    pub fn mtu(&self) -> u32 {
        unsafe { ffi::nm_setting_cdma_get_mtu(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingCdma::number` property of the setting
    #[doc(alias = "nm_setting_cdma_get_number")]
    #[doc(alias = "get_number")]
    pub fn number(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_cdma_get_number(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingCdma::password` property of the setting
    #[doc(alias = "nm_setting_cdma_get_password")]
    #[doc(alias = "get_password")]
    pub fn password(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_cdma_get_password(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the [`SettingSecretFlags`][crate::SettingSecretFlags] pertaining to the `property::SettingCdma::password`
    #[doc(alias = "nm_setting_cdma_get_password_flags")]
    #[doc(alias = "get_password_flags")]
    pub fn password_flags(&self) -> SettingSecretFlags {
        unsafe {
            from_glib(ffi::nm_setting_cdma_get_password_flags(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingCdma::username` property of the setting
    #[doc(alias = "nm_setting_cdma_get_username")]
    #[doc(alias = "get_username")]
    pub fn username(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_cdma_get_username(self.to_glib_none().0)) }
    }

    /// If non-zero, only transmit packets of the specified size or smaller,
    /// breaking larger packets up into multiple frames.
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    pub fn set_mtu(&self, mtu: u32) {
        glib::ObjectExt::set_property(self, "mtu", &mtu)
    }

    /// The number to dial to establish the connection to the CDMA-based mobile
    /// broadband network, if any. If not specified, the default number (`777`)
    /// is used when required.
    pub fn set_number(&self, number: Option<&str>) {
        glib::ObjectExt::set_property(self, "number", &number)
    }

    /// The password used to authenticate with the network, if required. Many
    /// providers do not require a password, or accept any password. But if a
    /// password is required, it is specified here.
    pub fn set_password(&self, password: Option<&str>) {
        glib::ObjectExt::set_property(self, "password", &password)
    }

    /// Flags indicating how to handle the `property::SettingCdma::password` property.
    #[doc(alias = "password-flags")]
    pub fn set_password_flags(&self, password_flags: SettingSecretFlags) {
        glib::ObjectExt::set_property(self, "password-flags", &password_flags)
    }

    /// The username used to authenticate with the network, if required. Many
    /// providers do not require a username, or accept any username. But if a
    /// username is required, it is specified here.
    pub fn set_username(&self, username: Option<&str>) {
        glib::ObjectExt::set_property(self, "username", &username)
    }

    #[cfg(any(feature = "v1_8", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_8")))]
    #[doc(alias = "mtu")]
    pub fn connect_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mtu_trampoline<F: Fn(&SettingCdma) + 'static>(
            this: *mut ffi::NMSettingCdma,
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
                b"notify::mtu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mtu_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "number")]
    pub fn connect_number_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_number_trampoline<F: Fn(&SettingCdma) + 'static>(
            this: *mut ffi::NMSettingCdma,
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
                b"notify::number\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_number_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "password")]
    pub fn connect_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_trampoline<F: Fn(&SettingCdma) + 'static>(
            this: *mut ffi::NMSettingCdma,
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
                b"notify::password\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "password-flags")]
    pub fn connect_password_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_password_flags_trampoline<F: Fn(&SettingCdma) + 'static>(
            this: *mut ffi::NMSettingCdma,
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
                b"notify::password-flags\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_password_flags_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "username")]
    pub fn connect_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_username_trampoline<F: Fn(&SettingCdma) + 'static>(
            this: *mut ffi::NMSettingCdma,
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
                b"notify::username\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_username_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for SettingCdma {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingCdma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingCdma")
    }
}
