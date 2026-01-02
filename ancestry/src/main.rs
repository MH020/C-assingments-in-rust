use crate::ancestry::{AncestryNodeT, PersonT, add_dad, add_mom, free_tree, make_person, make_tree, print_tree};

mod ancestry;

fn main() {
    large_heap_allocated_tree(); 
}


fn small_stack_allocated_tree() {

    let anne_p = PersonT {first_name: "Anna".to_string(), last_name: "Person".to_string(), year_born: 1990 };
    let anne_mor = PersonT {first_name: "Annemor".to_string(), last_name: "Person".to_string(), year_born:  1970 };
    let anne_far = PersonT {first_name: "Annefar".to_string(), last_name: "Person".to_string(), year_born: 1969 };

    let anne_mor_tree = AncestryNodeT{ mom: None, dad: None, person: anne_mor };
    let anne_far_tree = AncestryNodeT{ mom: None, dad: None, person: anne_far };
    let anne_tree = AncestryNodeT{ mom: Some(Box::new(anne_mor_tree)), dad: Some(Box::new(anne_far_tree)), person: anne_p };

    print_tree(&anne_tree)
}

fn large_heap_allocated_tree() {
    let anne_p = PersonT {first_name: "Anna".to_string(), last_name: "Person".to_string(), year_born: 1990 };
    let mut anne_tree = make_tree(anne_p);

    add_mom(&mut anne_tree, make_person("Annemor".to_string(), "Person".to_string(), 1970));
    add_dad(&mut anne_tree, make_person("Annefar".to_string(), "Person".to_string(), 1969));

    add_mom( anne_tree.mom.as_mut().expect("no mom"), make_person("Annemormor".to_string(), "Person".to_string(), 1950));
    add_dad( anne_tree.mom.as_mut().expect("no dad"), make_person("Annemorfar".to_string(), "Person".to_string(), 1945));

    add_mom( anne_tree.dad.as_mut().expect("no mom"), make_person("Annefarmor".to_string(), "Person".to_string(), 1949));
    add_dad( anne_tree.dad.as_mut().expect("no dad"), make_person("Annefarfar".to_string(), "Person".to_string(), 1944));

    print_tree(&anne_tree);
    
    free_tree(&mut anne_tree);

    print_tree(&anne_tree);

}