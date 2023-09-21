use crate::FeatureReader;

use libc::{c_char, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ushort};

use std::ffi::CStr;
use std::fmt;
use std::str::Utf8Error;

/// Attribute ID.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attribute(pub(crate) c_ushort);

/// Status of a node. This is a return value of [`Node::status()`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NodeStatus {
    /// Normal node defined in the dictionary.
    Normal,
    /// Unknown node not defined in the dictionary.
    Unknown,
    /// Virtual node representing a beginning of the sentence.
    Bos,
    /// Virtual node representing an end of the sentence.
    Eos,
    /// Virtual node representing an end of the N-best enumeration.
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

/// Node structure. It has the same layout as
/// [`MeCab::Node`](https://taku910.github.io/mecab/doxygen/structmecab__node__t.html).
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
    /// Unique node ID.
    pub id: c_uint,
    /// Length of the surface form.
    pub length: c_ushort,
    /// Length of the surface form including white space before the morph.
    pub rlength: c_ushort,

    /// Right attribute ID.
    pub rattr: Attribute,
    /// Left attribute ID.
    pub lattr: Attribute,

    /// Unique part of speech ID. This value is defined in "pos.def" file.
    pub posid: c_ushort,

    /// Character type.
    pub char_type: c_uchar,
    stat: c_uchar,
    isbest: c_uchar,

    /// Forward accumulative log summation. Available only when
    /// [`MARGINAL_PROB`](crate::RequestType::MARGINAL_PROB) is passed.
    pub alpha: c_float,
    /// Backward accumulative log summation. Available only when
    /// [`MARGINAL_PROB`](crate::RequestType::MARGINAL_PROB) is passed.
    pub beta: c_float,
    /// Marginal probability. Available only when
    /// [`MARGINAL_PROB`](crate::RequestType::MARGINAL_PROB) is passed.
    pub prob: c_float,
    /// Word cost.
    pub wcost: c_short,
    /// Best accumulative cost from the BOS node to this node.
    pub cost: c_long,
}

impl PartialEq for &Node {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(*self, *other)
    }
}
impl Eq for &Node {}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("Node");

        let f = f.field("id", &self.id).field("status", &self.status());

        let surf = self.surface();
        let surf = String::from_utf8_lossy(surf);

        let f = f.field("surface", &surf);

        f.finish()
    }
}

/// Path structure. It has the same layout as
/// [`MeCab::Path`](https://taku910.github.io/mecab/doxygen/structmecab__path__t.html).
#[repr(C)]
pub struct Path {
    rnode: *mut Node,
    rnext: *mut Path,
    lnode: *mut Node,
    lnext: *mut Path,

    /// Local cost.
    pub cost: c_int,
    /// Marginal probability.
    pub prob: c_float,
}

impl Node {
    /// Pointer to the next node.
    ///
    /// It returns `None` if the pointer is null.
    pub fn next(&self) -> Option<&Self> {
        unsafe { self.next.as_ref() }
    }
    /// Pointer to the previous node.
    ///
    /// It returns `None` if the pointer is null.
    pub fn prev(&self) -> Option<&Self> {
        unsafe { self.prev.as_ref() }
    }

    /// Pointer to the node which ends at the same position.
    ///
    /// It returns `None` if the pointer is null.
    pub fn enext(&self) -> Option<&Self> {
        unsafe { self.enext.as_ref() }
    }
    /// Pointer to the node which starts at the same position.
    ///
    /// It returns `None` if the pointer is null.
    pub fn bnext(&self) -> Option<&Self> {
        unsafe { self.bnext.as_ref() }
    }

    /// Pointer to the right path.
    ///
    /// Returns `None` if the pointer is null, and the pointer is null if
    /// [`ONE_BEST`](crate::RequestType::ONE_BEST) mode.
    pub fn rpath(&self) -> Option<&Path> {
        unsafe { self.rpath.as_ref() }
    }
    /// Pointer to the left path.
    ///
    /// Returns `None` if the pointer is null, and the pointer is null if
    /// [`ONE_BEST`](crate::RequestType::ONE_BEST) mode.
    pub fn lpath(&self) -> Option<&Path> {
        unsafe { self.lpath.as_ref() }
    }

    /// Status of this node.
    pub fn status(&self) -> NodeStatus {
        match self.stat {
            0 => NodeStatus::Normal,
            1 => NodeStatus::Unknown,
            2 => NodeStatus::Bos,
            3 => NodeStatus::Eos,
            _ => NodeStatus::EoNbest,
        }
    }

    /// Returns true if this node is best node. Equivalent to `MeCab::Node::isbest == 1`.
    pub fn is_best(&self) -> bool {
        self.isbest == 1
    }

    /// Length of the surface string. Equivalent to `MeCab::Node::length` (= [`Node::length`]).
    pub fn surface_len(&self) -> usize {
        self.length as _
    }

    /// Surface string. Equivalent to `MeCab::Node::surfase` with length `MeCab::Node::length`.
    pub fn surface(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.surface as *const u8, self.length as usize) }
    }
    /// Converts [`Node::surface()`] as a [`&str`].
    pub fn surface_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.surface())
    }
    /// Converts [`Node::surface()`] as a [`&str`].
    ///
    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn surface_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.surface())
    }

    /// Feature string. Equivalent to `MeCab::Node::feature` (without terminating `'\0'`).
    pub fn features(&self) -> &[u8] {
        unsafe {
            let s = CStr::from_ptr(self.feature);
            s.to_bytes()
        }
    }
    /// Converts [`Node::features()`] as a [`&str`].
    pub fn features_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.features())
    }
    /// Converts [`Node::features()`] as a [`&str`].
    ///
    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn features_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.features())
    }
    /// Returns an iterator of [`Node::features()`].
    ///
    /// This is the same as [`FeatureReader::from_node()`].
    pub fn feature_reader(&self) -> FeatureReader<'_> {
        FeatureReader::from_node(self)
    }
}

impl Path {
    /// Pointer to the next right path.
    ///
    /// It returns `None` if the pointer is null.
    pub fn rnext(&self) -> Option<&Self> {
        unsafe { self.rnext.as_ref() }
    }
    /// Pointer to the next left path.
    ///
    /// It returns `None` if the pointer is null.
    pub fn lnext(&self) -> Option<&Self> {
        unsafe { self.lnext.as_ref() }
    }

    /// Pointer to the right node.
    ///
    /// It returns `None` if the pointer is null.
    pub fn rnode(&self) -> Option<&Node> {
        unsafe { self.rnode.as_ref() }
    }
    /// Pointer to the left node.
    ///
    /// It returns `None` if the pointer is null.
    pub fn lnode(&self) -> Option<&Node> {
        unsafe { self.lnode.as_ref() }
    }
}
