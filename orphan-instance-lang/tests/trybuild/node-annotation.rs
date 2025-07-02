extern crate orphan_instance_lang;
use orphan_instance_lang::ast::Type;
use orphan_instance_lang::node::Node;

#[rustfmt::skip]
fn doesnt_compile() {
    let node: Node<i32> = 0.into();
    let _: &Type = node.get_annotation(); // Shouldn't be able to get annotation not added
    node.change_annotation(Type::I32); // Shouldn't be able to change annotation not added
    let node = node.add_annotation(Type::I32);
    let node = node.add_annotation(Type::I32); // Shouldn't be able to add annotation twice
    let _ = node.add_annotation_unchecked(Type::I32); // Shouldn't be able to add unchecked when annotation is already checked
    let _: &Type = node.get_annotation_unchecked(); // Shouldn't be able to get unchecked when annotation is already checked
}

fn main() {}
