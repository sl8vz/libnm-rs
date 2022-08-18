// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::Device;
use crate::Object;
use crate::WimaxNsp;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "NMDeviceWimax")]
    pub struct DeviceWimax(Object<ffi::NMDeviceWimax, ffi::NMDeviceWimaxClass>) @extends Device, Object;

    match fn {
        type_ => || ffi::nm_device_wimax_get_type(),
    }
}

impl DeviceWimax {
    /// Gets the active [`WimaxNsp`][crate::WimaxNsp].
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the access point or [`None`] if none is active
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_active_nsp")]
    #[doc(alias = "get_active_nsp")]
    pub fn active_nsp(&self) -> Option<WimaxNsp> {
        unsafe { from_glib_full(ffi::nm_device_wimax_get_active_nsp(self.to_glib_none().0)) }
    }

    /// Gets the ID of the serving Base Station when the device is connected.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the ID of the serving Base Station, or [`None`]
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_bsid")]
    #[doc(alias = "get_bsid")]
    pub fn bsid(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_wimax_get_bsid(self.to_glib_none().0)) }
    }

    /// Gets the center frequency (in KHz) of the radio channel the device is using
    /// to communicate with the network when connected. Has no meaning when the
    /// device is not connected.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the center frequency in KHz, or 0
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_center_frequency")]
    #[doc(alias = "get_center_frequency")]
    pub fn center_frequency(&self) -> u32 {
        unsafe { ffi::nm_device_wimax_get_center_frequency(self.to_glib_none().0) }
    }

    /// Gets the CINR (Carrier to Interference + Noise Ratio) of the current radio
    /// link in dB. CINR is a more accurate measure of radio link quality. Has no
    /// meaning when the device is not connected.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the CINR in dB, or 0
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_cinr")]
    #[doc(alias = "get_cinr")]
    pub fn cinr(&self) -> i32 {
        unsafe { ffi::nm_device_wimax_get_cinr(self.to_glib_none().0) }
    }

    /// Gets the hardware (MAC) address of the [`DeviceWimax`][crate::DeviceWimax]
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the hardware address. This is the internal string used by the
    ///  device, and must not be modified.
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_hw_address")]
    #[doc(alias = "get_hw_address")]
    pub fn hw_address(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_device_wimax_get_hw_address(self.to_glib_none().0)) }
    }

    /// Gets a [`WimaxNsp`][crate::WimaxNsp] by path.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    /// ## `path`
    /// the object path of the NSP
    ///
    /// # Returns
    ///
    /// the access point or [`None`] if none is found.
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_nsp_by_path")]
    #[doc(alias = "get_nsp_by_path")]
    pub fn nsp_by_path(&self, path: &str) -> Option<WimaxNsp> {
        unsafe {
            from_glib_none(ffi::nm_device_wimax_get_nsp_by_path(
                self.to_glib_none().0,
                path.to_glib_none().0,
            ))
        }
    }

    /// Gets all the scanned NSPs of the [`DeviceWimax`][crate::DeviceWimax].
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// a [`glib::PtrArray`][crate::glib::PtrArray] containing
    ///  all the scanned `NMWimaxNsps`.
    /// The returned array is owned by the client and should not be modified.
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_nsps")]
    #[doc(alias = "get_nsps")]
    pub fn nsps(&self) -> Vec<WimaxNsp> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::nm_device_wimax_get_nsps(
                self.to_glib_none().0,
            ))
        }
    }

    /// Gets the RSSI of the current radio link in dBm. This value indicates how
    /// strong the raw received RF signal from the base station is, but does not
    /// indicate the overall quality of the radio link. Has no meaning when the
    /// device is not connected.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the RSSI in dBm, or 0
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_rssi")]
    #[doc(alias = "get_rssi")]
    pub fn rssi(&self) -> i32 {
        unsafe { ffi::nm_device_wimax_get_rssi(self.to_glib_none().0) }
    }

    /// Average power of the last burst transmitted by the device, in units of
    /// 0.5 dBm. i.e. a TxPower of -11 represents an actual device TX power of
    /// -5.5 dBm. Has no meaning when the device is not connected.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    ///
    /// # Returns
    ///
    /// the TX power in dBm, or 0
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nm_device_wimax_get_tx_power")]
    #[doc(alias = "get_tx_power")]
    pub fn tx_power(&self) -> i32 {
        unsafe { ffi::nm_device_wimax_get_tx_power(self.to_glib_none().0) }
    }

    /// Notifies that a [`WimaxNsp`][crate::WimaxNsp] is added to the wimax device.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    /// ## `nsp`
    /// the new NSP
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nsp-added")]
    pub fn connect_nsp_added<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn nsp_added_trampoline<F: Fn(&DeviceWimax, &glib::Object) + 'static>(
            this: *mut ffi::NMDeviceWimax,
            nsp: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(nsp))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"nsp-added\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    nsp_added_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    /// Notifies that a [`WimaxNsp`][crate::WimaxNsp] is removed from the wimax device.
    ///
    /// # Deprecated since 1.2
    ///
    /// WiMAX is no longer supported.
    /// ## `nsp`
    /// the removed NSP
    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "nsp-removed")]
    pub fn connect_nsp_removed<F: Fn(&Self, &glib::Object) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn nsp_removed_trampoline<
            F: Fn(&DeviceWimax, &glib::Object) + 'static,
        >(
            this: *mut ffi::NMDeviceWimax,
            nsp: *mut glib::gobject_ffi::GObject,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), &from_glib_borrow(nsp))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"nsp-removed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    nsp_removed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "active-nsp")]
    pub fn connect_active_nsp_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_active_nsp_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::active-nsp\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_active_nsp_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "bsid")]
    pub fn connect_bsid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_bsid_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::bsid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_bsid_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "center-frequency")]
    pub fn connect_center_frequency_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_center_frequency_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::center-frequency\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_center_frequency_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "cinr")]
    pub fn connect_cinr_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_cinr_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::cinr\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_cinr_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "nsps")]
    pub fn connect_nsps_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_nsps_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::nsps\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_nsps_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "rssi")]
    pub fn connect_rssi_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rssi_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::rssi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rssi_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v1_2", deprecated = "Since 1.2")]
    #[doc(alias = "tx-power")]
    pub fn connect_tx_power_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tx_power_trampoline<F: Fn(&DeviceWimax) + 'static>(
            this: *mut ffi::NMDeviceWimax,
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
                b"notify::tx-power\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tx_power_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceWimax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DeviceWimax")
    }
}
