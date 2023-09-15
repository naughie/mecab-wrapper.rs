use super::Node;
use super::RequestType;
use crate::NodeIter;

use libc::{c_char, c_int, size_t};

use libc::c_void;
type VoidPtr = *mut c_void;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ops::ControlFlow;
use std::str::Utf8Error;

#[link(name = "cmecab")]
extern "C" {
    fn delete_lattice(lattice: VoidPtr);

    fn set_sentence(lattice: VoidPtr, sentence: *const c_char, len: size_t);

    fn lattice_to_string(lattice: VoidPtr) -> *const c_char;

    fn bos_node(lattice: VoidPtr) -> VoidPtr;
    fn eos_node(lattice: VoidPtr) -> VoidPtr;

    fn next_lattice(lattice: VoidPtr) -> bool;

    fn get_request_type(lattice: VoidPtr) -> c_int;
    fn set_request_type(lattice: VoidPtr, request_type: c_int);
    fn add_request_type(lattice: VoidPtr, request_type: c_int);
    fn remove_request_type(lattice: VoidPtr, request_type: c_int);

    fn lattice_what(lattice: VoidPtr) -> *const c_char;
    fn set_lattice_what(lattice: VoidPtr, what: *const c_char);
}

pub struct Lattice<'a> {
    void_lattice: VoidPtr,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Lattice<'a> {
    pub(crate) fn from_ptr(void_lattice: VoidPtr) -> Self {
        Self {
            void_lattice,
            phantom: PhantomData,
        }
    }
    pub(crate) fn as_mut_ptr(&mut self) -> VoidPtr {
        self.void_lattice
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
