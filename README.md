## Working with SurrealDB and Axum

This project is a simple example of how to use SurrealDB with the Axum web framework.

Using once_cell::sync::Lazy fot a thread-safe way of acessing our database inside of axum api routes.

```rust
type Db = Surreal<LocalDb>;
static DB: Lazy<Db> = Lazy::new(Surreal::init);
```

We introduce a **type alias** named `Db`. This alias is used to create an instance of the `Surreal` database, which is parameterized by a specific type called `LocalDb`. The second line declares a **static** variable named `DB`. The `Lazy` type ensures that the database is only initialized when it is first accessed.

```rust
let task_routes = api::task_routes::routes(DB.clone());
```

We pass the `DB` instance to the `task_routes` function, which is responsible for defining the routes for the task API. Cloning the `DB` instance allows us to share the same database across multiple routes but doen't clone the database itself because it's using Lazy.

```rust
    TaskService { db: &db}
        .create_task(input)
        .await
        .map(Json)
```

Each function takes a reference to the `DB` instance and uses it to interact with the database by calling the `create_task` method. The `create_task` method is an asynchronous function that returns a `Result` type. If the result is successful, the `Json` function is called to convert the result into a JSON response.