use crate::FeatureReader;

use libc::{c_char, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ushort};

use std::ffi::CStr;
use std::str::Utf8Error;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attribute(pub(crate) c_ushort);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeStatus {
    Normal,
    Unknown,
    Bos,
    Eos,
    EoNbest,
}

impl NodeStatus {
    #[inline]
    pub fn is_normal(self) -> bool {
        self == Self::Normal
    }

    #[inline]
    pub fn is_unknown(self) -> bool {
        self == Self::Unknown
    }

    #[inline]
    pub fn is_bos(self) -> bool {
        self == Self::Bos
    }

    #[inline]
    pub fn is_eos(self) -> bool {
        self == Self::Eos
    }

    #[inline]
    pub fn is_eon(self) -> bool {
        self == Self::EoNbest
    }
}

#[repr(C)]
pub struct Node {
    prev: *mut Node,
    next: *mut Node,
    enext: *mut Node,
    bnext: *mut Node,

    rpath: *mut Path,
    lpath: *mut Path,

    surface: *const c_char,
    feature: *const c_char,
    pub id: c_uint,
    pub length: c_ushort,
    pub rlength: c_ushort,

    pub rattr: Attribute,
    pub lattr: Attribute,

    pub posid: c_ushort,

    pub char_type: c_uchar,
    stat: c_uchar,
    isbest: c_uchar,

    pub alpha: c_float,
    pub beta: c_float,
    pub prob: c_float,
    pub wcost: c_short,
    pub cost: c_long,
}

#[repr(C)]
pub struct Path {
    rnode: *mut Node,
    rnext: *mut Path,
    lnode: *mut Node,
    lnext: *mut Path,

    pub cost: c_int,
    pub prob: c_float,
}

impl Node {
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
    }
    pub fn prev(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref() }
    }

    pub fn enext(&self) -> Option<&Self> {
        unsafe { self.enext.as_ref() }
    }
    pub fn bnext(&self) -> Option<&Self> {
        unsafe { self.bnext.as_ref() }
    }

    pub fn rpath(&self) -> Option<&Path> {
        unsafe { self.rpath.as_ref() }
    }
    pub fn lpath(&self) -> Option<&Path> {
        unsafe { self.lpath.as_ref() }
    }

    pub fn status(&self) -> NodeStatus {
        match self.stat {
            0 => NodeStatus::Normal,
            1 => NodeStatus::Unknown,
            2 => NodeStatus::Bos,
            3 => NodeStatus::Eos,
            _ => NodeStatus::EoNbest,
        }
    }

    pub fn is_best(&self) -> bool {
        self.isbest == 1
    }

    pub fn surface_len(&self) -> usize {
        self.length as _
    }

    pub fn surface(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.surface as *const u8, self.length as usize) }
    }
    pub fn surface_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.surface())
    }

    pub fn features(&self) -> &[u8] {
        unsafe {
            let s = CStr::from_ptr(self.feature);
            s.to_bytes()
        }
    }
    pub fn features_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.features())
    }
    pub fn feature_reader(&self) -> FeatureReader<'_> {
        FeatureReader::from_node(self)
    }
}

impl Path {
    pub fn rnext(&self) -> Option<&Self> {
        unsafe { self.rnext.as_ref() }
    }
    pub fn lnext(&self) -> Option<&Self> {
        unsafe { self.lnext.as_ref() }
    }

    pub fn rnode(&self) -> Option<&Node> {
        unsafe { self.rnode.as_ref() }
    }
    pub fn lnode(&self) -> Option<&Node> {
        unsafe { self.lnode.as_ref() }
    }
}
