// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_42", feature = "dox"))]
use Error;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use Request;
use SessionFeature;
use glib::object::IsA;
use glib::translate::*;
use soup_sys;
use std::fmt;
#[cfg(any(feature = "v2_42", feature = "dox"))]
use std::ptr;

glib_wrapper! {
    pub struct Requester(Object<soup_sys::SoupRequester, soup_sys::SoupRequesterClass, RequesterClass>) @implements SessionFeature;

    match fn {
        get_type => || soup_sys::soup_requester_get_type(),
    }
}

impl Requester {
    pub fn new() -> Requester {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_requester_new())
        }
    }
}

impl Default for Requester {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_REQUESTER: Option<&Requester> = None;

pub trait RequesterExt: 'static {
    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn request(&self, uri_string: &str) -> Result<Request, Error>;

    //#[cfg(any(feature = "v2_42", feature = "dox"))]
    //fn request_uri(&self, uri: /*Ignored*/&mut URI) -> Result<Request, Error>;
}

impl<O: IsA<Requester>> RequesterExt for O {
    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn request(&self, uri_string: &str) -> Result<Request, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = soup_sys::soup_requester_request(self.as_ref().to_glib_none().0, uri_string.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //#[cfg(any(feature = "v2_42", feature = "dox"))]
    //fn request_uri(&self, uri: /*Ignored*/&mut URI) -> Result<Request, Error> {
    //    unsafe { TODO: call soup_sys:soup_requester_request_uri() }
    //}
}

impl fmt::Display for Requester {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Requester")
    }
}