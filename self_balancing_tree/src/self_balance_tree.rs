use crate::node::Node;

pub struct Tree<T: std::cmp::Ord + std::marker::Copy> {
    pub head_node: Option<Node<T>>,
}

impl<T: std::cmp::Ord + std::marker::Copy> Tree<T> {
    pub fn new(values: &mut [T]) -> Tree<T> {
        Tree {
            head_node: Node::new(values),
        }
    }

    pub fn new_for_sorted(values: &[T]) -> Tree<T> {
        Tree {
            head_node: Node::new_for_sorted(values),
        }
    }

    pub fn pop(&mut self, value: T) {
        if self.exists(value) {
            let storage: Vec<T> = if let Some(head) = &self.head_node {
                Tree::rip_tree(head, Some(value))
            } else {
                Vec::new()
            };
            self.head_node = Node::new_for_sorted(&storage);
        }
    }

    pub fn insert(&mut self, value: T) {
        if !self.exists(value) {
            let mut storage: Vec<T> = if let Some(head) = &self.head_node {
                Tree::rip_tree(head, None)
            } else {
                Vec::new()
            };

            if storage.len() == 0 {
                storage.push(value);
            } else if value > *storage.get(storage.len() - 1).unwrap() {
                storage.push(value);
            } else if value < *storage.get(0).unwrap() {
                storage.insert(0, value);
            } else {
                for i in 0..storage.len() {
                    if value >= *storage.get(i).unwrap() {
                        storage.insert(i, value);
                        break;
                    }
                }
            }

            self.head_node = Node::new_for_sorted(&storage);
        }
    }

    pub fn exists(&self, value: T) -> bool {
        if self.head_node.is_none() {
            return false;
        }
        self.secret_exists(value, self.head_node.as_ref().unwrap())
    }

    fn secret_exists(&self, value: T, node: &Node<T>) -> bool {
        if value == node.value {
            true
        } else if value < node.value {
            if let Some(child) = &node.lower_child {
                return self.secret_exists(value, child);
            }
            false
        } else {
            if let Some(child) = &node.higher_child {
                return self.secret_exists(value, child);
            }
            false
        }
    }

    fn rip_tree(node: &Node<T>, value: Option<T>) -> Vec<T> {
        let mut return_vec: Vec<T>;
        if let Some(left_child) = &node.lower_child {
            return_vec = Tree::rip_tree(left_child, value);
        } else {
            return_vec = Vec::new();
        }

        if value.is_none() || value.unwrap() != node.value {
            return_vec.push(node.value);
        }

        if let Some(right_child) = &node.higher_child {
            return_vec.extend(Tree::rip_tree(right_child, value));
        }

        return_vec
    }

    /*
    It's all a mess. I attempted to make a more neat(read faster) solution but i couldn't ;(
    I'll leave my mess here so that you can see i attempted a solution thats nice.
    
    OK insert actually work for the faster variant,
    its not by a lot
    but the fastest speed definitaly increased a lot.
    */

    pub fn insert_experimental(&mut self, value: T) {
        if !self.exists(value) {
            //here is problem/**/*/*/*/ */ */ */ */
            if self.head_node.is_some() {
                Tree::secret_insert(value, self.head_node.as_mut().unwrap())
            } else {
                self.head_node = Some(Node::new_end_node(value));
            }
        }
    }

    fn secret_insert(value: T, node: &mut Node<T>) {
        // See which side needs a new node. and then send it down that path.
        // if it's the wrong element for that. swap it with the current node and sent it swapped value down.

        // Compare the value with the value of out current Node.
        // that way we can know which way we want to send our node.
        // then we need to compare amount of subnodes on each side.

        if node.left_sub_nodes <= node.right_sub_nodes {
            //we need to send a node to the left side.
            if value < node.value {
                //Yeah it works out since the value is less then current node.
                if let Some(child) = &mut node.lower_child {
                    Tree::secret_insert(value, child);
                    node.left_sub_nodes += 1;
                } else {
                    //Perfect! This node fits right here.
                    node.set_lower_child(value);
                    node.left_sub_nodes += 1;
                }
            } else {
                /*
                Our value needs to go to the right, but a new node needs to be created on the left side.
                We know our highest value is in current node so we send it down the left side
                and place our current value in this node and then sort the rightside (including this node.).
                */
                let switch_value = node.value;

                if let Some(right_child) = &mut node.higher_child {
                    //yoink up the lowest value on the right side
                    let(t, v)= Tree::fix_right_side(value, right_child);
                    node.value = v;
                    node.higher_child = Some(Box::new(t));
                } else {
                    node.value = value
                }

                if let Some(child) = &mut node.lower_child {
                    Tree::secret_insert(switch_value, child);
                    node.left_sub_nodes += 1;
                } else {
                    node.set_lower_child(switch_value);
                    node.left_sub_nodes += 1;
                }
            }
        } else {
            // Yikes. we need more nodes on the right.
            if value > node.value {
                //yeah works out.
                if let Some(child) = &mut node.higher_child {
                    Tree::secret_insert(value, child);
                    node.right_sub_nodes += 1;
                } else {
                    //Perfect! This node fits right here.
                    node.set_higher_child(value);
                    node.right_sub_nodes += 1;
                }
            } else {
                let switch_value = node.value;

                if let Some(left_child) = &node.lower_child {
                    let(t, v) = Tree::fix_left_side(value, left_child);
                    node.value = v;
                    node.lower_child = Some(Box::new(t));
                } else {
                    node.value = value;
                }

                if let Some(child) = &mut node.higher_child {
                    Tree::secret_insert(switch_value, child);
                    node.right_sub_nodes += 1;
                } else {
                    node.set_higher_child(switch_value);
                    node.right_sub_nodes += 1;
                }
            }
        }
    }

    fn fix_left_side (value: T, node: &Node<T>) -> (Node<T>, T) {
        let mut storage = Tree::rip_tree(node, None);
        let highest = storage.pop().unwrap();

        if storage.len() == 0 {
            storage.push(value);
        } else if value < *storage.get(0).unwrap() {
            storage.insert(0, value);
        } else if value >= *storage.get(storage.len() - 1).unwrap() {
            storage.push(value);
        }
        for i in 0..storage.len() {
            if value >= *storage.get(i).unwrap() {
                storage.insert(i, value);
                break;
            }
        }
        (Node::new_for_sorted(&storage).unwrap(), highest)
    }

    fn fix_right_side (value: T, node: &Node<T>) -> (Node<T>, T) {
        let mut storage = Tree::rip_tree(node, None);
        let lowest = storage.remove(0);

        if storage.len() == 0 {
            storage.push(value);
        } else if value < *storage.get(0).unwrap() {
            storage.insert(0, value);
        } else if value >= *storage.get(storage.len() - 1).unwrap() {
            storage.push(value);
        }
        for i in 0..storage.len() {
            if value >= *storage.get(i).unwrap() {
                storage.insert(i, value);
                break;
            }
        }
        (Node::new_for_sorted(&storage).unwrap(), lowest)
    }

    /*fn sort_sub_tree(node: &Node<T>) -> Node<T> {
        let mut storage: Vec<T>;
        /*
        We can assume the tree is sorted except root node
         */
        if let Some(left_child) = &node.lower_child {
            storage = Tree::rip_tree(left_child, None);
        } else {
            storage = Vec::new()
        }
        if let Some(right_child) = &node.higher_child {
            storage.extend(Tree::rip_tree(right_child, None));
        }
        if storage.len() == 0 {
            storage.push(node.value);
        } else if node.value < *storage.get(0).unwrap() {
            storage.insert(0, node.value);
        } else if node.value >= *storage.get(storage.len() - 1).unwrap() {
            storage.push(node.value);
        }
        for i in 0..storage.len() {
            if node.value >= *storage.get(i).unwrap() {
                storage.insert(i, node.value);
                break;
            }
        }
        Node::new_for_sorted(&storage).unwrap()
    }*/

    /*fn left_side_swap_values(&self, node: &mut Node<T>) {
        if let Some(child) = node.lower_child {
            if node.value < child.value {
                // we need to swap the values,
                let _temp = node.value;
                node.value = child.value;
                child.value = _temp;
            }
        } else {
            //we reached the end
        }
    }*/
}
