# Intent — `transfer`

Account-to-account balance transfer for two `u64` accumulators.

## Crate overview

A single primitive that moves a counted amount of units between
two mutable balances. Conservation is the load-bearing property:
no units may be created or destroyed, and the sum of the two
balances must be preserved across every call.

## `transfer`

```rust
pub fn transfer(from: &mut u64, to: &mut u64, amount: u64)
```

Move `amount` units from `from` to `to`. On success, `*from`
decreases by `amount` and `*to` increases by `amount`. The
function must be total over `(u64, u64, u64)`: every input
combination produces a defined behaviour. It must NOT panic for
any input.

When the transfer would underflow `*from` (i.e. `amount > *from`)
or overflow `*to` (i.e. `*to + amount > u64::MAX`), the function
must take the rejection branch: leave both balances unchanged
and return without panicking. There is no `Result` return — the
caller observes rejection only by seeing both arguments
unmodified.

The cross-call invariant is `*from + *to` (mod `u64::MAX + 1`)
remains constant from before the call to after.
