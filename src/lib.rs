#[derive(Debug)]
pub struct Node<'a> {
    pub name: &'a str,
    pub index: usize,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl<'a> Node<'a> {
    pub fn new(name: &'a str, index: usize, parent: Option<usize>) -> Node {
        Node {
            name,
            index,
            parent,
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Tree<'a> {
    arena: Vec<Node<'a>>,
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree::default()
    }

    pub fn get(&self, node_id: usize) -> &Node<'a> {
        &self.arena[node_id]
    }

    pub fn add(&mut self, name: &'a str, parent: Option<usize>) -> usize {
        let new_element_index = self.arena.len();
        let new_node = Node::new(name, new_element_index, parent);

        self.arena.push(new_node);
        // register child to parent node
        if let Some(parent_id) = parent {
            self.arena[parent_id].children.push(new_element_index);
        }

        new_element_index
    }

    pub fn path(&self, node_index: usize) -> String {
        let mut node = &self.arena[node_index];
        let mut path = node.name.to_string();

        while let Some(parent_index) = node.parent {
            node = &self.arena[parent_index];
            path.push('_');
            path += &node.name;
        }
        path
    }

    pub fn traverse(&self, node_index: usize, mut path: String) -> String {
        let node = &self.arena[node_index];
        path += &node.name;
        if let Some(parent_index) = node.parent {
            path.push('_');
            self.traverse(parent_index, path)
        } else {
            path
        }
    }

    pub fn r_path(&self, node_index: usize) -> String {
        self.traverse(node_index, String::new())
    }
}

#[test]
fn test_paths() {
    let mut tree = Tree::new();
    let root = tree.add("Root", None);
    let child_1 = tree.add("Child1", Some(root));
    let sub_child_1 = tree.add("SubChild1", Some(child_1));
    let _ = tree.add("SubChild2", Some(child_1));
    let _ = tree.add("Child2", Some(root));
    let _ = tree.add("Child3", Some(root));

    let node_path = tree.path(sub_child_1);
    assert_eq!("SubChild1_Child1_Root", node_path);

    let r_node_path = tree.r_path(sub_child_1);
    assert_eq!("SubChild1_Child1_Root", r_node_path);
}
