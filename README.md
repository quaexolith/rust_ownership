# Ownership in Rust
*Ownership* is a set of rules that governs how a Rust program manages memory.

## Ownership Rules
* Each value in Rust has an owner
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped

## Variable Scope
A scope is the range within a program for which an item is valid.

```
{                       // s is not valid here, it's not yet declared
    let s = "hello";    // s is valid from this point forward

    // do stuff with s
}                       // this scope is now over, and s is no longer valid
```

## The String Type

### Immutable
```
let s = String::from("hello");
```

### Mutable
```
let mut s = String::from("hello");

s.push_str(", world!");

println!("{}", s);
```

## Memory and Allocation
String literal contents are known at compile time so the text is hardcoded directly into the final executable.

With the `String` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents.

* The memory must be requested from the memory allocator at runtime
* We need a way of returning this memory to the allocator when we're done with our `String`

The first part is covered by `String::from`; its implementation requests the memory it needs.

The second part is different. In some languages, a garbage collector is used. In others, it's the programmer's responsibility to allocate and free memory.

There should only ever be one `allocate` to one `free`.

In Rust, memory is automatically returned once the variable that owns it goes out of scope. Rust calls a special function called `drop` which is where the author of `String` can put the code to return the memory. Rust calls `drop` automatically at the closing curly bracket.
