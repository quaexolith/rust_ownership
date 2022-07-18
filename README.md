# Ownership in Rust
*Ownership* is a set of rules that governs how a Rust program manages memory.

## The Stack and the Heap
Values that are known at compile time can be placed on the stack.

Values that are known at runtime must be placed on the heap. This is referred to as **allocating**. The memory allocator searches the heap for a space big enough to store the data and returns a pointer to it. The pointer may be stored on the stack but to retrieve the actual data, the pointer must be followed.

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

## Ways Variables and Data Interact: Move
Multiple variables can interact with the same data in different ways in Rust.

### Assigning the Integer Value of `x` to `y`
```
let x = 5;
let y = x;
```

Because integers are simple values with a known, fixed size, and these two `5` values are pushed onto the stack.

### Assigning the String Value of `s1` to `s2`
```
let s1 = String::from("hello");
let s2 = s1;
```

This looks similar to the `integer` example but what happens in memory is different.

A `String` is made up of 3 parts:
* A pointer to the memory that holds the contents of the string
* a length; how much memory, in bytes, the `String` is currently using
* a capacity; how much memory, in bytes, the `String` has received from the allocator

This group of data is stored on the stack. The actual contents of the string are stored on the heap.

When `s1` is assigned to `s2`, the `String` data is copied, meaning the values stored on the stack. The actual contents are not copied on the heap.

In order to avoid a **double free** error (freeing memory twice), when `s1` is assigned to `s2`, Rust invalidates `s1`. `s1` is no longer valid and using it causes an error. `borrow of moved value: s1`.

Since Rust copies the data on the stack and invalidates the first variable, the operation is referred to as a **move** rather than a **shallow copy**. There is a design choice implied by this: Rust will never automatically create "deep" copies of your data.
