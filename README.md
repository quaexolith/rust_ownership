# Ownership in Rust
*Ownership* is a set of rules that governs how a Rust program manages memory.

## Ownership Rules
* Each value in Rust has an owner
* There can only be one owner at a time
* When the owner goes out of scope, the value will be dropped
