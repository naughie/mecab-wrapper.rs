use crate::ffi::{Lattice, Node};

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
    #[inline]
    pub fn get_node(&self) -> Option<&'a Node> {
        self.node
    }

    #[inline]
    pub fn none() -> Self {
        Self { node: None }
    }

    #[inline]
    pub fn from_node(node: &'a Node) -> Self {
        Self { node: Some(node) }
    }

    #[inline]
    pub fn from_node_option(node: Option<&'a Node>) -> Self {
        Self { node }
    }

    pub fn from_bos(lattice: &'a Lattice) -> Self {
        let node = lattice.bos_node();
        Self { node }
    }
}
