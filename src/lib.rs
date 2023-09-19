//! # Async example
//!
//! Examples below use [`futures`](https://docs.rs/futures/latest/futures/index.html) and
//! [`tokio`](https://docs.rs/tokio/latest/tokio/index.html) crates.
//!
//! ```toml
//! tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
//! futures = { version = "0.3" }
//! ```
//!
//! Tasks can share the same tagger:
//!
//! ```no_run
//! use mecab_wrapper::{Model, Tagger, global_error_str};
//!
//! use futures::stream::{FuturesUnordered, StreamExt};
//!
//! #[tokio::main]
//! async fn main() {
//!     let inputs = ["Foo.", "Bar."];
//!
//!     let model = if let Some(model) = Model::new("-d /path/to/model/dir") {
//!         model
//!     } else {
//!         println!("null model: {}", global_error_str().unwrap());
//!         return;
//!     };
//!     let tagger = if let Some(tagger) = model.create_tagger() {
//!         tagger
//!     } else {
//!         println!("null tagger: {}", global_error_str().unwrap());
//!         return;
//!     };
//!
//!     let mut tasks = FuturesUnordered::new();
//!
//!     for i in 0..2 {
//!         let task = test_model(&model, &tagger, inputs[i]);
//!         tasks.push(task);
//!     }
//!
//!     while tasks.next().await.is_some() {}
//! }
//!
//! async fn test_model(model: &Model, tagger: &Tagger<'_>, input: &str) {
//!     let mut lattice = model.create_lattice();
//!     lattice.set_sentence(input);
//!     if !tagger.parse(&mut lattice) {
//!         println!(
//!             "could not parse into the lattice: {}",
//!             global_error_str().unwrap()
//!         );
//!         return;
//!     }
//!     let lattice = lattice;
//!
//!     let node_it = lattice.iter_nodes();
//!     for node in node_it {
//!         println!(
//!             "{:?} {} {}",
//!             node.status(),
//!             node.surface_str().unwrap(),
//!             node.features_str().unwrap()
//!         );
//!     }
//! }
//! ```
//!
//! You can create taggers per thread:
//!
//! ```no_run
//! use mecab_wrapper::{Model, Tagger, global_error_str};
//!
//! use futures::stream::{FuturesUnordered, StreamExt};
//!
//! #[tokio::main]
//! async fn main() {
//!     let inputs = ["Foo.", "Bar."];
//!
//!     let model = if let Some(model) = Model::new("-d /path/to/model/dir") {
//!         model
//!     } else {
//!         println!("null model: {}", global_error_str().unwrap());
//!         return;
//!     };
//!
//!     let mut tasks = FuturesUnordered::new();
//!
//!     for i in 0..2 {
//!         let task = test_model(&model, inputs[i]);
//!         tasks.push(task);
//!     }
//!
//!     while tasks.next().await.is_some() {}
//! }
//!
//! async fn test_model(model: &Model, input: &str) {
//!     let lattice = {
//!         let tagger = if let Some(tagger) = model.create_tagger() {
//!             tagger
//!         } else {
//!             println!("null tagger: {}", global_error_str().unwrap());
//!             return;
//!         };
//!
//!         let mut lattice = model.create_lattice();
//!         lattice.set_sentence(input);
//!         if !tagger.parse(&mut lattice) {
//!             println!(
//!                 "could not parse into the lattice: {}",
//!                 global_error_str().unwrap()
//!             );
//!             return;
//!         }
//!         lattice
//!     };
//!
//!     let node_it = lattice.iter_nodes();
//!     for node in node_it {
//!         println!(
//!             "{:?} {} {}",
//!             node.status(),
//!             node.surface_str().unwrap(),
//!             node.features_str().unwrap()
//!         );
//!     }
//! }
//! ```
//!
//! # Useful iterators
//!
//! `mecab-wrapper` defines two useful iterators: [`NodeIter`] and [`Features`]
//! ([`FeaturesIntoIter`]).
//!
//! ## Node iterator
//!
//! Iterations A-E are all equivalent:
//!
//! ```no_run
//! # use mecab_wrapper::Lattice;
//! # use mecab_wrapper::Node;
//! # use mecab_wrapper::NodeIter;
//! # fn do_something(node: &Node) {}
//! # fn test_node_iter(lattice: Lattice<'_>) {
//! // Iteration A
//! for node in lattice.iter_nodes() {
//!     do_something(node);
//! }
//!
//! // Iteration B
//! for node in NodeIter::from_bos(&lattice) {
//!     do_something(node);
//! }
//!
//! // Iteration C
//! for node in NodeIter::from_node_option(lattice.bos_node()) {
//!     do_something(node);
//! }
//!
//! // Iteration D
//! if let Some(bos) = lattice.bos_node() {
//!     for node in NodeIter::from_node(bos) {
//!         do_something(node);
//!     }
//! }
//!
//! // Iteration E
//! if let Some(bos) = lattice.bos_node() {
//!     let mut node = bos;
//!     do_something(node);
//!     while let Some(next) = node.next() {
//!         node = next;
//!         do_something(node);
//!     }
//! }
//! # }
//! ```
//!
//! # Feature iterator
//!
//! Since the `node.features()` (`node.features_str()`) is a comma-separated string of features,
//! [`FeatureReader`] simply parse it using [`csv`] crate, and [`Features`] is no more than a
//! wrapper of [`csv::ByteRecord`].
//!
//! ```no_run
//! # use mecab_wrapper::Node;
//! # fn test_feat_iter(node: &Node) -> Result<(), Box<dyn std::error::Error>> {
//! use mecab_wrapper::Feature;
//! use mecab_wrapper::FeatureReader;
//!
//! fn print_feature(feat: Feature<'_>) {
//!     println!("{} ({:?})", feat.as_str().unwrap(), feat.as_bytes());
//! }
//!
//! let mut reader = node.feature_reader();
//! for feat in reader.features()? {
//!     print_feature(feat);
//! }
//!
//! // Equivalent to:
//! let mut reader = FeatureReader::from_node(node);
//! for feat in reader.features()? {
//!     print_feature(feat);
//! }
//!
//! // Another alternative:
//! let mut reader = FeatureReader::from_features(node.features());
//! for feat in reader.features()? {
//!     print_feature(feat);
//! }
//! # Ok(())
//! # }
//! ```

#[cfg(feature = "cmecab")]
mod ffi;
#[cfg(feature = "cmecab")]
pub use ffi::*;

#[cfg(feature = "cmecab")]
mod node_iter;
#[cfg(feature = "cmecab")]
pub use node_iter::NodeIter;

mod feat;
pub use feat::Feature;
pub use feat::FeatureReader;
pub use feat::Features;
pub use feat::IntoIter as FeaturesIntoIter;
pub type FeatureError = csv::Error;
