type ConditionFunc = Box<dyn Fn(&Node) -> bool>;
type ActionFunc = Box<dyn Fn(&Node) -> NodeResult>;

enum NodeType {
    Root(Box<Node>),

    Sequence(Vec<Node>),
    Selector(Vec<Node>),

    Condition(ConditionFunc),
    Action(ActionFunc),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeResult {
    Running,
    Success,
    Failure
}

pub struct Node {
    node_type: NodeType,
}

impl Node {
    fn new(node_type: NodeType) -> Self {
        Self {
            node_type
        }
    }

    pub fn root(child: Node) -> Self {
        Self::new(NodeType::Root(Box::new(child)))
    }

    pub fn sequence(children: Vec<Node>) -> Self {
        Self::new(NodeType::Sequence(children))
    }

    pub fn selector(children: Vec<Node>) -> Self {
        Self::new(NodeType::Selector(children))
    }

    pub fn condition<F>(condition: F) -> Self
    where F: Fn(&Node) -> bool + 'static {
        Self::new(NodeType::Condition(Box::new(condition)))
    }

    pub fn action<F>(condition: F) -> Self
    where F: Fn(&Node) -> NodeResult + 'static {
        Self::new(NodeType::Action(Box::new(condition)))
    }

    pub fn tick(&self) -> NodeResult {
        match &self.node_type {
            NodeType::Root(node) => node.tick(),
            NodeType::Sequence(nodes) => {
                for node in nodes.iter() {
                    match node.tick() {
                        NodeResult::Running => return NodeResult::Running,
                        NodeResult::Failure => return NodeResult::Failure,
                        _ => (),
                    }
                }

                NodeResult::Success
            },
            NodeType::Selector(nodes) => {
                for node in nodes.iter() {
                    match node.tick() {
                        NodeResult::Running => return NodeResult::Running,
                        NodeResult::Success => return NodeResult::Success,
                        _ => (),
                    }
                }

                NodeResult::Failure
            },
            NodeType::Condition(condition) => {
                condition(self).then_some(NodeResult::Success).unwrap_or(NodeResult::Failure)
            },
            NodeType::Action(action) => {
                action(self)
            }
        }
    }
}

pub struct BehaviorTree {
    root: Node,
}

impl BehaviorTree {
    pub fn new(root: Node) -> Self {
        BehaviorTree {
            root,
        }
    }

    pub fn tick(&mut self) -> NodeResult {
        self.root.tick()
    }
}
