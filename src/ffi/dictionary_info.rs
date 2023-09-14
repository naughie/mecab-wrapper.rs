use libc::{c_char, c_int, c_uint, c_ushort};

use std::ffi::CStr;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DictionaryType {
    /// System dictionary
    System,
    /// User dictionary
    User,
    /// Unknown word dictionary
    UnknownWord,
}

#[repr(C)]
pub struct DictionaryInfo {
    filename: *const c_char,
    charset: *const c_char,
    pub size: c_uint,
    r#type: c_int,
    pub lsize: c_uint,
    pub rsize: c_uint,
    pub version: c_ushort,
}

unsafe impl Send for DictionaryInfo {}
unsafe impl Sync for DictionaryInfo {}

impl DictionaryInfo {
    pub fn filename(&self) -> &[u8] {
        unsafe {
            let s = CStr::from_ptr(self.filename);
            s.to_bytes()
        }
    }
    pub fn charset(&self) -> &[u8] {
        unsafe {
            let s = CStr::from_ptr(self.charset);
            s.to_bytes()
        }
    }

    pub fn dictionary_type(&self) -> DictionaryType {
        match self.r#type {
            0 => DictionaryType::System,
            1 => DictionaryType::User,
            _ => DictionaryType::UnknownWord,
        }
    }
}

impl fmt::Debug for DictionaryInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DictionaryInfo")
            .field("filename", &self.filename())
            .field("charset", &self.charset())
            .field("size", &self.size)
            .field("lsize", &self.lsize)
            .field("rsize", &self.rsize)
            .field("type", &self.dictionary_type())
            .field("version", &self.version)
            .finish()
    }
}
