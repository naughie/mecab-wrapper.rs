use libc::{c_char, c_int, c_uint, c_ushort};

use std::ffi::CStr;
use std::fmt;

/// Dictionary type. This is a return value of [`DictionaryInfo::dictionary_type()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DictionaryType {
    /// System dictionary
    System,
    /// User dictionary
    User,
    /// Unknown word dictionary
    UnknownWord,
}

/// Dictionary structure information.
///
/// It has the same layout as
/// [`MeCab::DictionaryInfo`](https://taku910.github.io/mecab/doxygen/structmecab__dictionary__info__t.html).
#[repr(C)]
pub struct DictionaryInfo {
    filename: *const c_char,
    charset: *const c_char,
    /// How many words are registered in this dictionary.
    pub size: c_uint,
    r#type: c_int,
    /// Size of left attributes.
    pub lsize: c_uint,
    /// Size of right attributes.
    pub rsize: c_uint,
    /// Version of this dictionary.
    pub version: c_ushort,
    next: *mut DictionaryInfo,
}

unsafe impl Send for DictionaryInfo {}
unsafe impl Sync for DictionaryInfo {}

impl DictionaryInfo {
    /// Filename of dictionary. On Windows, filename is stored in UTF-8 encoding.
    pub fn filename(&self) -> &[u8] {
        unsafe {
            let s = CStr::from_ptr(self.filename);
            s.to_bytes()
        }
    }

    /// Character set of the dictionary. E.g., `"SHIFT-JIS"`, `"UTF-8"`.
    pub fn charset(&self) -> &[u8] {
        unsafe {
            let s = CStr::from_ptr(self.charset);
            s.to_bytes()
        }
    }

    /// Dictionary type.
    pub fn dictionary_type(&self) -> DictionaryType {
        match self.r#type {
            0 => DictionaryType::System,
            1 => DictionaryType::User,
            _ => DictionaryType::UnknownWord,
        }
    }

    /// Pointer to the next dictionary info.
    ///
    /// It returns `None` if the pointer is null.
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
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
