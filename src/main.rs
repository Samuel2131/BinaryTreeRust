
mod binary_tree;

fn main() {
   let mut tree = binary_tree::BinaryTree::<i32>::new();
  
   let my_vec = vec![1, 10, 28, 5, 7, 13, 50, 24];
   tree.fill_tree(my_vec.into_iter());
   tree.print_in_order();
   println!("\nHeight : {}", tree.get_height());

   tree.insert(2);
   tree.insert(100);
   tree.insert(-18);
   tree.insert(0);
   tree.insert(4);
   tree.print_in_order();
   println!("\nHeight : {}", tree.get_height());

   tree.delete(2);
   tree.delete(10);
   tree.print_in_order();
   println!("\nHeight : {}", tree.get_height());

   match tree.search(0) {
      Some(c) => println!("Char at index = {}, is {}", 0, c),
      None => println!("Element not found at index = {}", 0)
   }

   match tree.search(28) {
      Some(c) => println!("Char at index = {}, is {}", 28, c),
      None => println!("Element not found at index = {}", 28)
   }

   match tree.search(100) {
      Some(c) => println!("Char at index = {}, is {}", 100, c),
      None => println!("Element not found at index = {}", 100)
   }

   match tree.search(21) {
      Some(c) => println!("Char at index = {}, is {}", 21, c),
      None => println!("Element not found at index = {}", 21)
   }

   tree.clear();
}
