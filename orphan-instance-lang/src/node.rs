//! This module defines the Node wrapper used in the AST
//! It exposes an annotation system which strictly enforces type safety on existing annotations,
//! only allowing get / set operations with preconfigured types

use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};
use typelist::typelist;

pub trait Annotation: 'static {}

struct Type;
impl Annotation for Type {}

typelist!(1, Type);

#[derive(Debug)]
pub struct Node<T, S = Nil> {
    pub node: T,
    annotations: HashMap<TypeId, Box<dyn Any>>,
    pub _state: PhantomData<S>,
}

impl<T> From<T> for Node<T> {
    fn from(value: T) -> Self {
        Node::new(value)
    }
}

impl<T> From<T> for Box<Node<T>> {
    fn from(value: T) -> Self {
        Node::from(value).into()
    }
}

impl<T, S> Node<T, S> {
    pub fn new(value: T) -> Self {
        Node {
            node: value,
            annotations: HashMap::new(),
            _state: PhantomData,
        }
    }

    /// Adds the provided annotation to the node, and returns a new node with the updated signature
    /// This invalidates the old node, as it is impossible to retrieve a value not included in the
    /// node's signature. Only the new node should be used from then on.
    pub fn add_annotation<U>(mut self, annotation: U) -> Node<T, Cons<U, S>>
    where
        U: Annotation,
        S: Excludes<U>,
    {
        self.annotations
            .insert(TypeId::of::<U>(), Box::new(annotation));
        Node {
            node: self.node,
            annotations: self.annotations,
            _state: PhantomData,
        }
    }

    pub fn add_annotation_unchecked<U>(&mut self, annotation: U)
    where
        U: Annotation,
        S: Excludes<U>,
    {
        self.annotations
            .insert(TypeId::of::<U>(), Box::new(annotation));
    }

    pub fn change_annotation<U>(&mut self, annotation: U)
    where
        U: Annotation,
        S: Includes<U>,
    {
        self.annotations
            .insert(TypeId::of::<U>(), Box::new(annotation));
    }

    /// # Panics
    pub fn get_annotation<U>(&self) -> &U
    where
        U: Annotation,
        S: Includes<U>,
    {
        let annotation = self
            .annotations
            .get(&TypeId::of::<U>())
            .and_then(|boxed| boxed.downcast_ref::<U>());
        annotation
            .expect("Annotation did not exist, which should have been guaranteed by type system")
    }

    pub fn get_annotation_unchecked<U>(&self) -> Option<&U>
    where
        U: Annotation,
        S: Excludes<U>,
    {
        self.annotations
            .get(&TypeId::of::<U>())
            .and_then(|boxed| boxed.downcast_ref::<U>())
    }
}
