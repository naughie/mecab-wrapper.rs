#[cfg(feature = "cmecab")]
use crate::ffi::Node;

use csv::ByteRecord;
use csv::ByteRecordIter;
use csv::Error as CsvError;
use csv::Reader as CsvReader;

use std::ops::Index;
use std::str::Utf8Error;

#[derive(Debug)]
pub struct FeatureReader<'a> {
    reader: CsvReader<&'a [u8]>,
}

impl<'a> FeatureReader<'a> {
    pub fn from_features(feats: &'a [u8]) -> Self {
        let reader = CsvReader::from_reader(feats);
        Self { reader }
    }

    #[cfg(feature = "cmecab")]
    #[inline]
    pub fn from_node(node: &'a Node) -> Self {
        Self::from_features(node.features())
    }

    pub fn features(&mut self) -> Result<Features<'_>, CsvError> {
        let record = self.reader.byte_headers()?;
        Ok(Features { record })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Features<'a> {
    record: &'a ByteRecord,
}

impl<'a> Features<'a> {
    pub fn as_csv_record(&self) -> &'a ByteRecord {
        self.record
    }

    pub fn is_empty(&self) -> bool {
        self.record.is_empty()
    }

    pub fn len(&self) -> usize {
        self.record.len()
    }

    pub fn get(&self, i: usize) -> Option<Feature<'_>> {
        self.record.get(i).map(|inner| Feature { inner })
    }

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

pub struct IntoIter<'a> {
    it: ByteRecordIter<'a>,
}

impl<'a> Iterator for IntoIter<'a> {
    type Item = Feature<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.it.next()?;
        Some(Feature { inner })
    }
}

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
