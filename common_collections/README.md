Rust: Collections

This document gives a brief overview of common collections in Rust: Vector, String, and HashMap.

Vector
- Heap-allocated, growable array type: Vec<T>.
- Elements must have the same type T; types are checked at compile time.
- Common operations: push, pop, indexing, iteration.
- Allocation and resizing happen at runtime.

String
- Owned, growable UTF-8 encoded string type: String.
- Use &str for string slices and String when ownership or mutation is needed.
- Strings are sequences of bytes representing UTF-8; be mindful of character boundaries when indexing.

HashMap
- Key-value store from std::collections::HashMap<K, V>.
- Choose an appropriate hasher if performance or determinism matters (the default hasher is cryptographically secure but not always the fastest).
- Common operations: insert, get, remove, contains_key.
