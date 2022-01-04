// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

use crate::VpnPluginFailure;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use crate::VpnServiceState;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::StaticType;
#[cfg(any(feature = "v1_2", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "NMVpnServicePlugin")]
    pub struct VpnServicePlugin(Object<ffi::NMVpnServicePlugin, ffi::NMVpnServicePluginClass>);

    match fn {
        type_ => || ffi::nm_vpn_service_plugin_get_type(),
    }
}

impl VpnServicePlugin {
    pub const NONE: Option<&'static VpnServicePlugin> = None;

    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    //#[doc(alias = "nm_vpn_service_plugin_get_secret_flags")]
    //#[doc(alias = "get_secret_flags")]
    //pub fn secret_flags(data: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }, secret_name: &str) -> Option<SettingSecretFlags> {
    //    unsafe { TODO: call ffi:nm_vpn_service_plugin_get_secret_flags() }
    //}

    //#[cfg(any(feature = "v1_2", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    //#[doc(alias = "nm_vpn_service_plugin_read_vpn_details")]
    //pub fn read_vpn_details(fd: i32, out_data: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }, out_secrets: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }) -> bool {
    //    unsafe { TODO: call ffi:nm_vpn_service_plugin_read_vpn_details() }
    //}
}

/// Trait containing all [`struct@VpnServicePlugin`] methods.
///
/// # Implementors
///
/// [`VpnServicePlugin`][struct@crate::VpnServicePlugin]
pub trait VpnServicePluginExt: 'static {
    #[doc(alias = "nm_vpn_service_plugin_disconnect")]
    fn disconnect(&self) -> Result<(), glib::Error>;

    #[doc(alias = "nm_vpn_service_plugin_failure")]
    fn failure(&self, reason: VpnPluginFailure);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "nm_vpn_service_plugin_get_connection")]
    #[doc(alias = "get_connection")]
    fn connection(&self) -> Option<gio::DBusConnection>;

    #[doc(alias = "nm_vpn_service_plugin_set_config")]
    fn set_config(&self, config: &glib::Variant);

    #[doc(alias = "nm_vpn_service_plugin_set_ip4_config")]
    fn set_ip4_config(&self, ip4_config: &glib::Variant);

    #[doc(alias = "nm_vpn_service_plugin_set_ip6_config")]
    fn set_ip6_config(&self, ip6_config: &glib::Variant);

    #[doc(alias = "nm_vpn_service_plugin_set_login_banner")]
    fn set_login_banner(&self, banner: &str);

    /// Shutdown the `self` and disconnect from D-Bus. After this,
    /// the plugin instance is dead and should no longer be used.
    /// It ensures to get no more requests from D-Bus. In principle,
    /// you don't need to shutdown the plugin, disposing the instance
    /// has the same effect. However, this gives a way to deactivate
    /// the plugin before giving up the last reference.
    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    #[doc(alias = "nm_vpn_service_plugin_shutdown")]
    fn shutdown(&self);

    /// The D-Bus service name of this plugin.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "service-name")]
    fn service_name(&self) -> Option<glib::GString>;

    /// The state of the plugin.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn state(&self) -> VpnServiceState;

    /// The state of the plugin.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn set_state(&self, state: VpnServiceState);

    /// Whether to watch for D-Bus peer's changes.
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "watch-peer")]
    fn is_watch_peer(&self) -> bool;

    #[doc(alias = "config")]
    fn connect_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "failure")]
    fn connect_failure<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ip4-config")]
    fn connect_ip4_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "ip6-config")]
    fn connect_ip6_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "login-banner")]
    fn connect_login_banner<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "quit")]
    fn connect_quit<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //#[doc(alias = "secrets-required")]
    //fn connect_secrets_required<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "state-changed")]
    fn connect_state_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    #[doc(alias = "state")]
    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnServicePlugin>> VpnServicePluginExt for O {
    fn disconnect(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok =
                ffi::nm_vpn_service_plugin_disconnect(self.as_ref().to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn failure(&self, reason: VpnPluginFailure) {
        unsafe {
            ffi::nm_vpn_service_plugin_failure(self.as_ref().to_glib_none().0, reason.into_glib());
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn connection(&self) -> Option<gio::DBusConnection> {
        unsafe {
            from_glib_full(ffi::nm_vpn_service_plugin_get_connection(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_config(&self, config: &glib::Variant) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_config(
                self.as_ref().to_glib_none().0,
                config.to_glib_none().0,
            );
        }
    }

    fn set_ip4_config(&self, ip4_config: &glib::Variant) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_ip4_config(
                self.as_ref().to_glib_none().0,
                ip4_config.to_glib_none().0,
            );
        }
    }

    fn set_ip6_config(&self, ip6_config: &glib::Variant) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_ip6_config(
                self.as_ref().to_glib_none().0,
                ip6_config.to_glib_none().0,
            );
        }
    }

    fn set_login_banner(&self, banner: &str) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_login_banner(
                self.as_ref().to_glib_none().0,
                banner.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_12")))]
    fn shutdown(&self) {
        unsafe {
            ffi::nm_vpn_service_plugin_shutdown(self.as_ref().to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn service_name(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "service-name")
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn state(&self) -> VpnServiceState {
        glib::ObjectExt::property(self.as_ref(), "state")
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn set_state(&self, state: VpnServiceState) {
        glib::ObjectExt::set_property(self.as_ref(), "state", &state)
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn is_watch_peer(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "watch-peer")
    }

    fn connect_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn config_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P, &glib::Variant) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            object: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_failure<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn failure_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P, u32) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            object: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                object,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"failure\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    failure_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ip4_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ip4_config_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P, &glib::Variant) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            object: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ip4-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ip4_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_ip6_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn ip6_config_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P, &glib::Variant) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            object: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"ip6-config\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    ip6_config_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_login_banner<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn login_banner_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P, &str) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            object: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(object),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"login-banner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    login_banner_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_quit<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn quit_trampoline<P: IsA<VpnServicePlugin>, F: Fn(&P) + 'static>(
            this: *mut ffi::NMVpnServicePlugin,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"quit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    quit_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    //fn connect_secrets_required<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype p0: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_state_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn state_changed_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P, u32) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            object: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref(),
                object,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"state-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_2")))]
    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<
            P: IsA<VpnServicePlugin>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::NMVpnServicePlugin,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(VpnServicePlugin::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for VpnServicePlugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("VpnServicePlugin")
    }
}
