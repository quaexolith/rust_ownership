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
