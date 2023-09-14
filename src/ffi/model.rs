use super::{Attribute, DictionaryInfo, Lattice, Node, Tagger};

use libc::{c_char, c_int, c_ushort};

use libc::c_void;
type VoidPtr = *mut c_void;

use std::ffi::CStr;
use std::ptr::NonNull;
use std::str::Utf8Error;

#[link(name = "cmecab")]
extern "C" {
    fn new_model(arg: *const c_char) -> VoidPtr;
    fn delete_model(model: VoidPtr);

    fn dictionary_info(model: VoidPtr) -> VoidPtr;
    fn model_version(model: VoidPtr) -> *const c_char;

    fn transition_cost(model: VoidPtr, rattr: c_ushort, lattr: c_ushort) -> c_int;

    fn swap_model(model: VoidPtr, new_model: VoidPtr) -> bool;

    fn new_tagger(model: VoidPtr) -> VoidPtr;
    fn new_lattice(model: VoidPtr) -> VoidPtr;

    fn model_lookup(
        model: VoidPtr,
        begin: *const c_char,
        end: *const c_char,
        lattice: VoidPtr,
    ) -> VoidPtr;
}

pub struct Model {
    void_model: NonNull<c_void>,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    pub fn new(arg: &str) -> Option<Self> {
        let mut v = Vec::with_capacity(arg.len() + 1);
        v.extend_from_slice(arg.as_bytes());
        v.push(b'\0');
        let arg = CStr::from_bytes_with_nul(&v).unwrap_or_default();

        unsafe {
            let void_model = new_model(arg.as_ptr());
            let void_model = NonNull::new(void_model)?;
            Some(Self { void_model })
        }
    }

    pub fn dictionary_info(&self) -> &DictionaryInfo {
        unsafe {
            let info = dictionary_info(self.void_model.as_ptr());
            &*{ info as *const DictionaryInfo }
        }
    }

    pub fn version(&self) -> &[u8] {
        unsafe {
            let ver = model_version(self.void_model.as_ptr());
            let s = CStr::from_ptr(ver);
            s.to_bytes()
        }
    }

    pub fn version_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.version())
    }

    pub fn transition_cost(&self, rattr: Attribute, lattr: Attribute) -> c_int {
        unsafe { transition_cost(self.void_model.as_ptr(), rattr.0, lattr.0) }
    }

    pub fn swap(&mut self, new_model: Self) -> bool {
        unsafe { swap_model(self.void_model.as_ptr(), new_model.void_model.as_ptr()) }
    }

    pub fn create_tagger(&self) -> Option<Tagger<'_>> {
        unsafe {
            let tagger = new_tagger(self.void_model.as_ptr());
            Tagger::from_ptr(tagger)
        }
    }

    pub fn create_lattice(&self) -> Lattice<'_> {
        unsafe {
            let lattice = new_lattice(self.void_model.as_ptr());
            Lattice::from_ptr(lattice)
        }
    }

    pub fn prefix_search<'a, 'b>(
        &'a self,
        prefix: &str,
        lattice: &'b mut Lattice<'a>,
    ) -> Option<&'b Node> {
        let begin = prefix.as_ptr() as *const c_char;
        unsafe {
            let end = begin.add(prefix.len());
            let node = model_lookup(self.void_model.as_ptr(), begin, end, lattice.as_mut_ptr());
            (node as *const Node).as_ref()
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            delete_model(self.void_model.as_ptr());
        }
    }
}
