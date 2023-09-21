use super::Node;
use super::RequestType;
use crate::{NodeIter, NodeRevIter};

use libc::{c_char, c_double, c_float, c_int, size_t};

use libc::c_void;
type VoidPtr = *mut c_void;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::str::Utf8Error;

#[link(name = "cmecab")]
extern "C" {
    fn new_lattice_standalone() -> VoidPtr;

    fn delete_lattice(lattice: VoidPtr);

    fn lattice_sentence(lattice: VoidPtr) -> *const c_char;
    fn lattice_sentence_size(lattice: VoidPtr) -> size_t;
    fn set_sentence(lattice: VoidPtr, sentence: *const c_char, len: size_t);

    fn lattice_to_string(lattice: VoidPtr) -> *const c_char;
    fn lattice_to_string_alloc(lattice: VoidPtr, buf: *mut c_char, size: size_t) -> *const c_char;
    fn nbest_string(lattice: VoidPtr, n: size_t) -> *const c_char;
    fn nbest_string_alloc(
        lattice: VoidPtr,
        n: size_t,
        buf: *mut c_char,
        size: size_t,
    ) -> *const c_char;
    fn node_string(lattice: VoidPtr, node: *const c_void) -> *const c_char;
    fn node_string_alloc(
        lattice: VoidPtr,
        node: *const c_void,
        buf: *mut c_char,
        size: size_t,
    ) -> *const c_char;

    fn bos_node(lattice: VoidPtr) -> VoidPtr;
    fn eos_node(lattice: VoidPtr) -> VoidPtr;

    fn next_lattice(lattice: VoidPtr) -> bool;

    fn is_available(lattice: VoidPtr) -> bool;

    fn clear_lattice(lattice: VoidPtr);

    fn get_request_type(lattice: VoidPtr) -> c_int;
    fn set_request_type(lattice: VoidPtr, request_type: c_int);
    fn add_request_type(lattice: VoidPtr, request_type: c_int);
    fn remove_request_type(lattice: VoidPtr, request_type: c_int);

    fn lattice_norm_factor(lattice: VoidPtr) -> c_double;
    fn lattice_set_norm_factor(lattice: VoidPtr, norm: c_double);
    fn lattice_theta(lattice: VoidPtr) -> c_float;
    fn lattice_set_theta(lattice: VoidPtr, theta: c_float);

    fn lattice_has_constraint(lattice: VoidPtr) -> bool;
    fn lattice_boundary_constraint(lattice: VoidPtr, pos: size_t) -> c_int;
    fn lattice_set_boundary_constraint(lattice: VoidPtr, pos: size_t, boundary: c_int);
    fn lattice_feature_constraint(lattice: VoidPtr, pos: size_t) -> *const c_char;
    fn lattice_set_feature_constraint(
        lattice: VoidPtr,
        begin_pos: size_t,
        end_pos: size_t,
        feature: *const c_char,
    );

    fn lattice_set_result(lattice: VoidPtr, result: *const c_char);

    fn new_node(lattice: VoidPtr) -> VoidPtr;

    fn lattice_what(lattice: VoidPtr) -> *const c_char;
    fn set_lattice_what(lattice: VoidPtr, what: *const c_char);
}

pub enum Boundary {
    /// The token boundary is not specified.
    NotSpecified,
    /// The position is a strong token boundary.
    Token,
    /// The position is not a token boundary.
    InsideToken,
}

pub struct Lattice<'a> {
    void_lattice: VoidPtr,
    phantom: PhantomData<&'a ()>,
}

impl Default for Lattice<'_> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Lattice<'a> {
    pub fn new() -> Self {
        unsafe {
            let void_lattice = new_lattice_standalone();
            Self::from_ptr(void_lattice)
        }
    }

    #[inline]
    pub(crate) fn from_ptr(void_lattice: VoidPtr) -> Self {
        Self {
            void_lattice,
            phantom: PhantomData,
        }
    }
    #[inline]
    pub(crate) fn as_mut_ptr(&mut self) -> VoidPtr {
        self.void_lattice
    }

    pub fn sentence_len(&self) -> usize {
        unsafe { lattice_sentence_size(self.void_lattice) }
    }

    pub fn sentence(&mut self) -> &[u8] {
        unsafe {
            let s = lattice_sentence(self.void_lattice);
            std::slice::from_raw_parts(s as _, self.sentence_len())
        }
    }

    pub fn sentence_str(&mut self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.sentence())
    }

    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn sentence_str_unchecked(&mut self) -> &str {
        std::str::from_utf8_unchecked(self.sentence())
    }

    pub fn set_sentence(&mut self, sentence: &str) {
        let ptr = sentence.as_ptr();
        let len = sentence.len();
        unsafe { set_sentence(self.void_lattice, ptr as _, len as _) }
    }

    pub fn to_bytes(&self) -> &[u8] {
        unsafe {
            let s = lattice_to_string(self.void_lattice);
            let s = CStr::from_ptr(s);
            s.to_bytes()
        }
    }
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.to_bytes())
    }
    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn to_str_unchecked(&self) -> &str {
        std::str::from_utf8_unchecked(self.to_bytes())
    }

    /// # Safety
    /// This method is unsafe because MeCab internally deletes `buf` and re-allocates new buffer.
    /// So, use this method *only if* you are sure i) which allocator MeCab uses and ii) by which
    /// allocator a `buf` is created.
    pub unsafe fn to_bytes_buffer(&self, buf: &mut Vec<u8>) {
        lattice_to_string_alloc(self.void_lattice, buf.as_mut_ptr() as _, buf.len());
    }

    pub fn nbest_to_bytes(&self, n: usize) -> &[u8] {
        unsafe {
            let s = nbest_string(self.void_lattice, n);
            let s = CStr::from_ptr(s);
            s.to_bytes()
        }
    }
    pub fn nbest_to_str(&self, n: usize) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.nbest_to_bytes(n))
    }
    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn nbest_to_str_unchecked(&self, n: usize) -> &str {
        std::str::from_utf8_unchecked(self.nbest_to_bytes(n))
    }

    /// # Safety
    /// This method is unsafe because MeCab internally deletes `buf` and re-allocates new buffer.
    /// So, use this method *only if* you are sure i) which allocator MeCab uses and ii) by which
    /// allocator a `buf` is created.
    pub unsafe fn nbest_to_bytes_buffer(&self, n: usize, buf: &mut Vec<u8>) {
        nbest_string_alloc(self.void_lattice, n, buf.as_mut_ptr() as _, buf.len());
    }

    pub fn node_to_bytes<'b>(&'b self, node: &'b Node) -> &[u8] {
        unsafe {
            let s = node_string(self.void_lattice, node as *const Node as _);
            let s = CStr::from_ptr(s);
            s.to_bytes()
        }
    }
    pub fn node_to_str<'b>(&'b self, node: &'b Node) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.node_to_bytes(node))
    }
    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn node_to_str_unchecked<'b>(&'b self, node: &'b Node) -> &str {
        std::str::from_utf8_unchecked(self.node_to_bytes(node))
    }

    /// # Safety
    /// This method is unsafe because MeCab internally deletes `buf` and re-allocates new buffer.
    /// So, use this method *only if* you are sure i) which allocator MeCab uses and ii) by which
    /// allocator a `buf` is created.
    pub unsafe fn node_to_bytes_buffer<'b>(&'b self, node: &'b Node, buf: &mut Vec<u8>) {
        node_string_alloc(
            self.void_lattice,
            node as *const Node as _,
            buf.as_mut_ptr() as _,
            buf.len(),
        );
    }

    pub fn bos_node(&self) -> Option<&Node> {
        unsafe {
            let node = bos_node(self.void_lattice);
            (node as *const Node).as_ref()
        }
    }
    pub fn eos_node(&self) -> Option<&Node> {
        unsafe {
            let node = eos_node(self.void_lattice);
            (node as *const Node).as_ref()
        }
    }

    #[inline]
    pub fn iter_nodes(&self) -> NodeIter<'_> {
        NodeIter::from_bos(self)
    }

    #[inline]
    pub fn iter_nodes_rev(&self) -> NodeRevIter<'_> {
        NodeRevIter::from_eos(self)
    }

    pub fn get_request_type(&mut self) -> RequestType {
        unsafe {
            let req = get_request_type(self.void_lattice);
            RequestType::from_int(req as _)
        }
    }

    pub fn set_request_type(&mut self, req: RequestType) {
        unsafe {
            let req = req.to_int() as _;
            set_request_type(self.void_lattice, req);
        }
    }

    pub fn add_request_type(&mut self, req: RequestType) {
        unsafe {
            let req = req.to_int() as _;
            add_request_type(self.void_lattice, req);
        }
    }

    pub fn remove_request_type(&mut self, req: RequestType) {
        unsafe {
            let req = req.to_int() as _;
            remove_request_type(self.void_lattice, req);
        }
    }

    pub fn next_nbest(&mut self) -> ControlFlow<(), ()> {
        unsafe {
            if next_lattice(self.void_lattice) {
                ControlFlow::Continue(())
            } else {
                ControlFlow::Break(())
            }
        }
    }

    pub fn norm_factor(&self) -> f64 {
        unsafe { lattice_norm_factor(self.void_lattice) }
    }

    pub fn set_norm_factor(&mut self, norm: f64) {
        unsafe {
            lattice_set_norm_factor(self.void_lattice, norm);
        }
    }

    pub fn theta(&self) -> f32 {
        unsafe { lattice_theta(self.void_lattice) }
    }

    pub fn set_theta(&mut self, theta: f32) {
        unsafe {
            lattice_set_theta(self.void_lattice, theta);
        }
    }

    pub fn has_constraint(&self) -> bool {
        unsafe { lattice_has_constraint(self.void_lattice) }
    }

    pub fn boundary_constraint(&self, pos: usize) -> Boundary {
        unsafe {
            let boundary = lattice_boundary_constraint(self.void_lattice, pos);
            match boundary {
                1 => Boundary::Token,
                2 => Boundary::InsideToken,
                _ => Boundary::NotSpecified,
            }
        }
    }

    pub fn set_boundary_constraint(&mut self, pos: usize, boundary: Boundary) {
        unsafe {
            let boundary = match boundary {
                Boundary::NotSpecified => 0,
                Boundary::Token => 1,
                Boundary::InsideToken => 2,
            };
            lattice_set_boundary_constraint(self.void_lattice, pos, boundary);
        }
    }

    pub fn feature_constraint(&self, pos: usize) -> &[u8] {
        unsafe {
            let features = lattice_feature_constraint(self.void_lattice, pos);
            let features = CStr::from_ptr(features);
            features.to_bytes()
        }
    }

    pub fn feature_constraint_str(&self, pos: usize) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.feature_constraint(pos))
    }

    /// # Safety
    /// See [`std::str::from_utf8_unchecked()`].
    pub unsafe fn feature_constraint_str_unchecked(&self, pos: usize) -> &str {
        std::str::from_utf8_unchecked(self.feature_constraint(pos))
    }

    pub fn set_feature_constraint(&mut self, begin: usize, end: usize, features: &CStr) {
        unsafe {
            lattice_set_feature_constraint(self.void_lattice, begin, end, features.as_ptr());
        }
    }

    pub fn set_result(&mut self, result: &CStr) {
        unsafe {
            lattice_set_result(self.void_lattice, result.as_ptr());
        }
    }

    pub fn new_node(&mut self) -> Option<&Node> {
        unsafe {
            let node = new_node(self.void_lattice);
            (node as *const Node).as_ref()
        }
    }

    pub fn clear(&mut self) {
        unsafe { clear_lattice(self.void_lattice) }
    }

    pub fn is_available(&self) -> bool {
        unsafe { is_available(self.void_lattice) }
    }

    pub fn error(&self) -> &[u8] {
        unsafe {
            let e = lattice_what(self.void_lattice);
            let s = CStr::from_ptr(e);
            s.to_bytes()
        }
    }

    pub fn error_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.error())
    }

    pub fn set_error(&mut self, e: &CStr) {
        unsafe { set_lattice_what(self.void_lattice, e.as_ptr()) }
    }
}

impl Drop for Lattice<'_> {
    fn drop(&mut self) {
        unsafe {
            delete_lattice(self.void_lattice);
        }
    }
}
