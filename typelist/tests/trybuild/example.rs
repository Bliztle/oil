extern crate typelist;
use std::marker::PhantomData;
use typelist::typelist;

struct Foo;
struct Bar;

typelist!(3, Foo, Bar);

// Node struct with generic state
struct Node<S> {
    value: i32,
    _state: PhantomData<S>,
}

impl Default for Node<Nil> {
    fn default() -> Self {
        Node {
            value: 0,
            _state: PhantomData,
        }
    }
}

impl<S> Node<S> {
    fn foo(&self) -> Node<Cons<Foo, S>> {
        Node {
            value: self.value + 1,
            _state: PhantomData,
        }
    }

    fn bar(&self) -> Node<Cons<Bar, S>> {
        Node {
            value: self.value + 1,
            _state: PhantomData,
        }
    }
}

impl<S> Node<S>
where
    S: Includes<Nil>,
{
    fn only_on_nil(&self) {
        println!("Only on foo'd struct");
    }
}

impl<S> Node<S>
where
    S: Includes<Foo>,
{
    fn only_on_food(&self) {
        println!("Only on foo'd struct");
    }
}

impl<S> Node<S>
where
    S: Includes<Foo> + Includes<Bar>,
{
    fn only_on_food_and_bard(&self) {
        println!("Only on foo'd and bar'd struct");
    }
}

#[rustfmt::skip]
pub fn main() {
    let node = Node::default();

    node.only_on_nil();
    node.foo().only_on_nil();
    node.foo().foo().only_on_nil();
    node.foo().bar().only_on_nil();

    node.bar().foo().only_on_food_and_bard();
    node.bar().only_on_food_and_bard(); // Bad. Need both Foo and Bar
    node.foo().only_on_food_and_bard(); // Bad. Need both Foo and Bar

    node.foo().only_on_food();
    node.foo().bar().only_on_food();
    node.bar().foo().only_on_food();
    node.bar().foo().bar().only_on_food();
    node.bar().foo().bar().bar().only_on_food(); // Bad, typelist! was called with depth 3

    node.foo().foo().only_on_food();

    node.bar().bar().only_on_food(); // Bad. Need Foo
    node.only_on_food(); // Bad. Need Foo
}
