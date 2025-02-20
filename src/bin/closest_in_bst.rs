#[derive(Debug, Clone)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

#[derive(Debug)]
struct BinaryTree {
    root: Option<Box<Node>>,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree { root: None }
    }

    fn insert(&mut self, value: i32) {
        self.root = Self::insert_node(self.root.take(), value);
    }

    fn insert_node(node: Option<Box<Node>>, value: i32) -> Option<Box<Node>> {
        match node {
            Some(mut n) => {
                if value < n.value {
                    n.left = Self::insert_node(n.left, value);
                } else {
                    n.right = Self::insert_node(n.right, value);
                }
                Some(n)
            }
            None => Some(Box::new(Node {
                value,
                left: None,
                right: None,
            })),
        }
    }

    fn in_order_traversal(&self) {
        Self::in_order_helper(&self.root);
    }

    fn in_order_helper(node: &Option<Box<Node>>) {
        if let Some(n) = node {
            Self::in_order_helper(&n.left);
            println!("{}", n.value);
            Self::in_order_helper(&n.right);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    // Insert values into the binary tree
    tree.insert(8);
    tree.insert(3);
    tree.insert(10);
    tree.insert(1);
    tree.insert(6);
    tree.insert(14);
    tree.insert(4);
    tree.insert(7);
    tree.insert(13);

    // Perform in-order traversal
    println!("In-order traversal of the binary tree:");
    tree.in_order_traversal();

    println!(
        "[Iter] - Closest Number is: {}",
        find_closest_value(&tree.root, 2, i32::MAX)
    );
    println!(
        "[Recursive] - Closest Number is: {}",
        find_closest_value_recursive(&tree.root, 2, i32::MAX)
    );
    // debug_assert_eq!(result, 12);
}

fn find_closest_value_recursive(
    tree_node: &Option<Box<Node>>,
    target: i32,
    mut closest: i32,
) -> i32 {
    if let Some(node) = tree_node.as_ref() {
        let node_value = tree_node.as_ref().unwrap().value;

        if (target - closest).abs() > (target - node_value).abs() {
            closest = node_value;
        }

        if target < node_value {
            closest = find_closest_value(&node.left, target, closest)
        } else if target > node_value {
            closest = find_closest_value(&node.right, target, closest);
        }
    }
    closest
}

fn find_closest_value(tree_node: &Option<Box<Node>>, target: i32, mut closest: i32) -> i32 {
    let mut current_node = tree_node.as_ref();

    while let Some(node) = current_node {
        let node_value = node.value;

        if (target - closest).abs() > (target - node_value).abs() {
            closest = node_value;
        }

        if target < node_value {
            current_node = node.left.as_ref();
        } else if target > node_value {
            // current_node = Some(current_node.as_ref().unwrap().right.as_ref().unwrap());
            current_node = node.right.as_ref();
        } else {
            break;
        }
    }

    closest
}
