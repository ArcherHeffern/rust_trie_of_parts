# Trie focusing on sections of arbitrary data structures instead of raw strings
Personal usecase was quickly searching for valid paths 

```rust
let mut trie: Trie<String, bool> = Trie::new();
let path_1 = Path::new("/etc/bin/echos");
let nonexistant_path = Path::new("/etc/bin/echo");

trie.insert(path_to_iter(path_1), true);
assert!(trie.contains(path_to_iter(path_1)));
assert!(!trie.contains(path_to_iter(nonexistant_path)));


let path_2 = Path::new("/etc/bin/echo/env");
trie.insert(path_to_iter(path_2), true);
let target_path = Path::new("/etc/bin/echo/env/frogs");

assert_eq!(trie.best_match(path_to_iter(target_path)), Some(path_2.clone()));
```

See `lib.rs` tests for more examples.