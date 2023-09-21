#[cfg(feature = "cmecab")]
use crate::ffi::Node;

use csv::ByteRecord;
use csv::ByteRecordIter;
use csv::Error as CsvError;
use csv::Reader as CsvReader;

use std::ops::Index;
use std::str::Utf8Error;

/// Helper to create [`Features`]. This is a wrapper of [`csv::Reader`].
///
/// This struct is necessary because the [`csv::Reader::byte_headers()`] returns a *reference* to
/// headers. If it implements a method like `into_byte_headers()`, we can get rid of
/// `FeatureReader`.
///
/// # Examples
///
/// ```
/// use mecab_wrapper::{Feature, FeatureReader};
///
/// let mut reader = FeatureReader::from_features(b"a,b,c");
/// let feats = reader.features().unwrap();
/// let mut feats_it = feats.iter();
///
/// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"a")));
/// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"b")));
/// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"c")));
/// assert_eq!(feats_it.next(), None);
/// ```
///
/// It can be also created by [`Node::feature_reader()`].
///
/// ```no_run
/// # use mecab_wrapper::Node;
///
/// # fn test(node: &Node) {
/// let mut reader = node.feature_reader();
/// for feat in reader.features().unwrap() {
///     println!("{feat:?}");
/// }
/// # }
/// ```
///
#[derive(Debug)]
pub struct FeatureReader<'a> {
    reader: CsvReader<&'a [u8]>,
}

impl<'a> FeatureReader<'a> {
    /// Initializes with the given feature string.
    ///
    /// ```
    /// use mecab_wrapper::{Feature, FeatureReader};
    ///
    /// let mut reader = FeatureReader::from_features(b"a,b,c");
    /// let feats = reader.features().unwrap();
    /// let mut feats_it = feats.iter();
    ///
    /// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"a")));
    /// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"b")));
    /// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"c")));
    /// assert_eq!(feats_it.next(), None);
    /// ```
    pub fn from_features(feats: &'a [u8]) -> Self {
        let reader = CsvReader::from_reader(feats);
        Self { reader }
    }

    /// Same as [`Self::from_features(node.features())`](Self::from_features()).
    #[cfg(feature = "cmecab")]
    #[inline]
    pub fn from_node(node: &'a Node) -> Self {
        Self::from_features(node.features())
    }

    /// Returns an [`IntoIterator`] of  [`Feature`].
    pub fn features(&mut self) -> Result<Features<'_>, CsvError> {
        let record = self.reader.byte_headers()?;
        Ok(Features { record })
    }
}

/// [`IntoIterator`] of [`Feature`]. This is a wrapper of [`&csv::ByteRecord`](csv::ByteRecord).
///
/// `Features` is returned by [`FeatureReader::features()`]. See [`FeatureReader`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Features<'a> {
    record: &'a ByteRecord,
}

impl<'a> Features<'a> {
    /// Returns the underlying [`ByteRecord`].
    pub fn as_csv_record(&self) -> &'a ByteRecord {
        self.record
    }

    /// Returns true if `self` has no features.
    ///
    /// ```
    /// use mecab_wrapper::FeatureReader;
    ///
    /// let mut reader = FeatureReader::from_features(b"");
    /// let feats = reader.features().unwrap();
    /// assert!(feats.is_empty());
    ///
    /// let mut reader = FeatureReader::from_features(b"a,b,c");
    /// let feats = reader.features().unwrap();
    /// assert!(!feats.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.record.is_empty()
    }

    /// The number of features `self` has.
    ///
    /// ```
    /// use mecab_wrapper::FeatureReader;
    ///
    /// let mut reader = FeatureReader::from_features(b"a,b,c");
    /// let feats = reader.features().unwrap();
    /// assert_eq!(feats.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.record.len()
    }

    /// Gets `i`-th feature. It returns `None` if the index is out-of-bound.
    ///
    /// ```
    /// use mecab_wrapper::{Feature, FeatureReader};
    ///
    /// let mut reader = FeatureReader::from_features(b"a,b,c");
    /// let feats = reader.features().unwrap();
    /// assert_eq!(feats.get(0), Some(Feature::from_inner(b"a")));
    /// assert_eq!(feats.get(1), Some(Feature::from_inner(b"b")));
    /// assert_eq!(feats.get(2), Some(Feature::from_inner(b"c")));
    /// assert_eq!(feats.get(3), None);
    /// ```
    pub fn get(&self, i: usize) -> Option<Feature<'_>> {
        self.record.get(i).map(|inner| Feature { inner })
    }

    /// Returns an iterator of [`Feature`].
    ///
    /// ```
    /// use mecab_wrapper::{Feature, FeatureReader};
    ///
    /// let mut reader = FeatureReader::from_features(b"a,b,c");
    /// let feats = reader.features().unwrap();
    /// let mut feats_it = feats.iter();
    /// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"a")));
    /// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"b")));
    /// assert_eq!(feats_it.next(), Some(Feature::from_inner(b"c")));
    /// assert_eq!(feats_it.next(), None);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = Feature<'_>> {
        self.record.iter().map(|inner| Feature { inner })
    }
}

impl Index<usize> for Features<'_> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(self.record, index)
    }
}

impl<'a> IntoIterator for Features<'a> {
    type Item = Feature<'a>;
    type IntoIter = IntoIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let it = self.record.into_iter();
        IntoIter { it }
    }
}

/// Return type of [`Features::into_iter()`].
pub struct IntoIter<'a> {
    it: ByteRecordIter<'a>,
}

impl<'a> Iterator for IntoIter<'a> {
    type Item = Feature<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.it.next()?;
        Some(Feature { inner })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }

    #[inline]
    fn count(self) -> usize {
        self.it.count()
    }
}

impl<'a> DoubleEndedIterator for IntoIter<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let inner = self.it.next_back()?;
        Some(Feature { inner })
    }
}

impl<'a> ExactSizeIterator for IntoIter<'a> {}

/// Represents a feature. This is created by [`Features`] or [`FeaturesIntoIter`](IntoIter).
///
/// A `Feature` is a component of the features of each [`Node`](crate::Node).
///
/// # Examples
///
/// ```
/// use mecab_wrapper::Feature;
///
/// let feat = Feature::from_inner(b"*");
/// assert_eq!(feat.as_bytes(), b"*");
/// assert_eq!(feat.as_str(), Ok("*"));
///
/// // Invalid UTF-8
/// let feat = Feature::from_inner(&[150]);
/// assert_eq!(feat.as_bytes(), &[150]);
/// assert!(feat.as_str().is_err());
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Feature<'a> {
    inner: &'a [u8],
}

impl<'a> Feature<'a> {
    #[inline]
    pub fn from_inner(inner: &'a [u8]) -> Self {
        Self { inner }
    }

    #[inline]
    pub fn as_bytes(self) -> &'a [u8] {
        self.inner
    }

    #[inline]
    pub fn as_str(self) -> Result<&'a str, Utf8Error> {
        std::str::from_utf8(self.inner)
    }
}
