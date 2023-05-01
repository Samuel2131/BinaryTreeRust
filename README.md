# BinaryTreeRust
## Simple binary tree in rust without using smart pointers;
## For the tree and the nodes i used structs with generic type, the struct has private unsafe functions and public safe functions;
## Functionality of BinaryTree struct:
* ***'new'*** function to initialize the tree;
* ***'get_height'*** function;
* ***'balance'*** function to rebalance the tree;
* ***'median'*** function to get a median between a tree value;
* insert node in tree with ***'uns_insert'*** private function and ***'insert'*** public function;
* delete node with ***'delete'*** function(in this function nothing is deallocated because when a node is deleted the tree must be rebalanced, and the balance function will call clear which will take care of deallocating);
* search for a node in the tree with the private function ***'uns_search'*** and the public function ***'search'***;
* ***'fill_tree'*** function to fill the tree starting from a generic iterator;
* ***'clear'*** and ***'uns_clear'*** function to empty the tree;
* ***'print_in_order'*** function to print all the values of the tree;
