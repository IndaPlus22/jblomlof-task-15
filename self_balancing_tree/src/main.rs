use crate::self_balance_tree::Tree;
mod self_balance_tree;
mod node;

/**
 * A self balancing tree for isize values.
 */

fn main() {

    println!("Hello, world!");
}

#[test]
fn empty_tree() {
    let mut tree: Tree<isize> = Tree::new(&mut []);
    assert!(tree.head_node.is_none());
    tree = self_balance_tree::Tree::new_for_sorted(&[]);
    assert!(tree.head_node.is_none());
}

#[test]
fn insert_tree() {
    let mut tree: Tree<isize> = Tree::new(&mut [1,0,2,4,3]);
    /*
    Tree should now be
        2
       / \
      1   4
     /   /
    0   3     
    */
    assert!(tree.head_node.clone().unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 3);

    tree.insert(-2);
    /*
    Tree should now be
        2
       / \
      0     4
     / \   /
   -2   1   3     
    */
    assert!(tree.head_node.clone().unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == -2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 3);

    tree.insert(7);
    /*
    Tree should now be
        2
       / \
      0     4
     / \   /  \
   -2   1  3  7   
    */

    assert!(tree.head_node.clone().unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == -2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().higher_child.unwrap().value == 7);

    tree.insert(9);
    /*
    Tree should now be
        3
       / \
      1     7
     / \   /  \
    0   2  4   9
   /
  -2 
    */

    assert!(tree.head_node.clone().unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().lower_child.unwrap().value == -2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 7);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().higher_child.unwrap().value == 9);
}

#[test]
fn pop_test() {
    let mut tree: Tree<isize> = Tree::new_for_sorted(&[-2, 0, 1, 2, 3, 4, 7, 9]);
        /*
    Tree should now be
        3
       / \
      1     7
     / \   /  \
    0   2  4   9
   /
  -2 
    */

    assert!(tree.head_node.clone().unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().lower_child.unwrap().value == -2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 7);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().higher_child.unwrap().value == 9);

    tree.pop(-2);
    /*
    Tree should now be
        3
       / \
      1     7
     / \   /  \
    0   2  4   9
    */
    assert!(tree.head_node.clone().unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 7);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().higher_child.unwrap().value == 9);
    
    tree.pop(3);
    /*
    Tree should now be
        4
       / \
      1     9
     / \   / 
    0   2  7   
    */
    assert!(tree.head_node.clone().unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 9);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 7);
}

#[test]
fn exists_test() {
    let mut tree: Tree<isize> = Tree::new(&mut [1,3,2,6,1,5,4,7]);
    //notice double 1.
    assert!(tree.exists(1));
    tree.pop(1);
    assert!(!tree.exists(1));
    //"proves" dupes cant exists
    
    assert!(!tree.exists(-1));
    tree.insert(-1);
    assert!(tree.exists(-1));
}

#[test]
fn insert_experimental_test() {
    let mut tree: Tree<isize> = Tree::new(&mut [1,0,5,4,3]);
    /*
    Tree should now be
        3
       / \
      1   5
     /   /
    0   4     
    */
    assert!(tree.head_node.clone().unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 5);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 4);

    tree.insert_experimental(2);
    /*
    Tree should now be
        3
       / \
      1    5
     / \   /
    0   2  4    
    */
    assert!(tree.head_node.clone().unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 5);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 4);

    tree.insert_experimental(-2);
    /*
    Tree should now be 
        2
       / \
      0    4
     / \   / \
   -2   1  3  5  
    */
    assert!(tree.head_node.clone().unwrap().value == 2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().value == 0);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().lower_child.unwrap().value == -2);
    assert!(tree.head_node.clone().unwrap().lower_child.unwrap().higher_child.unwrap().value == 1);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().value == 4);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().lower_child.unwrap().value == 3);
    assert!(tree.head_node.clone().unwrap().higher_child.unwrap().higher_child.unwrap().value == 5);
}