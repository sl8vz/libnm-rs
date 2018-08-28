// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Connection;
use Error;
use VpnEditor;
use VpnEditorPluginCapability;
#[cfg(any(feature = "v1_4", feature = "dox"))]
use VpnPluginInfo;

glib_wrapper! {
    pub struct VpnEditorPlugin(Object<ffi::NMVpnEditorPlugin, ffi::NMVpnEditorPluginInterface>);

    match fn {
        get_type => || ffi::nm_vpn_editor_plugin_get_type(),
    }
}

impl VpnEditorPlugin {
    #[cfg(any(feature = "v1_4", feature = "dox"))]
    pub fn load(plugin_name: &str, check_service: &str) -> Result<VpnEditorPlugin, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_vpn_editor_plugin_load(
                plugin_name.to_glib_none().0,
                check_service.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait VpnEditorPluginExt {
    fn export<P: IsA<Connection>>(&self, path: &str, connection: &P) -> Result<(), Error>;

    fn get_capabilities(&self) -> VpnEditorPluginCapability;

    fn get_editor<P: IsA<Connection>>(&self, connection: &P) -> Result<VpnEditor, Error>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_plugin_info(&self) -> Option<VpnPluginInfo>;

    fn get_suggested_filename<P: IsA<Connection>>(&self, connection: &P) -> Option<String>;

    //#[cfg(any(feature = "v1_4", feature = "dox"))]
    //fn get_vt(&self, vt: /*Ignored*/VpnEditorPluginVT, vt_size: usize) -> usize;

    fn import(&self, path: &str) -> Result<Connection, Error>;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_plugin_info<'a, P: Into<Option<&'a VpnPluginInfo>>>(&self, plugin_info: P);

    fn get_property_description(&self) -> Option<String>;

    fn get_property_name(&self) -> Option<String>;

    fn get_property_service(&self) -> Option<String>;

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnEditorPlugin> + IsA<glib::object::Object>> VpnEditorPluginExt for O {
    fn export<P: IsA<Connection>>(&self, path: &str, connection: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_vpn_editor_plugin_export(
                self.to_glib_none().0,
                path.to_glib_none().0,
                connection.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn get_capabilities(&self) -> VpnEditorPluginCapability {
        unsafe {
            from_glib(ffi::nm_vpn_editor_plugin_get_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_editor<P: IsA<Connection>>(&self, connection: &P) -> Result<VpnEditor, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_vpn_editor_plugin_get_editor(
                self.to_glib_none().0,
                connection.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_plugin_info(&self) -> Option<VpnPluginInfo> {
        unsafe {
            from_glib_none(ffi::nm_vpn_editor_plugin_get_plugin_info(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_suggested_filename<P: IsA<Connection>>(&self, connection: &P) -> Option<String> {
        unsafe {
            from_glib_full(ffi::nm_vpn_editor_plugin_get_suggested_filename(
                self.to_glib_none().0,
                connection.to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_4", feature = "dox"))]
    //fn get_vt(&self, vt: /*Ignored*/VpnEditorPluginVT, vt_size: usize) -> usize {
    //    unsafe { TODO: call ffi::nm_vpn_editor_plugin_get_vt() }
    //}

    fn import(&self, path: &str) -> Result<Connection, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_vpn_editor_plugin_import(
                self.to_glib_none().0,
                path.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_plugin_info<'a, P: Into<Option<&'a VpnPluginInfo>>>(&self, plugin_info: P) {
        let plugin_info = plugin_info.into();
        let plugin_info = plugin_info.to_glib_none();
        unsafe {
            ffi::nm_vpn_editor_plugin_set_plugin_info(self.to_glib_none().0, plugin_info.0);
        }
    }

    fn get_property_description(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "description".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn get_property_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "name".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn get_property_service(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "service".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn connect_property_description_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::description",
                transmute(notify_description_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::name",
                transmute(notify_name_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::service",
                transmute(notify_service_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn notify_description_trampoline<P>(
    this: *mut ffi::NMVpnEditorPlugin,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnEditorPlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnEditorPlugin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_name_trampoline<P>(
    this: *mut ffi::NMVpnEditorPlugin,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnEditorPlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnEditorPlugin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_service_trampoline<P>(
    this: *mut ffi::NMVpnEditorPlugin,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnEditorPlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnEditorPlugin::from_glib_borrow(this).downcast_unchecked())
}
