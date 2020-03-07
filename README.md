# typenum-promote
Promote integer literal to type-level integer

```rust
use typenum::Unsigned
use typenum_promote::promote;

assert_eq!(<promote!(12345) as Unsigned>::to_u64(), 12345);
```
