// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use Error;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord, Hash)]
    pub struct IPAddress(Shared<ffi::NMIPAddress>);

    match fn {
        ref => |ptr| ffi::nm_ip_address_ref(ptr),
        unref => |ptr| ffi::nm_ip_address_unref(ptr),
        get_type => || ffi::nm_ip_address_get_type(),
    }
}

impl IPAddress {
    pub fn new(family: i32, addr: &str, prefix: u32) -> Result<IPAddress, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::nm_ip_address_new(family, addr.to_glib_none().0, prefix, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //pub fn new_binary<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(family: i32, addr: P, prefix: u32) -> Result<IPAddress, Error> {
    //    unsafe { TODO: call ffi::nm_ip_address_new_binary() }
    //}

    pub fn dup(&self) -> Option<IPAddress> {
        unsafe { from_glib_full(ffi::nm_ip_address_dup(self.to_glib_none().0)) }
    }

    fn equal(&self, other: &IPAddress) -> bool {
        unsafe {
            from_glib(ffi::nm_ip_address_equal(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    pub fn get_address(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_ip_address_get_address(self.to_glib_none().0)) }
    }

    //pub fn get_address_binary<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, addr: P) {
    //    unsafe { TODO: call ffi::nm_ip_address_get_address_binary() }
    //}

    pub fn get_attribute(&self, name: &str) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::nm_ip_address_get_attribute(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    pub fn get_attribute_names(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::nm_ip_address_get_attribute_names(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_family(&self) -> i32 {
        unsafe { ffi::nm_ip_address_get_family(self.to_glib_none().0) }
    }

    pub fn get_prefix(&self) -> u32 {
        unsafe { ffi::nm_ip_address_get_prefix(self.to_glib_none().0) }
    }

    pub fn set_address(&self, addr: &str) {
        unsafe {
            ffi::nm_ip_address_set_address(self.to_glib_none().0, addr.to_glib_none().0);
        }
    }

    //pub fn set_address_binary<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, addr: P) {
    //    unsafe { TODO: call ffi::nm_ip_address_set_address_binary() }
    //}

    pub fn set_attribute<'a, P: Into<Option<&'a glib::Variant>>>(&self, name: &str, value: P) {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            ffi::nm_ip_address_set_attribute(self.to_glib_none().0, name.to_glib_none().0, value.0);
        }
    }

    pub fn set_prefix(&self, prefix: u32) {
        unsafe {
            ffi::nm_ip_address_set_prefix(self.to_glib_none().0, prefix);
        }
    }
}

impl PartialEq for IPAddress {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

impl Eq for IPAddress {}