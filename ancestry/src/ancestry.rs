use std::ptr::null;

use crate::ancestry;

pub struct PersonT {
    pub first_name: String, 
    pub last_name: String, 
    pub year_born: u32 
}

pub struct AncestryNodeT {
    pub mom: Option<Box<AncestryNodeT>>, // box is a pointer on the stack that points to data on the heap. mom owns a AncestryNodeT on the heap 
    pub dad: Option<Box<AncestryNodeT>>,
    pub person: PersonT, 
}


pub fn make_person(first_name: String, last_name: String, year_born: u32) -> PersonT {

    let person = PersonT { first_name, last_name, year_born }; 

    person
}

pub fn free_person(person: Box<PersonT>) {
    drop(person);
}

pub fn make_tree(person: PersonT) -> AncestryNodeT {

    let branch = AncestryNodeT { mom: None, dad: None, person: person }; 

    branch

}

pub fn add_mom(child_node: &mut AncestryNodeT, mom_person: PersonT) -> bool{

    if child_node.mom.is_some() {
        return false
    };

    let mom_node = AncestryNodeT { mom: None, dad: None, person: mom_person };

    child_node.mom = Some(Box::new(mom_node)); 

    true
}

pub fn add_dad(child_node: &mut AncestryNodeT, dad_person: PersonT) -> bool{

    if child_node.dad.is_some() {
        return false
    };

    let dad_node = AncestryNodeT { mom: None, dad: None, person: dad_person };

    child_node.dad = Some(Box::new(dad_node)); 

    true
}

pub fn free_tree(node: &mut AncestryNodeT) {
    if let Some(mut dad) = node.dad.take() {
        free_tree(&mut dad);
    }
    if let Some( mut mom) = node.mom.take() {
        free_tree(&mut mom);
    }
    
}


fn print_tree_recursive(node: &AncestryNodeT, prefix: &str, is_last: bool) {
    // Print the current node
    println!("{}{}{} {}", prefix, if is_last { "└── " } else { "├── " },
             node.person.first_name, node.person.last_name);

    // Create the new prefix for children
    let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    match (&node.mom, &node.dad) {
        (Some(mom), Some(dad)) => {
            print_tree_recursive(mom, &new_prefix, false);
            print_tree_recursive(dad, &new_prefix, true);
        }
        (Some(mom), None) => {
            print_tree_recursive(mom, &new_prefix, true);
        }
        (None, Some(dad)) => {
            print_tree_recursive(dad, &new_prefix, true);
        }
        (None, None) => {} 
    }
}

pub fn print_tree(node: &AncestryNodeT) {
    print_tree_recursive(node, "", true);
}