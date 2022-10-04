use std::collections::{BTreeSet, HashMap};

use std::sync;

use crate::window::Window;

pub struct Bsp {
    tree: Node,
    nodes: HashMap<i64, Node>,
    curr_id: i64,
}

impl Bsp {
    pub fn new() -> Self {
        Self {
            tree: Node::Empty { id: 0 },
            nodes: HashMap::new(),
            curr_id: 0,
        }
    }
    pub fn insert(&mut self, window: sync::Arc<Window>) {}

    pub fn get_id(&mut self) -> i64 {
        self.curr_id += 1;
        self.curr_id
    }
}

enum Node {
    Parent {
        id: i64,
        left: Box<Node>,
        right: Box<Node>,
    },
    Window {
        id: i64,
        window: sync::Arc<Window>,
    },
    Empty {
        id: i64,
    },
}

trait AddWindow {
    fn add_window();
}

impl Node {
    pub fn insert(&self, bsp: &mut Bsp, window: sync::Arc<Window>) -> Option<Node> {
        match &self {
            Node::Parent { id, left, right } => None,
            Node::Window {
                id,
                window: node_window,
            } => Some(Node::Parent {
                id: *id,
                left: Box::from(Node::Window {
                    id: bsp.get_id(),
                    window: node_window.clone(),
                }),
                right: Box::from(Node::Window {
                    id: bsp.get_id(),
                    window: window.clone(),
                }),
            }),
            Node::Empty { id } => Some(Node::Window {
                id: *id,
                window: window,
            }),
        }
    }
    pub fn pop(&self, bsp: &mut Bsp, window: sync::Arc<Window>) -> Option<Node> {
        match &self {
            Node::Parent { id, left, right } => None,
            Node::Window { id, window: _ } => Some(Node::Empty { id: *id }),
            Node::Empty { id } => None,
        }
    }
}
