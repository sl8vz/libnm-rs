// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files
// DO NOT EDIT

#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use glib::translate::*;
#[cfg(any(feature = "v1_32", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_32")))]
use std::mem;
#[cfg(any(feature = "v1_18", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
use std::ptr;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct IPRoutingRule(Shared<ffi::NMIPRoutingRule>);

    match fn {
        ref => |ptr| ffi::nm_ip_routing_rule_ref(ptr),
        unref => |ptr| ffi::nm_ip_routing_rule_unref(ptr),
        type_ => || ffi::nm_ip_routing_rule_get_type(),
    }
}

impl IPRoutingRule {
    /// ## `addr_family`
    /// the address family of the routing rule. Must be either
    ///  `AF_INET` (2) or `AF_INET6` (10).
    ///
    /// # Returns
    ///
    /// a newly created rule instance with the
    ///  provided address family. The instance is unsealed.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_new")]
    pub fn new(addr_family: i32) -> IPRoutingRule {
        unsafe { from_glib_full(ffi::nm_ip_routing_rule_new(addr_family)) }
    }

    /// ## `other`
    /// the other [`IPRoutingRule`][crate::IPRoutingRule] instance to compare
    ///
    /// # Returns
    ///
    /// zero, a positive, or a negative integer to indicate
    ///  equality or how the arguments compare.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_cmp")]
    pub fn cmp(&self, other: Option<&IPRoutingRule>) -> i32 {
        unsafe { ffi::nm_ip_routing_rule_cmp(self.to_glib_none().0, other.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set action.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_action")]
    #[doc(alias = "get_action")]
    pub fn action(&self) -> u8 {
        unsafe { ffi::nm_ip_routing_rule_get_action(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the address family of the rule. Either `AF_INET` or `AF_INET6`.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_addr_family")]
    #[doc(alias = "get_addr_family")]
    pub fn addr_family(&self) -> i32 {
        unsafe { ffi::nm_ip_routing_rule_get_addr_family(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the destination port end setting.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_destination_port_end")]
    #[doc(alias = "get_destination_port_end")]
    pub fn destination_port_end(&self) -> u16 {
        unsafe { ffi::nm_ip_routing_rule_get_destination_port_end(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the destination port start setting.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_destination_port_start")]
    #[doc(alias = "get_destination_port_start")]
    pub fn destination_port_start(&self) -> u16 {
        unsafe { ffi::nm_ip_routing_rule_get_destination_port_start(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set from/src parameter or
    ///  [`None`], if no value is set.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_from")]
    #[doc(alias = "get_from")]
    pub fn from(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_ip_routing_rule_get_from(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the set prefix length for the from/src parameter.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_from_len")]
    #[doc(alias = "get_from_len")]
    pub fn from_len(&self) -> u8 {
        unsafe { ffi::nm_ip_routing_rule_get_from_len(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the fwmark setting.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_fwmark")]
    #[doc(alias = "get_fwmark")]
    pub fn fwmark(&self) -> u32 {
        unsafe { ffi::nm_ip_routing_rule_get_fwmark(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the fwmask setting.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_fwmask")]
    #[doc(alias = "get_fwmask")]
    pub fn fwmask(&self) -> u32 {
        unsafe { ffi::nm_ip_routing_rule_get_fwmask(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set iifname or [`None`] if unset.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_iifname")]
    #[doc(alias = "get_iifname")]
    pub fn iifname(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_ip_routing_rule_get_iifname(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the "invert" setting of the rule.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_invert")]
    #[doc(alias = "get_invert")]
    pub fn inverts(&self) -> bool {
        unsafe { from_glib(ffi::nm_ip_routing_rule_get_invert(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the ipproto of the rule.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_ipproto")]
    #[doc(alias = "get_ipproto")]
    pub fn ipproto(&self) -> u8 {
        unsafe { ffi::nm_ip_routing_rule_get_ipproto(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set oifname or [`None`] if unset.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_oifname")]
    #[doc(alias = "get_oifname")]
    pub fn oifname(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_ip_routing_rule_get_oifname(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the priority. A valid priority is in the range from
    ///  0 to `G_MAXUINT32`. If unset, -1 is returned.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_priority")]
    #[doc(alias = "get_priority")]
    pub fn priority(&self) -> i64 {
        unsafe { ffi::nm_ip_routing_rule_get_priority(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the source port end setting.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_source_port_end")]
    #[doc(alias = "get_source_port_end")]
    pub fn source_port_end(&self) -> u16 {
        unsafe { ffi::nm_ip_routing_rule_get_source_port_end(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the source port start setting.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_source_port_start")]
    #[doc(alias = "get_source_port_start")]
    pub fn source_port_start(&self) -> u16 {
        unsafe { ffi::nm_ip_routing_rule_get_source_port_start(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the suppress_prefixlength of the rule. -1 means that the value is unset.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "nm_ip_routing_rule_get_suppress_prefixlength")]
    #[doc(alias = "get_suppress_prefixlength")]
    pub fn suppress_prefixlength(&self) -> i32 {
        unsafe { ffi::nm_ip_routing_rule_get_suppress_prefixlength(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set table.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_table")]
    #[doc(alias = "get_table")]
    pub fn table(&self) -> u32 {
        unsafe { ffi::nm_ip_routing_rule_get_table(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the set to/dst parameter or
    ///  [`None`], if no value is set.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_to")]
    #[doc(alias = "get_to")]
    pub fn to(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::nm_ip_routing_rule_get_to(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// the set prefix length for the to/dst parameter.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_to_len")]
    #[doc(alias = "get_to_len")]
    pub fn to_len(&self) -> u8 {
        unsafe { ffi::nm_ip_routing_rule_get_to_len(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// the tos of the rule.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_get_tos")]
    #[doc(alias = "get_tos")]
    pub fn tos(&self) -> u8 {
        unsafe { ffi::nm_ip_routing_rule_get_tos(self.to_glib_none().0) }
    }

    ///
    /// # Returns
    ///
    /// [`true`] if a uid range is set.
    ///
    /// ## `out_range_start`
    /// returns the start of the range
    ///  or 0 if the range is not set.
    ///
    /// ## `out_range_end`
    /// returns the end of the range
    ///  or 0 if the range is not set.
    #[cfg(any(feature = "v1_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_32")))]
    #[doc(alias = "nm_ip_routing_rule_get_uid_range")]
    #[doc(alias = "get_uid_range")]
    pub fn uid_range(&self) -> Option<(u32, u32)> {
        unsafe {
            let mut out_range_start = mem::MaybeUninit::uninit();
            let mut out_range_end = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::nm_ip_routing_rule_get_uid_range(
                self.to_glib_none().0,
                out_range_start.as_mut_ptr(),
                out_range_end.as_mut_ptr(),
            ));
            let out_range_start = out_range_start.assume_init();
            let out_range_end = out_range_end.assume_init();
            if ret {
                Some((out_range_start, out_range_end))
            } else {
                None
            }
        }
    }

    ///
    /// # Returns
    ///
    /// whether `self` is sealed. Once sealed, an instance
    ///  cannot be modified nor unsealed.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_is_sealed")]
    pub fn is_sealed(&self) -> bool {
        unsafe { from_glib(ffi::nm_ip_routing_rule_is_sealed(self.to_glib_none().0)) }
    }

    ///
    /// # Returns
    ///
    /// a newly created rule instance with
    ///  the same settings as `self`. Note that the instance will
    ///  always be unsealred.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_new_clone")]
    pub fn new_clone(&self) -> Option<IPRoutingRule> {
        unsafe { from_glib_full(ffi::nm_ip_routing_rule_new_clone(self.to_glib_none().0)) }
    }

    /// Seals the routing rule. Afterwards, the instance can no longer be
    /// modified, and it is a bug to call any of the accessors that would
    /// modify the rule. If `self` was already sealed, this has no effect.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_seal")]
    pub fn seal(&self) {
        unsafe {
            ffi::nm_ip_routing_rule_seal(self.to_glib_none().0);
        }
    }

    /// Note that currently only certain actions are allowed. [`validate()`][Self::validate()]
    /// will reject unsupported actions as invalid.
    /// ## `action`
    /// the action to set
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_action")]
    pub fn set_action(&self, action: u8) {
        unsafe {
            ffi::nm_ip_routing_rule_set_action(self.to_glib_none().0, action);
        }
    }

    /// ## `start`
    /// the start port to set.
    /// ## `end`
    /// the end port to set.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_destination_port")]
    pub fn set_destination_port(&self, start: u16, end: u16) {
        unsafe {
            ffi::nm_ip_routing_rule_set_destination_port(self.to_glib_none().0, start, end);
        }
    }

    /// ## `fwmark`
    /// the fwmark
    /// ## `fwmask`
    /// the fwmask
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_fwmark")]
    pub fn set_fwmark(&self, fwmark: u32, fwmask: u32) {
        unsafe {
            ffi::nm_ip_routing_rule_set_fwmark(self.to_glib_none().0, fwmark, fwmask);
        }
    }

    /// The name supports C backslash escaping for non-UTF-8 characters.
    /// Note that `nm_ip_routing_rule_from_string()` too uses backslash
    /// escaping when tokenizing the words by whitespace. So, in string
    /// representation you'd get double backslashes.
    /// ## `iifname`
    /// the iifname to set or [`None`] to unset.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_iifname")]
    pub fn set_iifname(&self, iifname: Option<&str>) {
        unsafe {
            ffi::nm_ip_routing_rule_set_iifname(self.to_glib_none().0, iifname.to_glib_none().0);
        }
    }

    /// ## `invert`
    /// the new value to set
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_invert")]
    pub fn set_invert(&self, invert: bool) {
        unsafe {
            ffi::nm_ip_routing_rule_set_invert(self.to_glib_none().0, invert.into_glib());
        }
    }

    /// ## `ipproto`
    /// the ipproto to set
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_ipproto")]
    pub fn set_ipproto(&self, ipproto: u8) {
        unsafe {
            ffi::nm_ip_routing_rule_set_ipproto(self.to_glib_none().0, ipproto);
        }
    }

    /// The name supports C backslash escaping for non-UTF-8 characters.
    /// Note that `nm_ip_routing_rule_from_string()` too uses backslash
    /// escaping when tokenizing the words by whitespace. So, in string
    /// representation you'd get double backslashes.
    /// ## `oifname`
    /// the oifname to set or [`None`] to unset.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_oifname")]
    pub fn set_oifname(&self, oifname: Option<&str>) {
        unsafe {
            ffi::nm_ip_routing_rule_set_oifname(self.to_glib_none().0, oifname.to_glib_none().0);
        }
    }

    /// A valid priority ranges from 0 to `G_MAXUINT32`. "-1" is also allowed
    /// to reset the priority. It is a bug calling this function with any
    /// other value.
    /// ## `priority`
    /// the priority to set
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_priority")]
    pub fn set_priority(&self, priority: i64) {
        unsafe {
            ffi::nm_ip_routing_rule_set_priority(self.to_glib_none().0, priority);
        }
    }

    /// ## `start`
    /// the start port to set.
    /// ## `end`
    /// the end port to set.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_source_port")]
    pub fn set_source_port(&self, start: u16, end: u16) {
        unsafe {
            ffi::nm_ip_routing_rule_set_source_port(self.to_glib_none().0, start, end);
        }
    }

    /// ## `suppress_prefixlength`
    /// the suppress_prefixlength to set. The value -1 means
    ///  unset.
    #[cfg(any(feature = "v1_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_20")))]
    #[doc(alias = "nm_ip_routing_rule_set_suppress_prefixlength")]
    pub fn set_suppress_prefixlength(&self, suppress_prefixlength: i32) {
        unsafe {
            ffi::nm_ip_routing_rule_set_suppress_prefixlength(
                self.to_glib_none().0,
                suppress_prefixlength,
            );
        }
    }

    /// ## `table`
    /// the table to set
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_table")]
    pub fn set_table(&self, table: u32) {
        unsafe {
            ffi::nm_ip_routing_rule_set_table(self.to_glib_none().0, table);
        }
    }

    /// ## `tos`
    /// the tos to set
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_set_tos")]
    pub fn set_tos(&self, tos: u8) {
        unsafe {
            ffi::nm_ip_routing_rule_set_tos(self.to_glib_none().0, tos);
        }
    }

    /// For a valid range, start must be less or equal to end.
    /// If set to an invalid range, the range gets unset.
    /// ## `uid_range_start`
    /// the uid_range start to set.
    /// ## `uid_range_end`
    /// the uid_range start to set.
    #[cfg(any(feature = "v1_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_32")))]
    #[doc(alias = "nm_ip_routing_rule_set_uid_range")]
    pub fn set_uid_range(&self, uid_range_start: u32, uid_range_end: u32) {
        unsafe {
            ffi::nm_ip_routing_rule_set_uid_range(
                self.to_glib_none().0,
                uid_range_start,
                uid_range_end,
            );
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "nm_ip_routing_rule_to_string")]
    //pub fn to_string(&self, to_string_flags: IPRoutingRuleAsStringFlags, extra_args: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }) -> Result<glib::GString, glib::Error> {
    //    unsafe { TODO: call ffi:nm_ip_routing_rule_to_string() }
    //}

    ///
    /// # Returns
    ///
    /// [`true`] if the rule validates.
    #[cfg(any(feature = "v1_18", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    #[doc(alias = "nm_ip_routing_rule_validate")]
    pub fn validate(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let is_ok = ffi::nm_ip_routing_rule_validate(self.to_glib_none().0, &mut error);
            assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    //#[cfg(any(feature = "v1_18", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_18")))]
    //#[doc(alias = "nm_ip_routing_rule_from_string")]
    //pub fn from_string(str: &str, to_string_flags: IPRoutingRuleAsStringFlags, extra_args: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }) -> Result<IPRoutingRule, glib::Error> {
    //    unsafe { TODO: call ffi:nm_ip_routing_rule_from_string() }
    //}
}
