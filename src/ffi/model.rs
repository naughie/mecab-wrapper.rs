use super::{Attribute, DictionaryInfo, Lattice, ModelArgs, Node, Tagger};

use libc::{c_char, c_int, c_ushort};

use libc::c_void;
type VoidPtr = *mut c_void;

use std::ffi::CStr;
use std::ptr::NonNull;
use std::str::Utf8Error;

#[link(name = "cmecab")]
extern "C" {
    fn delete_model(model: VoidPtr);

    fn dictionary_info(model: VoidPtr) -> VoidPtr;
    fn model_version() -> *const c_char;

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

/// Wrapper of the
/// [`MeCab::Model`](https://taku910.github.io/mecab/doxygen/classMeCab_1_1Model.html) class.
pub struct Model {
    void_model: NonNull<c_void>,
}

unsafe impl Send for Model {}
unsafe impl Sync for Model {}

impl Model {
    /// Factory method to create a new Model.
    /// Equivalent to `MeCab::createModel()`.
    ///
    /// Returns `None` if the new model cannot be initialized. Use
    /// [`global_error()`](crate::global_error())
    /// ([`global_error_str()`](crate::global_error_str())) to obtain the cause of the errors.
    ///
    /// See [`ModelArgs`] for details and examples.
    pub fn new<Arg: ModelArgs>(arg: Arg) -> Option<Self> {
        let void_model = arg.create_model();
        let void_model = NonNull::new(void_model)?;
        Some(Self { void_model })
    }

    /// Dictionary information.
    pub fn dictionary_info(&self) -> &DictionaryInfo {
        unsafe {
            let info = dictionary_info(self.void_model.as_ptr());
            &*{ info as *const DictionaryInfo }
        }
    }

    /// Returns a version string.
    pub fn version<'v>() -> &'v [u8] {
        unsafe {
            let ver = model_version();
            let s = CStr::from_ptr(ver);
            s.to_bytes()
        }
    }

    /// Converts [`Model::version()`] as a [`&str`].
    pub fn version_str<'v>() -> Result<&'v str, Utf8Error> {
        std::str::from_utf8(Self::version())
    }

    /// Returns the transition cost from `rattr` to `lattr`.
    pub fn transition_cost(&self, rattr: Attribute, lattr: Attribute) -> c_int {
        unsafe { transition_cost(self.void_model.as_ptr(), rattr.0, lattr.0) }
    }

    /// Swaps the instance with `new_model`.
    ///
    /// Returns true if the model is swapped successfully.
    ///
    /// This method is thread safe. All taggers created by [`Model::create_tagger()`] will also be
    /// updated asynchronously. No need to stop the parsing thread explicitly before swapping model
    /// objects.
    pub fn swap(&mut self, new_model: Self) -> bool {
        unsafe { swap_model(self.void_model.as_ptr(), new_model.void_model.as_ptr()) }
    }

    /// Creates a new tagger object. Equivalent to `MeCab::Model::createTagger()`.
    ///
    /// Returns `None` if the new tagger cannot be initialized. Use
    /// [`global_error()`](crate::global_error())
    /// ([`global_error_str()`](crate::global_error_str())) to obtain the cause of the errors.
    ///
    /// All of the returned tagger share this model as a parsing model.
    pub fn create_tagger(&self) -> Option<Tagger<'_>> {
        unsafe {
            let tagger = new_tagger(self.void_model.as_ptr());
            Tagger::from_ptr(tagger)
        }
    }

    /// Creates a new lattice object. Equivalent to `MeCab::Model::createLattice()`.
    pub fn create_lattice(&self) -> Lattice<'_> {
        unsafe {
            let lattice = new_lattice(self.void_model.as_ptr());
            Lattice::from_ptr(lattice)
        }
    }

    /// Performs common prefix search. Equivalent to
    /// ```cpp
    /// const char *begin = prefix.as_ptr();
    /// const char *end = begin + prefix.len();
    /// MeCab::Model::lookup(begin, end, lattice)
    /// ```
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
