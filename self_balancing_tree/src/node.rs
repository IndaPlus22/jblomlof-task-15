#[derive(Clone)]
pub struct Node<T: std::cmp::Ord + std::marker::Copy> {
    pub value: T,
    pub lower_child: Option<Box<Node<T>>>,
    pub higher_child: Option<Box<Node<T>>>,
    pub left_sub_nodes: usize,
    pub right_sub_nodes: usize,
}

impl<T: std::cmp::Ord + std::marker::Copy> Node<T> {
    pub fn new_end_node(value: T) -> Node<T> {
        Node {
            value: value,
            lower_child: None,
            higher_child: None,
            left_sub_nodes: 0,
            right_sub_nodes: 0,
        }
    }

   /* pub fn set_lower_child(&mut self, value: T) {
        self.lower_child = Some(Box::new(Node::new_end_node(value)));
    }

    pub fn set_higher_child(&mut self, value: T) {
        self.higher_child = Some(Box::new(Node::new_end_node(value)));

    }*/
    
    /**
     * the function that init. the whole tree.
     * I hope its sorted otherwise yo can go and sort it
     * Or use new()
     */
    pub fn new_for_sorted(values: &[T]) -> Option<Node<T>> {
        if values.len() == 0 {
            return None;
        }
        
        //removing dupes
        let mut x = values.to_vec();
        x.dedup();

        Some(Node::init_node_and_children(x.as_slice()).0)
    }

    /**
     * Just pass an &mut array and it'll be fine
     * If you know the array is sorted you can use `new_for_sorted`
     * returns the root node for a new (sorted) tree.
     */
    pub fn new(values: &mut [T]) -> Option<Node<T>> {
        if values.len() == 0 {
            return None;
        }
        values.sort_unstable();
        
        // remove dupes.
        let mut x = values.to_vec();
        x.dedup(); 

        Some(Node::init_node_and_children(x.as_slice()).0)
    }

    // this is the function that init everything,
    //but cannot be used by user, since it also returns height for recursive init. 
    //returns amount of subnodes)
    fn init_node_and_children(values: &[T]) -> (Node<T>, usize) {
        if values.len() == 0 {
            panic!("DONT YOU DARE PASS 0");
        }
        if values.len() == 1 {
            (Node::new_end_node(values[0]),0 + 1)
        }
        else if values.len() == 2 {
            (Node {
                value: values[values.len()/2], // will be values[1] but for consinteny have it in terms of functions

                //we only have values greater than this node's value.
                lower_child: Some(Box::new(Node::init_node_and_children(&values[0..values.len()/2]).0)), // really is just [0..1]
                higher_child: None,
                
                // we got one on the left,
                //but nothing on the right
                left_sub_nodes: 1,
                right_sub_nodes: 0,
            }, 1 + 0 + 1)
        }
        else {
             /*
                BIGGEST PART OF THIS FUNCTION 
                Pretty simple. Take the middle value (for odd length (length as float)/2.0 + 0.5.... with other words the rightmost)
                Store that value in this node.
                Everything on the left hand side is sent to the lower_child node
                and on the right to the higher ...
                */
            let (lower, sub_l) = Node::init_node_and_children(&values[0..values.len()/2]);
            let (higher, sub_r) = Node::init_node_and_children(&values[(values.len()/2 + 1)..values.len()]);
            
            (Node {
                value: values[values.len()/2],
                lower_child: Some(Box::new(lower)), 
                higher_child: Some(Box::new(higher)),
                left_sub_nodes: sub_l,
                right_sub_nodes: sub_r,
            }, sub_l + sub_r + 1)
        }
    }
}