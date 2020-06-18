use crate::ui::WidgetId;

pub struct Graph<T> {
    nodes: Vec<Node<T>>,
}

pub struct Node<T> {
    parent: Option<NodeId>,
    previous: Option<NodeId>,
    next: Option<NodeId>,
    child_first: Option<NodeId>,
    child_last: Option<NodeId>,

    pub data: T,
}

pub struct NodeId {
    index: usize,
}

impl Graph<WidgetId> {
    pub fn new_node(&mut self, data: T) -> NodeId {
        let next_index = self.nodes.len();

        self.nodes.push(Node {
            parent: None,
            previous: None,
            next: None,
            child_first: None,
            child_last: None,
            data,
        });
        NodeId { index: next_index }
    }
}