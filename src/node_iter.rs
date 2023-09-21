use crate::ffi::{Lattice, Node};

/// Iterates nodes forward.
///
/// The iterations below are equivalent:
///
/// ```no_run
/// # use mecab_wrapper::Lattice;
/// # use mecab_wrapper::Node;
/// # use mecab_wrapper::NodeIter;
/// # fn do_something(node: &Node) {}
/// # fn test_node_iter(lattice: Lattice<'_>) {
/// // Iteration A
/// for node in lattice.iter_nodes() {
///     do_something(node);
/// }
///
/// // Iteration B
/// for node in NodeIter::from_bos(&lattice) {
///     do_something(node);
/// }
///
/// // Iteration C
/// for node in NodeIter::from_node_option(lattice.bos_node()) {
///     do_something(node);
/// }
///
/// // Iteration D
/// if let Some(bos) = lattice.bos_node() {
///     for node in NodeIter::from_node(bos) {
///         do_something(node);
///     }
/// }
///
/// // Iteration E
/// if let Some(bos) = lattice.bos_node() {
///     let mut node = bos;
///     do_something(node);
///     while let Some(next) = node.next() {
///         node = next;
///         do_something(node);
///     }
/// }
/// # }
pub struct NodeIter<'a> {
    node: Option<&'a Node>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = node.next();
        Some(node)
    }
}

impl<'a> NodeIter<'a> {
    /// Returns the current node (= the node which the next [`Iterator::next()`] will return).
    ///
    /// When `self` is initialized with [`Self::from_node(node)`](Self::from_node()), `get_node()`
    /// before the first call of `next()` returns `Some(node)`. Similarly, when initialized with
    /// [`Self::none()`], it returns `None`.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Node, NodeIter};
    /// # fn test(node: &Node) {
    /// let mut it = NodeIter::from_node(node);
    /// assert_eq!(it.get_node(), Some(node));
    ///
    /// let next = it.next();
    /// assert_eq!(next, Some(node));
    ///
    /// let curr = it.get_node();
    /// let next = it.next();
    /// assert_eq!(curr, next);
    /// assert_ne!(it.get_node(), next);
    /// # }
    /// ```
    #[inline]
    pub fn get_node(&self) -> Option<&'a Node> {
        self.node
    }

    /// Initializes with `None`. The returned iterator works like
    /// [`Empty`](std::iter::Empty).
    ///
    /// ```no_run
    /// # use mecab_wrapper::NodeIter;
    /// # fn test() {
    /// let mut it = NodeIter::none();
    /// assert_eq!(it.get_node(), None);
    /// assert_eq!(it.next(), None);
    /// # }
    /// ```
    #[inline]
    pub fn none() -> Self {
        Self { node: None }
    }

    /// Initializes with the given node.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Node, NodeIter};
    /// # fn test(node: &Node) {
    /// let mut it = NodeIter::from_node(node);
    /// assert_eq!(it.get_node(), Some(node));
    /// assert_eq!(it.next(), Some(node));
    /// assert_eq!(it.next(), node.next());
    /// # }
    /// ```
    #[inline]
    pub fn from_node(node: &'a Node) -> Self {
        Self { node: Some(node) }
    }

    /// This is identical with [`Self::from_node()`] or [`Self::none()`] depending on the
    /// `node` is `Some` or not.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Node, NodeIter};
    /// # fn test(node: &Node) {
    /// let mut it = NodeIter::from_node_option(Some(node));
    /// assert_eq!(it.next(), Some(node));
    ///
    /// let mut it = NodeIter::from_node_option(None);
    /// assert_eq!(it.next(), None);
    /// # }
    /// ```
    #[inline]
    pub fn from_node_option(node: Option<&'a Node>) -> Self {
        Self { node }
    }

    /// Initializes with the [`Lattice::bos_node()`]. This is the same as
    /// `NodeIter::from_node_option(lattice.bos_node())`.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Lattice, NodeIter};
    /// # fn test(lattice: &Lattice<'_>) {
    /// let mut it = NodeIter::from_bos(lattice);
    /// assert_eq!(it.get_node(), lattice.bos_node());
    /// assert_eq!(it.next(), lattice.bos_node());
    /// # }
    /// ```
    pub fn from_bos(lattice: &'a Lattice) -> Self {
        let node = lattice.bos_node();
        Self { node }
    }
}

/// Iterates nodes backward.
///
/// The iterations below are equivalent:
///
/// ```no_run
/// # use mecab_wrapper::Lattice;
/// # use mecab_wrapper::Node;
/// # use mecab_wrapper::NodeRevIter;
/// # fn do_something(node: &Node) {}
/// # fn test_node_iter(lattice: Lattice<'_>) {
/// // Iteration A
/// for node in lattice.iter_nodes_rev() {
///     do_something(node);
/// }
///
/// // Iteration B
/// for node in NodeRevIter::from_eos(&lattice) {
///     do_something(node);
/// }
///
/// // Iteration C
/// for node in NodeRevIter::from_node_option(lattice.eos_node()) {
///     do_something(node);
/// }
///
/// // Iteration D
/// if let Some(eos) = lattice.eos_node() {
///     for node in NodeRevIter::from_node(eos) {
///         do_something(node);
///     }
/// }
///
/// // Iteration E
/// if let Some(eos) = lattice.eos_node() {
///     let mut node = eos;
///     do_something(node);
///     while let Some(prev) = node.prev() {
///         node = prev;
///         do_something(node);
///     }
/// }
/// # }
pub struct NodeRevIter<'a> {
    node: Option<&'a Node>,
}

impl<'a> Iterator for NodeRevIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = node.prev();
        Some(node)
    }
}

impl<'a> NodeRevIter<'a> {
    /// Returns the current node (= the node which the next [`Iterator::next()`] will return).
    ///
    /// When `self` is initialized with [`Self::from_node(node)`](Self::from_node()), `get_node()`
    /// before the first call of `next()` returns `Some(node)`. Similarly, when initialized with
    /// [`Self::none()`], it returns `None`.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Node, NodeRevIter};
    /// # fn test(node: &Node) {
    /// let mut it = NodeRevIter::from_node(node);
    /// assert_eq!(it.get_node(), Some(node));
    ///
    /// let next = it.next();
    /// assert_eq!(next, Some(node));
    ///
    /// let curr = it.get_node();
    /// let next = it.next();
    /// assert_eq!(curr, next);
    /// assert_ne!(it.get_node(), next);
    /// # }
    /// ```
    #[inline]
    pub fn get_node(&self) -> Option<&'a Node> {
        self.node
    }

    /// Initializes with `None`. The returned iterator works like
    /// [`Empty`](std::iter::Empty).
    ///
    /// ```no_run
    /// # use mecab_wrapper::NodeRevIter;
    /// # fn test() {
    /// let mut it = NodeRevIter::none();
    /// assert_eq!(it.get_node(), None);
    /// assert_eq!(it.next(), None);
    /// # }
    /// ```
    #[inline]
    pub fn none() -> Self {
        Self { node: None }
    }

    /// Initializes with the given node.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Node, NodeRevIter};
    /// # fn test(node: &Node) {
    /// let mut it = NodeRevIter::from_node(node);
    /// assert_eq!(it.get_node(), Some(node));
    /// assert_eq!(it.next(), Some(node));
    /// assert_eq!(it.next(), node.prev());
    /// # }
    /// ```
    #[inline]
    pub fn from_node(node: &'a Node) -> Self {
        Self { node: Some(node) }
    }

    /// This is identical with [`Self::from_node()`] or [`Self::none()`] depending on the
    /// `node` is `Some` or not.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Node, NodeRevIter};
    /// # fn test(node: &Node) {
    /// let mut it = NodeRevIter::from_node_option(Some(node));
    /// assert_eq!(it.next(), Some(node));
    ///
    /// let mut it = NodeRevIter::from_node_option(None);
    /// assert_eq!(it.next(), None);
    /// # }
    /// ```
    #[inline]
    pub fn from_node_option(node: Option<&'a Node>) -> Self {
        Self { node }
    }

    /// Initializes with the [`Lattice::eos_node()`]. This is the same as
    /// `NodeRevIter::from_node_option(lattice.eos_node())`.
    ///
    /// ```no_run
    /// # use mecab_wrapper::{Lattice, NodeRevIter};
    /// # fn test(lattice: &Lattice<'_>) {
    /// let mut it = NodeRevIter::from_eos(lattice);
    /// assert_eq!(it.get_node(), lattice.eos_node());
    /// assert_eq!(it.next(), lattice.eos_node());
    /// # }
    /// ```
    pub fn from_eos(lattice: &'a Lattice) -> Self {
        let node = lattice.eos_node();
        Self { node }
    }
}
