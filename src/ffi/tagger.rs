use super::Lattice;

use libc::c_char;
use libc::c_void;
type VoidPtr = *mut c_void;

use std::ffi::CStr;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::str::Utf8Error;

#[link(name = "cmecab")]
extern "C" {
    fn delete_tagger(tagger: VoidPtr);

    fn parse(tagger: VoidPtr, lattice: VoidPtr) -> bool;

    fn tagger_what(tagger: VoidPtr) -> *const c_char;
    fn tagger_version(tagger: VoidPtr) -> *const c_char;
}

pub struct Tagger<'a> {
    void_tagger: NonNull<c_void>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> Tagger<'a> {
    pub(crate) fn from_ptr(void_tagger: VoidPtr) -> Option<Self> {
        let void_tagger = NonNull::new(void_tagger)?;
        Some(Self {
            void_tagger,
            phantom: PhantomData,
        })
    }

    pub fn parse(&self, lattice: &mut Lattice) -> bool {
        unsafe { parse(self.void_tagger.as_ptr(), lattice.as_mut_ptr()) }
    }

    pub fn error(&self) -> &[u8] {
        unsafe {
            let e = tagger_what(self.void_tagger.as_ptr());
            let s = CStr::from_ptr(e);
            s.to_bytes()
        }
    }

    pub fn error_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.error())
    }

    pub fn version(&self) -> &[u8] {
        unsafe {
            let ver = tagger_version(self.void_tagger.as_ptr());
            let s = CStr::from_ptr(ver);
            s.to_bytes()
        }
    }

    pub fn version_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.version())
    }
}

unsafe impl Send for Tagger<'_> {}
unsafe impl Sync for Tagger<'_> {}

impl Drop for Tagger<'_> {
    fn drop(&mut self) {
        unsafe {
            delete_tagger(self.void_tagger.as_ptr());
        }
    }
}
