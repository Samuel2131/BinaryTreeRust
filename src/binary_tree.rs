

use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashSet;
use rand::Rng;

fn get_layout<T>() -> Layout {
    return Layout::new::<Node<T>>();
}

struct Node<T> {
    element: T,
    deep: usize,
    character: u8,
    father: Option<*mut Node<T>>,
    left: Option<*mut Node<T>>,
    right: Option<*mut Node<T>>
}

#[derive(Clone)]
pub struct BinaryTree<T> {
    root: Option<*mut Node<T>>,
    height: usize,
    median: Option<T>,
    set_element: HashSet<T>,
}

impl <T> BinaryTree<T> where T: std::clone::Clone + std::fmt::Display + std::cmp::PartialEq + PartialOrd + Ord + Eq + std::hash::Hash {

    pub fn new() -> BinaryTree<T> {
        return BinaryTree{root: None, height: 0, median: None, set_element: HashSet::<T>::new() };
    }

    unsafe fn free(&self, node: *mut Node<T>) {
        dealloc(node as *mut u8, get_layout::<T>());
    }

    fn inc_height(&mut self) {
        self.height += 1;
    }

    unsafe fn create_node(&self, element: T, deep: usize, father: Option<*mut Node<T>>, left: Option<*mut Node<T>>, right: Option<*mut Node<T>>) -> Option<*mut Node<T>> {
        let mut r = rand::thread_rng();
        let new_node = alloc(get_layout::<T>()) as *mut Node<T>;
        *new_node = Node {element: element, deep: deep, character:(r.gen_range(65..91)),  father: father, left: left, right: right};
        return Some(new_node);
    }

    fn median(&self) -> T {
        let mut vec:Vec<_> = self.set_element.iter().collect();
        vec.sort();
        return (**(vec.get(vec.len()/2).unwrap())).clone();
    }

    unsafe fn balance(&mut self) {
        let mut temp_set = self.set_element.clone();
        let median = self.median();
        self.clear();
        println!("Median = {}", median);
        
        self.uns_insert(self.root, median.clone());
        self.set_element.insert(median.clone());
        temp_set.retain(|x| *x != median);
        for element in temp_set.iter() {
            self.uns_insert(self.root,(*element).clone());
            self.set_element.insert((*element).clone());
        }
    }
    
    unsafe fn uns_insert(&mut self, node: Option<*mut Node<T>>, element: T) {
        match node {
            None => {
                self.root = self.create_node(element, self.get_height(), None, None, None);
            },
            Some(item) => {
                if element > (*item).element { 
                    match (*item).right {
                        Some(item) => self.uns_insert(Some(item), element),
                        None => {
                            let new_node = self.create_node(element, (*item).deep+1, Some(item), None, None);
                            (*item).right = new_node;
                            if ((*item).deep+1) > self.height { self.inc_height(); }
                        }
                    }
                }
                else if element < (*item).element { 
                    match (*item).left {
                        Some(item) => self.uns_insert(Some(item), element),
                        None => {
                            let new_node = self.create_node(element, (*item).deep+1, Some(item), None, None);
                            (*item).left = new_node;
                            if ((*item).deep+1) > self.height { self.inc_height(); }
                        }
                    }
                }
                else { return; }
                
            }
        }
    }
    
    unsafe fn print_node(&self, node: Option<*mut Node<T>>) {
        match node {
            Some(item) => {
                print!("Address node = {:p}, Character = {}, Element = {}, deep = {}", item, (*item).character as char, (*item).element, (*item).deep);
                match (*item).father {
                    Some(father) => { print!(", Father = {}", (*father).element); },
                    None => { print!(", Father = None"); }
                }
                match (*item).left {
                    Some(left) => { print!(", Left = {}", (*left).element); },
                    None => { print!(", Left = None"); }
                }
                match (*item).right {
                    Some(right) => { print!(", Right = {}", (*right).element); },
                    None => { print!(", Right = None"); }
                }
            },
            None => ()
        }
        println!();
    }

    unsafe fn uns_print_in_order(&self, node:Option<*mut Node<T>>) {
        match node {
            Some(item) => {
                self.uns_print_in_order((*item).left);
                self.print_node(Some(item));
                self.uns_print_in_order((*item).right);
            },
            None => ()
        }
    }

    unsafe fn uns_clear(&mut self, node: Option<*mut Node<T>>) {
        match node {
            Some(item) => {
                self.uns_clear((*item).left);
                self.uns_clear((*item).right);
                self.free(item);
            },
            None => ()
        }
    }

    fn control_median(&mut self) {
        unsafe {
            self.balance();
            self.median = Some(self.median());
        }
    }

    unsafe fn uns_search(&self, node: Option<*mut Node<T>>, i: usize, element: T) -> Option<char> {
        match node {
            Some(item) => {
                if  element == (*item).element {
                    return Some((*item).character as char);
                }
                else if element > (*item).element {
                    return self.uns_search((*item).right, i+1, element);
                }
                else {
                    return self.uns_search((*item).left, i+1, element);
                }
            },
            None => return None
        }
    }

    //Interfaces: 
    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn insert(&mut self, element: T) {
        unsafe {
            self.uns_insert(self.root, element.clone());
            self.set_element.insert(element);
            self.control_median();
        }
    }

    pub fn delete(&mut self, element: T) {
        self.set_element.remove(&element);
        self.control_median();
    }

    pub fn fill_tree(&mut self, iter: impl Iterator<Item=T>) {
        for element in iter {
            self.set_element.insert(element);
        }
        self.control_median();
    }

    pub fn search(&self, element: T) -> Option<char> {
        unsafe {
            return self.uns_search(self.root, 0, element);
        }
    }

    pub fn print_in_order(&self) {
        unsafe {
            self.uns_print_in_order(self.root);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            self.uns_clear(self.root);
            self.root = None;
            self.height = 0;
            self.median = None;
            self.set_element.clear();
        }
    }

}