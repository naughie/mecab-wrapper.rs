mod dictionary_info;
pub use dictionary_info::DictionaryInfo;
pub use dictionary_info::DictionaryType;

mod model;
pub use model::Model;

mod tagger;
pub use tagger::Tagger;

mod lattice;
pub use lattice::Lattice;

mod node;
pub use node::Attribute;
pub use node::Node;
pub use node::NodeStatus;
pub use node::Path;

mod request_type;
pub use request_type::RequestType;

use libc::c_char;

use std::ffi::CStr;
use std::str::Utf8Error;

#[link(name = "cmecab")]
extern "C" {
    fn get_global_error() -> *const c_char;
}

/// Returns the error string.
///
/// Equivalent to `MeCab::getLastError()`.
pub fn global_error<'a>() -> &'a [u8] {
    unsafe {
        let e = get_global_error();
        let s = CStr::from_ptr(e);
        s.to_bytes()
    }
}

/// Converts [`global_error()`] as a [`&str`].
pub fn global_error_str<'a>() -> Result<&'a str, Utf8Error> {
    std::str::from_utf8(global_error())
}
