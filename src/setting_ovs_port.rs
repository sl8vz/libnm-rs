// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Setting;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::object::Cast;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::translate::*;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use glib::ToValue;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMSettingOvsPort")]
    pub struct SettingOvsPort(Object<ffi::NMSettingOvsPort, ffi::NMSettingOvsPortClass>) @extends Setting;

    match fn {
        type_ => || ffi::nm_setting_ovs_port_get_type(),
    }
}

impl SettingOvsPort {
    /// Creates a new [`SettingOvsPort`][crate::SettingOvsPort] object with default values.
    ///
    /// # Returns
    ///
    /// the new empty [`SettingOvsPort`][crate::SettingOvsPort] object
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_new")]
    pub fn new() -> SettingOvsPort {
        unsafe { Setting::from_glib_full(ffi::nm_setting_ovs_port_new()).unsafe_cast() }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsPort::bond-downdelay` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_get_bond_downdelay")]
    #[doc(alias = "get_bond_downdelay")]
    pub fn bond_downdelay(&self) -> u32 {
        unsafe { ffi::nm_setting_ovs_port_get_bond_downdelay(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsPort::bond-mode` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_get_bond_mode")]
    #[doc(alias = "get_bond_mode")]
    pub fn bond_mode(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_ovs_port_get_bond_mode(
                self.to_glib_none().0,
            ))
        }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsPort::bond-updelay` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_get_bond_updelay")]
    #[doc(alias = "get_bond_updelay")]
    pub fn bond_updelay(&self) -> u32 {
        unsafe { ffi::nm_setting_ovs_port_get_bond_updelay(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsPort::lacp` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_get_lacp")]
    #[doc(alias = "get_lacp")]
    pub fn lacp(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_setting_ovs_port_get_lacp(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsPort::tag` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_get_tag")]
    #[doc(alias = "get_tag")]
    pub fn tag(&self) -> u32 {
        unsafe { ffi::nm_setting_ovs_port_get_tag(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the `property::SettingOvsPort::vlan-mode` property of the setting
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "nm_setting_ovs_port_get_vlan_mode")]
    #[doc(alias = "get_vlan_mode")]
    pub fn vlan_mode(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::nm_setting_ovs_port_get_vlan_mode(
                self.to_glib_none().0,
            ))
        }
    }

    /// The time port must be inactive in order to be considered down.
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "bond-downdelay")]
    pub fn set_bond_downdelay(&self, bond_downdelay: u32) {
        glib::ObjectExt::set_property(self, "bond-downdelay", &bond_downdelay)
    }

    /// Bonding mode. One of "active-backup", "balance-slb", or "balance-tcp".
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "bond-mode")]
    pub fn set_bond_mode(&self, bond_mode: Option<&str>) {
        glib::ObjectExt::set_property(self, "bond-mode", &bond_mode)
    }

    /// The time port must be active before it starts forwarding traffic.
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "bond-updelay")]
    pub fn set_bond_updelay(&self, bond_updelay: u32) {
        glib::ObjectExt::set_property(self, "bond-updelay", &bond_updelay)
    }

    /// LACP mode. One of "active", "off", or "passive".
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn set_lacp(&self, lacp: Option<&str>) {
        glib::ObjectExt::set_property(self, "lacp", &lacp)
    }

    /// The VLAN tag in the range 0-4095.
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    pub fn set_tag(&self, tag: u32) {
        glib::ObjectExt::set_property(self, "tag", &tag)
    }

    /// The VLAN mode. One of "access", "native-tagged", "native-untagged",
    /// "trunk" or unset.
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "vlan-mode")]
    pub fn set_vlan_mode(&self, vlan_mode: Option<&str>) {
        glib::ObjectExt::set_property(self, "vlan-mode", &vlan_mode)
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "bond-downdelay")]
    pub fn connect_bond_downdelay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bond_downdelay_trampoline<F: Fn(&SettingOvsPort) + 'static>(
            this: *mut ffi::NMSettingOvsPort,
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
                b"notify::bond-downdelay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bond_downdelay_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "bond-mode")]
    pub fn connect_bond_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bond_mode_trampoline<F: Fn(&SettingOvsPort) + 'static>(
            this: *mut ffi::NMSettingOvsPort,
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
                b"notify::bond-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bond_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "bond-updelay")]
    pub fn connect_bond_updelay_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bond_updelay_trampoline<F: Fn(&SettingOvsPort) + 'static>(
            this: *mut ffi::NMSettingOvsPort,
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
                b"notify::bond-updelay\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bond_updelay_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "lacp")]
    pub fn connect_lacp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_lacp_trampoline<F: Fn(&SettingOvsPort) + 'static>(
            this: *mut ffi::NMSettingOvsPort,
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
                b"notify::lacp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_lacp_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "tag")]
    pub fn connect_tag_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tag_trampoline<F: Fn(&SettingOvsPort) + 'static>(
            this: *mut ffi::NMSettingOvsPort,
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
                b"notify::tag\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tag_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
    #[doc(alias = "vlan-mode")]
    pub fn connect_vlan_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_vlan_mode_trampoline<F: Fn(&SettingOvsPort) + 'static>(
            this: *mut ffi::NMSettingOvsPort,
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
                b"notify::vlan-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vlan_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_10")))]
impl Default for SettingOvsPort {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for SettingOvsPort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingOvsPort")
    }
}
