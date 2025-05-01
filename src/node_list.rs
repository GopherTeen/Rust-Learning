struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

pub struct NodeList {
    head: Option<Box<Node>>,
}

impl NodeList {
    pub fn new() -> NodeList {
        // 伪头节点
        NodeList {
            head: Some(Box::new(Node {
                val: (-1),
                next: (None),
            })),
        }
    }

    pub fn insert(&mut self, val: i32) {
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if node.next.is_none() || node.next.as_ref().unwrap().val >= val {
                let new_node = Box::new(Node {
                    val,
                    next: node.next.take(),
                });
                node.next = Some(new_node);
                return;
            }
            cur = &mut node.next;
        }
    }

    pub fn remove(&mut self, val: i32) {
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if let Some(node_next) = &mut node.next {
                if node_next.val == val {
                    node.next = node_next.next.take();
                    return;
                }
            }
            cur = &mut node.next;
        }
    }

    pub fn search(&mut self, val: i32) -> bool {
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if node.val == val {
                return true;
            }
            cur = &mut node.next;
        }
        return false;
    }

    pub fn print(&self) {
        let mut cur = &self.head;
        while let Some(node) = cur {
            println!("{}", node.val);
            cur = &node.next;
        }
    }
}
