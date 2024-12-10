use tree::Node;

fn main() {
    let mut root = Node::new("root".to_string());
    let mut child_a = root.add_child("child_a".into());
    child_a.add_child("child_aa".into());
    child_a.add_child("child_ab".into());
    let mut child_b = root.add_child("child_b".into());
    println!("\n\nTree befor detach: {root:#?}");
    child_a.detach();
    println!("\n\nDetached child_a: {child_a:#?}");
    println!("\n\nTree after detach: {root:#?}");
    child_a = child_b.attach(child_a);
    *child_a.borrow_mut() = "roota".into();
    println!("\n\nTree after attach: {root:#?}");
}
