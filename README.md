# zscan

Byte and substring search with no dependencies, no allocation and an always
`no_std` library. Rust 1.70 or newer; MIT licensed.

```rust
use zscan::{find, rfind, find_byte, rfind_byte, Finder};

assert_eq!(find_byte(0, b"ab\0cd"), Some(2));
assert_eq!(rfind_byte(b'a', b"aba"), Some(2));
assert_eq!(find(b"ababa", b"aba"), Some(0));
assert_eq!(rfind(b"ababa", b"aba"), Some(2));
let finder = Finder::new(b"aba");
assert_eq!(finder.find(b"xxaba"), Some(2));
assert_eq!(finder.rfind(b"ababa"), Some(2));
```

All inputs are byte slices, including NUL and invalid UTF-8. Returned positions
are byte offsets in the original haystack. An empty needle matches at `0`
forwards and `haystack.len()` backwards. An overlong needle returns `None`.

Byte search takes O(n) time. Two-Way substring search takes worst-case O(n + m)
time and O(1) extra space. `Finder` borrows its needle and stores fixed-size
metadata for both directions; it never copies or reverses the input.

```sh
cargo test --locked
cargo test --release --locked
cargo test --doc --locked
```
