# Darcy Feature Set
Darcy feature set includes all intended features for Darcy, including standard library implementations

## Commentary:
Darcy is supposed to be a language that is easy to pick up and learn for people with no computer science experience. That being said, it should be very capable and be able to perform large scale operations, plotting, etc. The features should be elegantly implemented and easy to use. Syntax should be familiar to existing programming languages but easy to understand for new programmers. Ideally, in basic operations, with the exception of some keywords and symbols, a non-programmer should be able to tell vaguely what is happening in any specific part of code. 

I am going forward with great caution that this does not turn out to be Julia #2 but worse. Julia is in many ways the superior language to R, however because it's complicated and the syntax is unfamiliar, I think a lot of people avoid it.

On top of this, Darcy needs to offer incredible tooling for a wide variety of code editors and if posible, I would really like to make a custom Darcy IDE in egui or Tauri that makes writing Darcy code a much more visual experience and allows for the traversal and viewing of data to be a lot easier, along with things like graphs and major functions. Darcy's package manager should be exceptional, especially when compared to tools offerred by competitors (i hate pip so much >:O ).

I say all of this but this is assuming I even ever finish the damn project. 

## Featureset:
- [ ] Variables, procedures, and mathematical operations
- [ ] Unique data types like arrays, matricies, lists, vectors, etc.
- [ ] Custom types like classes, structs, tuples, enums, etc.
- [ ] Functional programming feautures like closures and anonymous functions
- [ ] Support for manual multi-threading and concurrency
- [ ] Support for vectorized operations
- [ ] Support for a standard plotting library

## Basics:
- [ ] Variables
- [ ] Procedres
- [ ] Mathematical operations
- [ ] Type casting
- [ ] Selection
- [ ] Iteration
- [ ] Pattern matching

Variables:
```
<name>: <type> = <value>

x = 10
y = 2.3
z: string = "hello"

print x, y, z

x -> 5
y -> 3.14

print x, y
```

Procedures:
```
proc <name> (<parameters>) returns <return type> {
    <procedure body>
}

proc add_five (num: number) returns number {
    return num + 5
}

add_five(10)            | 15
```

Mathematical Operations
```
10 / 2                 | 5
10 * 2                 | 20
10 + 2                 | 12
10 - 2                 | 8
10 % 2                 | 0
10 ^ 2                 | 100

use math
math.factorial(10)     | 3628800
math.sqrt(64)          | 8
math.sin(30)           | 0.5
math.cos(60)           | 0.5
math.tan(45)           | 1
math.asin(0.5)         | 30
math.acos(0.5)         | 60
math.atan(1)           | 45
```

## Data Types
- [ ] Lists
- [ ] Matricies
- [ ] Vectors
- [ ] Standard library implementation

## Custom Types
- [ ] Classes
- [ ] Structs
- [ ] Enums
- [ ] Tuples

## Functional Programming
- [ ] Closures
- [ ] Anonymous functions

## Multi-threading & Concurrency
- [ ] Streamlined support for coroutines
- [ ] Streamling async/await syntax

## Vectorize Operations
- [ ] Elementwise operations
- [ ] Aggregate operations
- [ ] Array manipulation
- [ ] Boolean operations
- [ ] Mathematical operations