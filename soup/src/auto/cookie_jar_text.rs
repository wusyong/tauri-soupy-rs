// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_26", feature = "dox"))]
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::Value;
use gobject_sys;
use soup_sys;
use std::fmt;
use CookieJar;
use SessionFeature;

glib_wrapper! {
    pub struct CookieJarText(Object<soup_sys::SoupCookieJarText, soup_sys::SoupCookieJarTextClass, CookieJarTextClass>) @extends CookieJar, @implements SessionFeature;

    match fn {
        get_type => || soup_sys::soup_cookie_jar_text_get_type(),
    }
}

impl CookieJarText {
    #[cfg(any(feature = "v2_26", feature = "dox"))]
    pub fn new(filename: &str, read_only: bool) -> CookieJarText {
        assert_initialized_main_thread!();
        unsafe {
            CookieJar::from_glib_full(soup_sys::soup_cookie_jar_text_new(filename.to_glib_none().0, read_only.to_glib())).unsafe_cast()
        }
    }
}

pub const NONE_COOKIE_JAR_TEXT: Option<&CookieJarText> = None;

pub trait CookieJarTextExt: 'static {
    fn get_property_filename(&self) -> Option<GString>;
}

impl<O: IsA<CookieJarText>> CookieJarTextExt for O {
    fn get_property_filename(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"filename\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `filename` getter")
        }
    }
}

impl fmt::Display for CookieJarText {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CookieJarText")
    }
}
