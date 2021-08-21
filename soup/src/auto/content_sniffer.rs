// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::SessionFeature;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "SoupContentSniffer")]
    pub struct ContentSniffer(Object<ffi::SoupContentSniffer, ffi::SoupContentSnifferClass>) @implements SessionFeature;

    match fn {
        type_ => || ffi::soup_content_sniffer_get_type(),
    }
}

impl ContentSniffer {
    #[doc(alias = "soup_content_sniffer_new")]
    pub fn new() -> ContentSniffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::soup_content_sniffer_new())
        }
    }
}

#[cfg(any(feature = "v2_28", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_28")))]
impl Default for ContentSniffer {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CONTENT_SNIFFER: Option<&ContentSniffer> = None;

pub trait ContentSnifferExt: 'static {
    #[doc(alias = "soup_content_sniffer_get_buffer_size")]
    #[doc(alias = "get_buffer_size")]
    fn buffer_size(&self) -> usize;

    //#[doc(alias = "soup_content_sniffer_sniff")]
    //fn sniff<P: IsA<Message>>(&self, msg: &P, buffer: &mut Buffer, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString>;
}

impl<O: IsA<ContentSniffer>> ContentSnifferExt for O {
    fn buffer_size(&self) -> usize {
        unsafe {
            ffi::soup_content_sniffer_get_buffer_size(self.as_ref().to_glib_none().0)
        }
    }

    //fn sniff<P: IsA<Message>>(&self, msg: &P, buffer: &mut Buffer, params: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) -> Option<glib::GString> {
    //    unsafe { TODO: call ffi:soup_content_sniffer_sniff() }
    //}
}

impl fmt::Display for ContentSniffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ContentSniffer")
    }
}
