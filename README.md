```
module main # Define module's name
# Module should be named in lowercase

import (
    # Here your import modules
    hroma/io
    # hroma/files.(open)
)
# import hroma/lists - if you have only one import, parentheses could be avoided

main : IO () # Declare a main function signature
let main = { # Declare main function
    # main function is the start point of program

    # You can omit function signature name. In that case, it would bind to the next first function declaration
    : String -> String 
    let foo = bar -> bar <> "42"
                         ^--- string concationation operator

    # Function signature with () (Unit) param could be avoided. Function type would infer its return type
    let baz = foo "Your number is: " 
    #             ^--- parentheses in function call should be avoided
    #   ^--- String

    # Last expression or fuction call will be the return, no return keyword needed
    io.puts baz
}

: Int -> Int
let inc = n -> n + 1

# You can define a public function with ! (bang) operator. In that case other modules can access it
# By default, all modules's functions are private
: List Int -> List Int
let! pub_fn = list -> {
    let new_list, _ = map (inc, list) # parentheses can be avoided with $ operator: map $ inc list

    # map function returns a Pair, that you can directly unwrap in let statement
    # You can discard function result with _ (underscore) operator

    new_list
}

# somewhere in other module...
# module foo
# import project_name/main
# let baz = main.pub_fn [1, 2, 3]
#                       ^--- List literal

# Function application:
: ((Int -> Int), Int) -> Int
let applicate = f, n -> f n

# Recursion:
: Int -> Int
let rec factorial = n ->
#   ^-- recursive function declaration
    match n of
        0 -> 1
        n -> n * factorial (n - 1)

# Pairs, Records

# Pair is a data type with two slots, both of them can be any type
# You can create pair with Pair literal: (A, B)
let pair = (42, "42")

# Then you can access pair slots by unwrapping it in let declaration as mentioned earlier
let a, b = pair

# Another way to access pair slots is by pattern matching:
: (Int, Int) -> Int
let foo = pair ->
    match pair of
        (x, y) -> x + y

# Record is a data type that holds a set of entries - key:value
# To create a record you need to define a custom type:
let type MyRecord = {
    MyRecord {foo: Int, bar: String, baz: Int}
} deriving (NonLin)

# Then you can create it in a function:
let record = MyRecord {foo: 42, bar: "42", baz: 24}

# One way to access record fields is by dot accessor:
: MyRecord -> Int
let process_my_record = r -> r.foo # 42

# But a better way would be by using pattern matching:
# though in case of single field retrieval, dot accessor would be a bit faster to use
: MyRecord -> Int
let precess_my_record2 = r ->
    match r of
        MyRecord {foo, _, baz} -> foo + baz


# Special types: Result, Option, List

# Result is a ADT mostly used to be returned in a function
let type Result T, E = {
    Ok T
    Err E
} # Result type definition


# Example of Result usage
: (Int, Int) -> Result (Int, String)
let safe_division = a, b ->
    match b of
        0 -> Err "Division by zero"
        n -> Ok (a / b)

# Then you can use it in pattern matching:
match safe_division (10, 0) of
    Ok n -> io.puts (show n)
    Err e -> io.puts ("Error in division function: " <> e)

# Option is a similar type:
let type Option T = {
    Some T
    None
}

# It could be used for optional parameter:

: (Int, Option Int) -> Int
let bar = a, b ->
    match b of
        Some n -> a + b
        None -> a

let baz = bar (4, Some(38)) # 42

# Linear types:
# By default all ADT types in Hroma are Linear
# Even if it contains only primitive types 
let type MyType = {
    Foo
    Baz Int
    Bar {a: Int, b: String}
} deriving (Drop) # Standart Drop trait satisfies only if all of its variants consists of NonLin types


# Record field dot-accessing will consume whole record if record in linear
let foo = Bar {a: 42, b: "42"}
let bar = foo.a # foo is consumed here
let baz = foo.b # Compiler error: Linear type used twice

# If you need to acces all fields you must use pattern matching:
let fred = Bar {a: 42, b: "42"}
match fred of
    Bar {a, b} -> io.puts (b <> show a)

# If you need non linear ADT you can derive NonLin trait:
let type MyNonLinType = {
    Bar {foo: Int, bar: String}
} deriving (NonLin)
# Type can satisfy NonLin trait only if all of its variants consists of only NonLin types (types that also satisfy that trait)
# Types like Int, Bool, Float also satisfies NonLin trait

# Polymorphic type like List, Result, Pair satisfies NonLin type only if all of its parametres is NonLin

# You can achive same behavior with auto keyword and type constraints:
let type Either E, T = {
    Left E
    Right T
}

auto NonLin for Either E, T where E: NonLin, T: NonLin

# Type constraints should also be used in polymorphic function signature

polymorphic_fn : (a, b) -> c where a: NonLin, c: NonLin
let polymorphic_fn = a, b -> {...}


# If your resource need special drop logic you can implement it with this syntax:
let type DB = {
    Database String
} 

impl Drop for DB = {
    drop = db ->
        match db of
            Database addr -> driver.close_connection addr
}

# Hroma implements RAII, so the drop function will be called right after the resource is out of its scope

let program = {
    let db = psql.open "localhost"
    let res, _ = psql.query (db, "SELECT * FROM foo")

    io.puts res
    # after that db connection will be auth close
}

# Pipe-operator:

# Imperative style:
let list = lists.concat ([1, 2, 3, 4, 5], [1, 2])
list len, _ = lists.len list
io.puts len

# Can be replaced by Pipe-operator

([1, 2, 3, 4, 5], [1, 2])
    ||> lists.concat # equivalent to lists.concat ([1, 2, 3, 4, 5], [1, 2])
    #   ^--- (List Int, List Int) -> List Int
    |> lists.len # equivalent to lists.len [1, 2, 3, 4, 5, 1, 2] this function will return length of list and a list back
    |> io.puts # >> 7
    # Though previous operation returns (Int, List Int), second slot in the pair is discarded
    # (same as let len, _ = lists.len list)
    # If you need to preserve whole pair, use ||> operator
    # если нужно ее сохранить, то используйте ||>
```
