# `typelist`

A flexible and zero-cost abstraction over the *typestate pattern*, enabling type-level state tracking in Rust through compile-time type lists.

Use `typelist` to express which states or annotations have been applied to a value, ensuring correctness and order of operations **at compile time**.

---

## ✨ Features

- **Ergonomic typestate tracking** with compile-time safety
- **Flexible macros** to construct nested type lists
- **Powerful trait-based querying**, like `Includes<T>`
- **No runtime overhead** — all information is encoded at the type level
- Ideal for **builder patterns**, **AST transformations**, or **typed pipelines**

---

## 📦 Usage

### Defining Type States

```rust
use std::marker::PhantomData;
use typelist::typelist;

// Define your marker types (states)
struct FooState;
struct BarState;

// Generate typelist types
typelist!(2, FooState, BarState);

// A generic struct that tracks state through its type
struct Node<S = Nil> {
  _state: PhantomData<S>,
}
```

### Transitioning Between States

```rust
impl<S> Node<S> {
  pub fn new() -> Self {
    Node { _state: PhantomData }
  }

  pub fn foo(&self) -> Node<Cons<FooState, S>> {
    Node::new()
  }

  pub fn bar(&self) -> Node<Cons<BarState, S>> {
    Node::new()
  }
}
```

### Restricting Functionality by Type State

```rust
use typelist::Includes;

// Only available if FooState is present
impl<S> Node<S> where S: Includes<FooState> {
  pub fn only_on_foo(&self) { }
}

// Only available if BarState is present
impl<S> Node<S> where S: Includes<BarState> {
  pub fn only_on_bar(&self) { }
}

// Only available if both states are present
impl<S> Node<S> where S: Includes<FooState> + Includes<BarState> {
  pub fn only_on_foo_and_bar(&self) { }
}
```

### Example Usage

```rust
let node = Node::new();

// ✅ Allowed
node.foo().only_on_foo();
node.bar().foo().only_on_foo();
node.foo().bar().only_on_foo();
node.foo().bar().only_on_foo_and_bar();
node.bar().foo().only_on_foo_and_bar();

// ❌ Not allowed (would fail to compile)
// node.bar().only_on_foo();
// node.foo().only_on_foo_and_bar();
// node.bar().only_on_foo_and_bar();
```

---

## ✅ Use Cases

- **AST annotation tracking**: Ensure each node is transformed exactly once or in proper order.
- **Builder patterns**: Enforce build-time field initialization with compile-time safety.
- **Typed pipelines**: Chain operations while statically tracking configuration or transformation state.
- **Compile-time validation**: Catch misuse at the type level with zero runtime checks.

---

## 📦 Installation

In your `Cargo.toml`:

```toml
typelist = { git = "https://github.com/your-org/typelist" }
```

Or use a local path:

```toml
typelist = { path = "../typelist" }
```

---

## 📄 License

Licensed under the [MIT License](LICENSE-MIT)
