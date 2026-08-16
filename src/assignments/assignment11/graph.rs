//! A small graph library.
//!
//! A node has a i32 value and (directed) edges to other nodes. A node does not have multiple edges
//! to the same node. Nodes are not associated with a particular domain, and users can freely
//! create nodes however they like. However, after a node is created, it can be added to a
//! `SubGraph`, which form a subgraph of the graph of all nodes. A node can be added to multiple
//! subgraphs. `SubGraph` has a method to check if the it has a cycle.
//!
//! The goal of this assignment is to learn how to deal with inherently shared mutable data in
//! Rust. Design the types and fill in the `todo!()`s in methods. There are several possible
//! approaches to this problem and you may import anything from the std library accordingly.
//!
//! Refer `graph_grade.rs` for test cases.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(PartialEq, Eq, Debug)]
enum VisitStatus {
    Unvisited,
    Visiting,
    Visited,
}

/// Handle to a graph node.
///
/// `NodeHandle` should implement `Clone`, which clones the handle without cloning the underlying
/// node. That is, there can be multiple handles to the same node.
/// The user can access the node through a handle if it does not violate Rust's aliasing rules.
///
/// You can freely add fields to this struct.
#[derive(Debug, Clone)]
pub struct NodeHandle(Rc<RefCell<Node>>);

#[derive(Debug)]
struct Node {
    value: i32,
    edges: Vec<NodeHandle>,
}

/// Error type for graph operations.
#[derive(Debug)]
pub struct GraphError;

/// Subgraph
///
/// You can freely add fields to this struct.
#[derive(Debug)]
pub struct SubGraph {
    nodes: HashSet<NodeHandle>,
}

impl NodeHandle {
    /// Creates a node and returns the handle to it.
    pub fn new(value: i32) -> Self {
        Self(Rc::new(RefCell::new(Node {
            value,
            edges: vec![],
        })))
    }

    /// Adds an edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully added.
    /// Returns `Ok(false)` if an edge to `to` already exits.
    pub fn add_edge(&self, to: NodeHandle) -> Result<bool, GraphError> {
        let mut node = self.0.try_borrow_mut().map_err(|_| GraphError)?;
        if node.edges.contains(&to) {
            return Ok(false);
        }

        node.edges.push(to);
        Ok(true)
    }

    /// Removes the edge to `to`.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`. Returns `Ok(true)` if the edge is successfully removed.
    /// Returns `Ok(false)` if an edge to `to` does not exist.
    pub fn remove_edge(&self, to: &NodeHandle) -> Result<bool, GraphError> {
        let mut node = self.0.try_borrow_mut().map_err(|_| GraphError)?;
        if let Some(index) = node.edges.iter().position(|e| e == to) {
            drop(node.edges.remove(index));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Removes all edges.
    /// If the modification cannot be done, e.g. because of aliasing issues, returns
    /// `Err(GraphError)`.
    pub fn clear_edges(&self) -> Result<(), GraphError> {
        let mut node = self.0.try_borrow_mut().map_err(|_| GraphError)?;
        node.edges.clear();
        Ok(())
    }
}

impl PartialEq for NodeHandle {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for NodeHandle {}

impl Hash for NodeHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Rc::as_ptr(&self.0).hash(state);
    }
}

impl Default for SubGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl SubGraph {
    /// Creates a new subgraph.
    pub fn new() -> Self {
        SubGraph {
            nodes: HashSet::new(),
        }
    }

    /// Adds a node to the subgraph. Returns true iff the node is newly added.
    pub fn add_node(&mut self, node: NodeHandle) -> bool {
        self.nodes.insert(node)
    }

    /// Removes a node from the subgraph. Returns true iff the node is successfully removed.
    pub fn remove_node(&mut self, node: &NodeHandle) -> bool {
        self.nodes.remove(node)
    }

    /// Returns true iff the subgraph contains a cycle. Nodes that do not belong to this subgraph
    /// are ignored. See <https://en.wikipedia.org/wiki/Cycle_(graph_theory)> for an algorithm.
    pub fn detect_cycle(&self) -> bool {
        let mut status = HashMap::<NodeHandle, VisitStatus>::new();

        for node in &self.nodes {
            if status.get(node) != Some(&VisitStatus::Visited) {
                if Self::dfs(node, &self.nodes, &mut status) {
                    return true;
                }
            }
        }
        false
    }

    fn dfs(
        current: &NodeHandle,
        subgraph_nodes: &HashSet<NodeHandle>,
        status: &mut HashMap<NodeHandle, VisitStatus>,
    ) -> bool {
        let _ = status.insert(current.clone(), VisitStatus::Visiting);

        let node_ref = current.0.borrow();

        for neighbor in &node_ref.edges {
            if !subgraph_nodes.contains(neighbor) {
                continue;
            }

            match status.get(neighbor) {
                Some(VisitStatus::Visiting) => {
                    return true;
                }
                Some(VisitStatus::Visited) => {
                    continue;
                }
                Some(VisitStatus::Unvisited) | None => {
                    if Self::dfs(neighbor, subgraph_nodes, status) {
                        return true;
                    }
                }
            }
        }
        let _ = status.insert(current.clone(), VisitStatus::Visited);
        false
    }
}
