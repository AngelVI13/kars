#[derive(Debug)]
pub struct Node<'a> {
    name: &'a str,
    index: usize,
    parent: Option<usize>,
    children: Vec<usize>,
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

#[derive(Debug)]
pub struct Tree<'a> {
    arena: Vec<Node<'a>>,
    root: usize,
}

impl<'a> Default for Tree<'a> {
    fn default() -> Self {
        let root_node = Node::new("Root", 0, None);
        let arena: Vec<Node> = vec![root_node];
        Tree { arena, root: 0 }
    }
}

impl<'a> Tree<'a> {
    pub fn new() -> Tree<'a> {
        Tree::default()
    }

    pub fn add(&mut self, name: &'a str, parent: usize) -> usize {
        let new_element_index = self.arena.len();
        let new_node = Node::new(name, new_element_index, Some(parent));

        self.arena.push(new_node);
        // register child to parent node
        self.arena[parent].children.push(new_element_index);

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
    let mut tree: Tree = Tree::new();

    let child_1 = tree.add("Child1", tree.root);
    let sub_child_1 = tree.add("SubChild1", child_1);
    let _ = tree.add("SubChild2", child_1);
    let _ = tree.add("Child2", tree.root);
    let _ = tree.add("Child3", tree.root);

    let node_path = tree.path(tree.arena[sub_child_1].index);
    assert_eq!("SubChild1_Child1_Root", node_path);

    let r_node_path = tree.r_path(tree.arena[sub_child_1].index);
    assert_eq!("SubChild1_Child1_Root", r_node_path);
}
