// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use glib::GString;
use libc;
use soup_sys;
use DateFormat;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Date(Boxed<soup_sys::SoupDate>);

    match fn {
        copy => |ptr| soup_sys::soup_date_copy(mut_override(ptr)),
        free => |ptr| soup_sys::soup_date_free(ptr),
        get_type => || soup_sys::soup_date_get_type(),
    }
}

impl Date {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Date {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_date_new(year, month, day, hour, minute, second))
        }
    }

    pub fn new_from_now(offset_seconds: i32) -> Date {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_date_new_from_now(offset_seconds))
        }
    }

    pub fn new_from_string(date_string: &str) -> Option<Date> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_date_new_from_string(date_string.to_glib_none().0))
        }
    }

    pub fn new_from_time_t(when: libc::c_long) -> Date {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(soup_sys::soup_date_new_from_time_t(when))
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_day(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_day(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_hour(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_hour(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_minute(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_minute(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_month(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_month(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_offset(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_offset(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_second(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_second(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_utc(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_utc(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    pub fn get_year(&mut self) -> i32 {
        unsafe {
            soup_sys::soup_date_get_year(self.to_glib_none_mut().0)
        }
    }

    #[cfg(any(feature = "v2_24", feature = "dox"))]
    pub fn is_past(&mut self) -> bool {
        unsafe {
            from_glib(soup_sys::soup_date_is_past(self.to_glib_none_mut().0))
        }
    }

    pub fn to_string(&mut self, format: DateFormat) -> GString {
        unsafe {
            from_glib_full(soup_sys::soup_date_to_string(self.to_glib_none_mut().0, format.to_glib()))
        }
    }

    pub fn to_time_t(&mut self) -> libc::c_long {
        unsafe {
            soup_sys::soup_date_to_time_t(self.to_glib_none_mut().0)
        }
    }

    //#[cfg(any(feature = "v2_24", feature = "dox"))]
    //pub fn to_timeval(&mut self, time: /*Ignored*/glib::TimeVal) {
    //    unsafe { TODO: call soup_sys:soup_date_to_timeval() }
    //}
}
