**Linea** is a statically-typed functional programming language for building reliable and efficient applications. Its core innovation is a default-linear type system that guarantees resource safety without sacrificing expressiveness, combined with an elegant syntax that blends the power of mathematical foundations with practical simplicity.

> Linea is under active development
> syntax is the subject to change

## Philosophy

Linea is built on three core principles:

1. Safety by Default: Linear types eliminate resource management errors (use-after-free, double-free) at compile time. (No GC at all!)

2. Expressiveness without Clutter: A concise syntax, type inference, and a powerful trait system allow you to write complex logic clearly and simply.

3. Practicality: The language (will provide) provides tools for real-world tasks—IO, networking, concurrency—while staying true to the functional paradigm.

## Key Features

- Default-Linear Type System for guaranteed memory and resource safety without expensive GC.

- Strong Static Typing with type inference.

- Algebraic Data Types (ADT) and pattern matching (match).

- High-Level Abstractions: polymorphism and custom traits (typeclasses).

- Side-Effect Management via built-in IO, Result, and Option types.

- Elegant Syntax with lightweight parenthesis rules and pipe operators (|>, ||>).

- Module System with visibility control (let! for public functions).

## Your first programm

```linea
module main

import linea/io

main : IO ()
let main = {
    let message = "Hello, World!"

    io.puts message
}
```

## Resource safety with Linear Types

```linea
# A linear type representing a database connection
let type DBConnection = {
    Database String
}

# Custom destructor logic; called automatically when the value goes out of scope (RAII)
impl Drop for DBConnection = {
    let drop = conn ->
        match conn of
            Database addr -> driver.close_connection addr
}

: IO () # nameless (implicit) signature bounding
let main = {
    let db = psql.open "localhost:5432" # `db` is of linear type DBConnection
    let result, db2 = psql.query (db, "SELECT * FROM users")
    # `db` is consumed by `query` and is no longer available here
    io.puts (show result)
    # `drop` is called automatically here if the resource hasn't been consumed


    # or with pipe composition
    # "localhost:5432"
    #   |> psql.open
    #   |> psql.query "SELECT * FROM users"
    #   |> show
    #   |> io.puts
    #   ^--- drop will be called at the end of the pipe chain
}
```

## Error Handling with `Result` and `match`

```linea
import linea/result

: (Int, Int) -> Result (Int, String)
let safe_divide = a, b ->
    match b of
        0 -> Err "Division by zero"
        _ -> Ok (a // b)

let result = safe_divide (10, 2)
    |> result.map_err (\n -> n * 3) # Transform the result if it's `Ok`

match result of
    Ok value -> io.puts ("Success: " <> show value)
    Err error -> io.puts ("Error: " <> error)
```

## Defining and Using Traits (Typeclasses)

```linea
# Define a trait for types that can be represented as a String
let trait Show = {
    show : Self -> String
}

# Define a custom type
let type User = {
    User { name: String, age: Int }
}

# Implement the Show trait for User
impl Show for User = {
    let show = user ->
        match user of
            User {name, age} -> name <> " (" <> show age <> " years old)"
}

let user = User {name: "Alice", age: 30}
io.puts (show user) # "Alice (30 years old)"

```

## Polymorphic (generic) functions and types

```linea
map : ((a -> b), List a) -> (List b, List a) where a: NonLin
let rec map = f, l ->
    match l of
        [] -> ([], [])
        [x, ..xs] -> {
            let y = f x # `x` is not consumed because we ensure that `x` is not linear in type constraint
            let ys, new_xs = map (f, xs) # `xs` is consumed because List doesn't have NonLin trait

            ([y, ..ys], [x, ..new_xs])
        }

let type Either E, T = { # Same as Result but with different name
    Left E
    Right T
}
# Automatically make type derive NonLin if both generics have NonLin trait
auto NonLin for Either E, T where E: NonLin, T: NonLin
# Same with Drop trait
auth Drop for Either E, T where E: NonLin, T: NonLin
# In previous example List didn't have auto NonLin, that is why `xs` is linear
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
