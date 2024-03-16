## Working with SurrealDB and Axum

This project is a simple example of how to use SurrealDB with the Axum web framework.

Using once_cell::sync::Lazy fot a thread-safe way of acessing our database inside of axum api routes.

```rust
type Db = Surreal<LocalDb>;
static DB: Lazy<Db> = Lazy::new(Surreal::init);
```

We introduce a **type alias** named `Db`. This alias is used to create an instance of the `Surreal` database, which is parameterized by a specific type called `LocalDb`. The second line declares a **static** variable named `DB`. The `Lazy` type ensures that the database is only initialized when it is first accessed.