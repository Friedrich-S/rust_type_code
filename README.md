# Rust Type Code

This repository contains a personal experiment to see to what extent it is
possible to execute code at compile time using only the Rust generics system.

A more detailed blog post will follow where the concepts used here are
explained.

## Basic Idea

The current implementation is very similar to Haskell. A function is represented
as some struct with a `Function` trait. Data is propagated using associated
types on the traits. The console output (IO) is stored in the `Function::Io`
associated type and represented using a type string.

### Type Strings

Strings are represented here using a list of characters. A character is
represented with a unique struct for that character and lists are represented
using nested tuples. The list `['A', 'B', 'C']` would be represented as
`(((TypeListElem<C>,), TypeListElem<B>), TypeListElem<A>)`.
