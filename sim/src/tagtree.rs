
use std::vec::Vec;

pub enum TagTree<Leaf, Node, Edge> {
    Leaf(Leaf),
    Node(Node, Vec<(Edge, Box<TagTree<Leaf, Node, Edge>>)>)
}

impl<Leaf, Node, Edge> TagTree<Leaf, Node, Edge> {

    pub fn fold_tag_tree<T, FL, FN> (
        &self,
        ref reduce_leaf: FL,
        ref reduce_node: FN) -> T
        where
        FL: Fn(&Leaf) -> T,
        FN: Fn(&Node, Vec<(&Edge, T)>) -> T
        {
            match self {
                &TagTree::Leaf(ref leaf) => reduce_leaf(leaf),
                &TagTree::Node(ref node, ref children) => {
                    let edge_subs = children.iter().map(|edge_child| {
                        match edge_child {
                            &(ref edge, box ref child) => {
                                let sub = child.fold_tag_tree(reduce_leaf, reduce_node);
                                (edge, sub)
                            }
                        }
                    }).collect();
                    reduce_node(node, edge_subs)
                }
            }
        }



    pub fn fold_tag_tree2<T, FL, FN, FE> (
        &self,
        ref reduce_leaf: FL,
        ref reduce_node: FN,
        ref reduce_edge: FE) -> T
        where 
        FL: Fn(&Leaf) -> T, 
        FN: Fn(&Node, Vec<T>) -> T, 
        FE: Fn(&Edge, T) -> T 
        {
            match self {
                &TagTree::Leaf(ref leaf) => reduce_leaf(leaf),
                &TagTree::Node(ref node, ref children) => {
                    let subs = children.iter().map(|edge_child| { 
                        match edge_child {
                            &(ref edge, box ref child) => {
                                let sub = child.fold_tag_tree2(reduce_leaf, reduce_node, reduce_edge);    
                                reduce_edge(edge, sub)
                            }
                        }
                    }).collect();
                    reduce_node(node, subs)
                }
            }
        }
}
