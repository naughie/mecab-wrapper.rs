use libc::{c_char, c_int};

use libc::c_void;
type VoidPtr = *mut c_void;

use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::iter::once;

#[link(name = "cmecab")]
extern "C" {
    fn new_model_argv(argc: c_int, argv: *const *const c_char) -> VoidPtr;
    fn new_model_single(arg: *const c_char) -> VoidPtr;
}

/// `ModelArgs` represents arguments to `MeCab::createModel()`. It is used only in
/// [`Model::new()`](crate::Model::new()).
///
/// The function `createModel()` has two variants: `createModel(int argc, char **argv)` and
/// `createModel(const char *arg)`. A type that implements `ModelArgs` selects one of the two
/// signatures automatically.
///
/// Generally speaking, they have almost no differences in terms of memory allocation: the former
/// does heap allocation in the Rust implementation, while the latter does in MeCab (C++)
/// implementation.
///
/// # Implementations
///
/// ## `&CStr`/`&CString`/`CString`/`Cow<CStr>`
///
/// They just take pointers [`CStr::as_ptr()`] and call `createModel(const char *arg)`.
///
/// ## `u8`-array (`&[u8]`, `Vec<u8>`, `&str`, `String`, etc.)
///
/// If they end with a NULL character (`'\0'`), they are converted by
/// [`CStr::from_bytes_with_nul_unchecked()`] and call `impl ModelArgs` for `&CStr`.
/// In such a case, we do not need new heap allocation.
///
/// Otherwise, we need to append a NULL character, so `&[u8]`/`&str` require heap allocation.
///
/// ## Arary of `&CStr`/`&CString`/`CString`/`Cow<CStr>`
///
/// When we call `ModelArgs::create_model()` for an array `[c_str1, c_str2, ..., c_strN]`, it will be
/// equivalent to `createModel(int argc, char **argv)` for
/// ```txt
/// argc = N + 1
/// argv = ["cargo run\0", c_str1, ..., c_strN]
/// ```
///
/// Essentially the program name `argv[0]` is meaningless because it is used only when printing
/// usages, like `"Usage: PROG_NAME [options] files"`.
///
/// These implementations need heap allocation to convert `&[&Cstr]` into `Vec<*const c_char>`.
///
/// ## Array of `(OptionKey, value)` pairs
///
/// [`OptionKey`] represents keys of possible options. For example, a tupple `(OptionKey::Rcfile,
/// "./dicrc")` means a command-line argument `"--rcfile ./dicrc"`. [`OptionKey`]s are converted
/// into `&'static CStr`, which lie on the static memory, and `value`s are just taken the pointers
/// by [`CStr::as_ptr()`].
///
/// `ModelArgs::create_model()` for an array `[(OptionKey::Key1, value1), ..., (OptionKey::KeyN, valueN)]`
/// is equivalent to `<&[&CStr] as ModelArgs>::create_model()` for
/// ```txt
/// ["key1\0", value1, "key2\0", value2, ..., "keyN\0", valueN]
/// ```
/// which is in turn equivalent to
/// ```txt
/// argc = 2 * N + 1
/// argv = ["cargo run\0", "key1\0", value1, "key2\0", value2, ..., "keyN\0", valueN]
/// ```
///
/// ## Array of `*const c_char`
///
/// This implementation is used for a low-level wrapper of `createModel(int argc, char **argv)`. It
/// takes no conversion like prepending the program name (`"cargo run\0"`), nor heap allocation.
/// Therefore, the call `<&[*const c_char]>::create_model(array)` is simply equivalent to
/// ```txt
/// argc = array.len()
/// argv = array
/// ```
///
/// # Examples
///
/// ```no_run
/// use mecab_wrapper::{Model, OptionKey};
/// use std::ffi::{CStr, CString};
///
/// // without arguments
/// let model = Model::new("").unwrap();
/// let model = Model::new(b"\0").unwrap();
///
/// // as a single string
/// let model = Model::new("-d /usr/local/mecab/dic/ipadic -O wakati").unwrap();
/// let model = Model::new(b"--unk-feature UNK").unwrap();
/// let model = Model::new(CString::new("-d .").unwrap()).unwrap();
///
/// // NULL terminating string may reduce memory allocation
/// let model = Model::new("-d /usr/local/mecab/dic/ipadic -O wakati\0").unwrap();
/// let model = Model::new(b"--unk-feature UNK\0").unwrap();
///
/// // as an array
/// let dicdir_key = unsafe { CStr::from_bytes_with_nul_unchecked(b"-d\0") };
/// let dicdir_value = unsafe { CStr::from_bytes_with_nul_unchecked(b"/usr/local/mecab/dic/ipadic\0") };
///
/// let output_key = unsafe { CStr::from_bytes_with_nul_unchecked(b"-O\0") };
/// let output_value = unsafe { CStr::from_bytes_with_nul_unchecked(b"wakati\0") };
///
/// let model = Model::new(&[dicdir_key, dicdir_value, output_key, output_value]);
/// let model = Model::new(
///     &[(OptionKey::Dicdir, dicdir_value), (OptionKey::OutputFormatType, output_value)]
/// );
/// ```
pub trait ModelArgs {
    /// Wrapper of `MeCab::createModel()`.
    fn create_model(self) -> VoidPtr;
}

impl ModelArgs for &[u8] {
    fn create_model(self) -> VoidPtr {
        if self.ends_with(&[0]) {
            let arg = unsafe { CStr::from_bytes_with_nul_unchecked(self) };

            arg.create_model()
        } else {
            let mut v = Vec::with_capacity(self.len() + 1);
            v.extend_from_slice(self);
            v.push(b'\0');
            let arg = unsafe { CStr::from_bytes_with_nul_unchecked(&v) };

            arg.create_model()
        }
    }
}

impl<const N: usize> ModelArgs for &[u8; N] {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &Vec<u8> {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for Vec<u8> {
    fn create_model(mut self) -> VoidPtr {
        if !self.ends_with(&[0]) {
            self.push(b'\0');
        }
        let arg = unsafe { CStr::from_bytes_with_nul_unchecked(&self) };

        arg.create_model()
    }
}

impl ModelArgs for &str {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.as_bytes().create_model()
    }
}

impl ModelArgs for &String {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.as_str().create_model()
    }
}

impl ModelArgs for String {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.into_bytes().create_model()
    }
}

impl ModelArgs for &CStr {
    fn create_model(self) -> VoidPtr {
        unsafe { new_model_single(self.as_ptr()) }
    }
}

impl ModelArgs for &CString {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.as_c_str().create_model()
    }
}

impl ModelArgs for CString {
    #[inline]
    fn create_model(self) -> VoidPtr {
        self.as_c_str().create_model()
    }
}

struct IterInnerImpl<I>(I);

impl<'a, I: Iterator<Item = &'a CStr>> ModelArgs for IterInnerImpl<I> {
    #[inline]
    fn create_model(self) -> VoidPtr {
        let v: Vec<*const c_char> = self.0.map(|s| s.as_ptr()).collect();
        unsafe { new_model_argv(v.len() as _, v.as_ptr()) }
    }
}

// For system_name in MeCab::init_param()
// It is used only for printing usage ("Usage: SYSTEM_NAME [options] files")
const SYSTEM_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"cargo run\0") };

impl ModelArgs for &[*const c_char] {
    fn create_model(self) -> VoidPtr {
        unsafe { new_model_argv(self.len() as _, self.as_ptr()) }
    }
}
impl<const N: usize> ModelArgs for &[*const c_char; N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[&CStr] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().copied());
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[&CStr; N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[&CString] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().map(|s| s.as_c_str()));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[&CString; N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[CString] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().map(|s| s.as_c_str()));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[CString; N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[Cow<'_, CStr>] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().map(|s| s.as_ref()));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[Cow<'_, CStr>; N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

/// Represents keys of options for `MeCab::createModel()`. It can be used as an argument to
/// [`Model::new()`](crate::Model::new()) (see [`ModelArgs`]).
///
/// Some options such as `help` are omitted because they have no effects for `lib` usage of MeCab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptionKey {
    /// Path of a resource file.
    Rcfile,
    /// Path of a system dictionary dir.
    Dicdir,
    /// Path of a user dictionary file.
    Userdic,
    /// Output format type ("wakati", "none", etc.).
    OutputFormatType,
    /// Max grouping size for unknown words (int).
    MaxGroupingSize,
    /// User-defined node format.
    NodeFormat,
    /// User-defined unknown node format.
    UnkFormat,
    /// User-defined beginning-of-sentence format.
    BosFormat,
    /// User-defined end-of-sentence format.
    EosFormat,
    /// User-defined end-of-NBest format.
    EonFormat,
    /// Feature for unknown words.
    UnkFeature,
    /// Input buffer size (int).
    InputBufferSize,
    /// Cost factor (int).
    CostFactor,
}

impl OptionKey {
    #[inline]
    fn as_bytes_with_null(self) -> &'static [u8] {
        use self::OptionKey::*;

        match self {
            Rcfile => b"rcfile\0",
            Dicdir => b"dicdir\0",
            Userdic => b"userdic\0",
            OutputFormatType => b"output-format-type\0",
            MaxGroupingSize => b"max-grouping-size\0",
            NodeFormat => b"node-format\0",
            UnkFormat => b"unk-format\0",
            BosFormat => b"bos-format\0",
            EosFormat => b"eos-format\0",
            EonFormat => b"eon-format\0",
            UnkFeature => b"unk-feature\0",
            InputBufferSize => b"input-buffer-size\0",
            CostFactor => b"cost-factor\0",
        }
    }

    fn as_c_str(self) -> &'static CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes_with_null()) }
    }
}

fn twice(first: OptionKey, second: &CStr) -> impl Iterator<Item = &CStr> {
    once(first.as_c_str()).chain(once(second))
}

impl ModelArgs for &[(OptionKey, &CStr)] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().flat_map(|&(key, val)| twice(key, val)));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[(OptionKey, &CStr); N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[(OptionKey, &CString)] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().flat_map(|&(key, val)| twice(key, val)));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[(OptionKey, &CString); N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[(OptionKey, CString)] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().flat_map(|&(key, ref val)| twice(key, val)));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[(OptionKey, CString); N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}

impl ModelArgs for &[(OptionKey, Cow<'_, CStr>)] {
    fn create_model(self) -> VoidPtr {
        let it = once(SYSTEM_NAME).chain(self.iter().flat_map(|&(key, ref val)| twice(key, val)));
        IterInnerImpl(it).create_model()
    }
}
impl<const N: usize> ModelArgs for &[(OptionKey, Cow<'_, CStr>); N] {
    fn create_model(self) -> VoidPtr {
        self.as_slice().create_model()
    }
}
