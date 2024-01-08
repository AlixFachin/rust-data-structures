use std::collections::VecDeque;
use std::fmt::Display;
use std::fmt::Result;

pub struct BinaryTree<T: PartialOrd+Display> {
    val: Option<T>,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T: PartialOrd+Display> BinaryTree<T> {

    fn new_leaf(new_val: T) -> BinaryTree<T> {
        return BinaryTree { val: Some(new_val), left: None, right: None }
    }

    pub fn new() -> BinaryTree<T> {
        return BinaryTree {
            val: None,
            left: None,
            right: None,
        }
    }

    pub fn append(&mut self, new_val: T) {

        if let None = self.val {
            self.val = Some(new_val);
        } else {
            if new_val < *self.val.as_ref().unwrap() {
                match self.left {
                    None => self.left = Some(Box::new(BinaryTree::new_leaf(new_val))),
                    Some(ref mut child) => child.append(new_val),
                }   
            } else {
                match self.right {
                    None => self.right = Some(Box::new(BinaryTree::new_leaf(new_val))),
                    Some(ref mut child) => child.append(new_val),
                }
            }
        }
    }

    pub fn contains(&self, val: T) -> bool {
        if let Some(ref root_val) = self.val {
            if val == *root_val {
                return true;
            } else  if val < *root_val {
                if let Some(ref left_child) = self.left {
                    return left_child.contains(val);
                } else {
                    return false;
                }
            } else {
                if let Some(ref right_child) = self.right {
                    return right_child.contains(val);
                } else {
                    return false;
                }
            }
        } else {
            return false;
        }
    }
    /**
     * Returns a balanced tree
     */
    pub fn rebalance(&self) -> BinaryTree<&T> {

        let mut contents: Vec<&T> = vec![];
        
        enum NodeOrTree<'a, T: PartialOrd + Display> {
            Node(&'a T),
            Subtree(&'a BinaryTree<T>),
        }
        let mut process_stack: VecDeque<NodeOrTree<T>> = VecDeque::new();
        
        process_stack.push_back(NodeOrTree::Subtree(&self));
        while process_stack.len() != 0 {
            let cur_node = process_stack.pop_front().unwrap();
            match cur_node {
                NodeOrTree::Node(root_val) => {
                    contents.push(root_val);
                },
                NodeOrTree::Subtree(current_tree) => {

                    if let Some(ref left_tree) = current_tree.left {
                        process_stack.push_back(NodeOrTree::Subtree(left_tree));
                    }
                    if let Some(ref tree_root_val) = current_tree.val {
                        process_stack.push_back(NodeOrTree::Node(tree_root_val));
                    }
                    if let Some(ref right_tree) = current_tree.right {
                        process_stack.push_back(NodeOrTree::Subtree(right_tree));
                    }
                }
            }
        }

        // Now that we have the contents in right order, we can create a new tree
        let mut balanced_tree :BinaryTree<&T> = BinaryTree::new();
        for item in contents {
            balanced_tree.append(item);
        }
        return balanced_tree;
    }  

}

impl<T: PartialOrd+Display> Display for BinaryTree<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        
        let result;
        match self.val {
            None => result = String::from("[]"),
            Some(ref root_val) => {                
                let result_left;
                let result_right;
                if let Some(ref left_tree) = self.left {
                    result_left = left_tree.to_string();
                } else {
                    result_left = String::from("");
                }
                if let Some(ref right_tree) = self.right {
                    result_right = right_tree.to_string();
                } else {
                    result_right = String::from("");
                }

                result = format!("[{},{},{}]",result_left, root_val, result_right);
                
            }
        }
        return writeln!(f,"{}",&result);
   }

}
