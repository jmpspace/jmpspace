
use std::iter::{FromIterator, IteratorExt};
use std::vec::Vec;

enum TagTree<Leaf, Node, Edge> {
  Leaf(Leaf),
  Node(Node, Vec<(Edge, Box<TagTree<Leaf, Node, Edge>>)>)
}

fn foldTagTree<Leaf, Node, Edge, T, FL, FN, FE> (
  reduceLeaf: FL,
  reduceNode: FN,
  reduceEdge: FE,
  tree: TagTree<Leaf, Node, Edge>) -> T 
  where FL: Fn(Leaf) -> T, FN: Fn(Node, Vec<T>) -> T, FE: Fn(Edge, T) -> T 
{
  let y = |tree| { foldTagTree(reduceLeaf, reduceNode, reduceEdge, tree) };
  let reduceChild = |(edge, child)| { reduceEdge(edge, y(child)) };
  match tree {
    TagTree::Leaf(leaf) => reduceLeaf(leaf),
    TagTree::Node(node, subs) => reduceNode(node, subs.into_iter().map(reduceChild).from_iter())
  }
}