use std::i32;

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

    fn node_depths(&self, depth: i32, node: &Option<Box<Node>>) -> i32 {
        match node {
            Some(n) => {
                let l = self.node_depths(depth + 1, &n.left);
                let r = self.node_depths(depth + 1, &n.right);

                if l > r {
                    l
                } else {
                    r
                }
            }
            None => return depth - 1,
        }
        // return depth + self.node_depths(depth, node
    }

    // fn in_order_traversal(&self) {
    //     Self::in_order_helper(&self.root);
    // }

    // fn in_order_helper(node: &Option<Box<Node>>) {
    //     if let Some(n) = node {
    //         Self::in_order_helper(&n.left);
    //         println!("{}", n.value);
    //         Self::in_order_helper(&n.right);
    //     }
    // }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(22); // 0
    tree.insert(10); // 1
    tree.insert(25); // 1
    tree.insert(19); // 2
    tree.insert(9); // 2
    tree.insert(4); // 3
    tree.insert(50); // 2
    tree.insert(1); // 4

    // tree.in_order_traversal();

    let mut branches_sums: Vec<i32> = Vec::new();

    branch_sum(&tree.root, 0, &mut branches_sums);

    println!("{:?}", branches_sums);

    println!("{:?}", tree.node_depths(0, &tree.root))
}

fn branch_sum(current_node: &Option<Box<Node>>, running_sum: i32, branches_sums: &mut Vec<i32>) {
    if let Some(node) = current_node.as_ref() {
        let new_running_sum = running_sum + node.value;

        if node.left.is_none() && node.right.is_none() {
            branches_sums.push(new_running_sum);
            return;
        }

        branch_sum(&node.left, new_running_sum, branches_sums);
        branch_sum(&node.right, new_running_sum, branches_sums);
    }
}
