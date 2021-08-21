// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct URI(Boxed<ffi::SoupURI>);

    match fn {
        copy => |ptr| ffi::soup_uri_copy(mut_override(ptr)),
        free => |ptr| ffi::soup_uri_free(ptr),
        type_ => || ffi::soup_uri_get_type(),
    }
}

impl URI {
    #[doc(alias = "soup_uri_new")]
    pub fn new(uri_string: Option<&str>) -> Option<URI> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_uri_new(uri_string.to_glib_none().0))
        }
    }

    #[doc(alias = "soup_uri_new_with_base")]
    #[doc(alias = "new_with_base")]
    pub fn with_base(base: &mut URI, uri_string: &str) -> URI {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_uri_new_with_base(base.to_glib_none_mut().0, uri_string.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_28", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
    #[doc(alias = "soup_uri_copy_host")]
    pub fn copy_host(&mut self) -> Option<URI> {
        unsafe {
            from_glib_full(ffi::soup_uri_copy_host(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_fragment")]
    #[doc(alias = "get_fragment")]
    pub fn fragment(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_fragment(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_host")]
    #[doc(alias = "get_host")]
    pub fn host(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_host(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_password")]
    #[doc(alias = "get_password")]
    pub fn password(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_password(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_path(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_port")]
    #[doc(alias = "get_port")]
    pub fn port(&mut self) -> u32 {
        unsafe {
            ffi::soup_uri_get_port(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_query")]
    #[doc(alias = "get_query")]
    pub fn query(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_query(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_scheme")]
    #[doc(alias = "get_scheme")]
    pub fn scheme(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_scheme(self.to_glib_none_mut().0))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[doc(alias = "soup_uri_get_user")]
    #[doc(alias = "get_user")]
    pub fn user(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::soup_uri_get_user(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "soup_uri_set_fragment")]
    pub fn set_fragment(&mut self, fragment: Option<&str>) {
        unsafe {
            ffi::soup_uri_set_fragment(self.to_glib_none_mut().0, fragment.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_uri_set_host")]
    pub fn set_host(&mut self, host: Option<&str>) {
        unsafe {
            ffi::soup_uri_set_host(self.to_glib_none_mut().0, host.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_uri_set_password")]
    pub fn set_password(&mut self, password: Option<&str>) {
        unsafe {
            ffi::soup_uri_set_password(self.to_glib_none_mut().0, password.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_uri_set_path")]
    pub fn set_path(&mut self, path: &str) {
        unsafe {
            ffi::soup_uri_set_path(self.to_glib_none_mut().0, path.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_uri_set_port")]
    pub fn set_port(&mut self, port: u32) {
        unsafe {
            ffi::soup_uri_set_port(self.to_glib_none_mut().0, port);
        }
    }

    #[doc(alias = "soup_uri_set_query")]
    pub fn set_query(&mut self, query: Option<&str>) {
        unsafe {
            ffi::soup_uri_set_query(self.to_glib_none_mut().0, query.to_glib_none().0);
        }
    }

    //#[doc(alias = "soup_uri_set_query_from_fields")]
    //pub fn set_query_from_fields(&mut self, first_field: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:soup_uri_set_query_from_fields() }
    //}

    //#[doc(alias = "soup_uri_set_query_from_form")]
    //pub fn set_query_from_form(&mut self, form: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
    //    unsafe { TODO: call ffi:soup_uri_set_query_from_form() }
    //}

    #[doc(alias = "soup_uri_set_scheme")]
    pub fn set_scheme(&mut self, scheme: &str) {
        unsafe {
            ffi::soup_uri_set_scheme(self.to_glib_none_mut().0, scheme.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_uri_set_user")]
    pub fn set_user(&mut self, user: Option<&str>) {
        unsafe {
            ffi::soup_uri_set_user(self.to_glib_none_mut().0, user.to_glib_none().0);
        }
    }

    #[doc(alias = "soup_uri_to_string")]
    pub fn to_string(&mut self, just_path_and_query: bool) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::soup_uri_to_string(self.to_glib_none_mut().0, just_path_and_query.into_glib()))
        }
    }

    #[doc(alias = "soup_uri_uses_default_port")]
    pub fn uses_default_port(&mut self) -> bool {
        unsafe {
            from_glib(ffi::soup_uri_uses_default_port(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "soup_uri_decode")]
    pub fn decode(part: &str) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_uri_decode(part.to_glib_none().0))
        }
    }

    #[doc(alias = "soup_uri_encode")]
    pub fn encode(part: &str, escape_extra: Option<&str>) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_uri_encode(part.to_glib_none().0, escape_extra.to_glib_none().0))
        }
    }

    #[doc(alias = "soup_uri_normalize")]
    pub fn normalize(part: &str, unescape_extra: Option<&str>) -> Option<glib::GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_uri_normalize(part.to_glib_none().0, unescape_extra.to_glib_none().0))
        }
    }
}
