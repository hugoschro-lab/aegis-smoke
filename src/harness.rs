//! Kani proof harness. **Locked** — the Aegis loop is not allowed to rewrite
//! this file. The orchestrator reads it as immutable context.
//!
//! Specification: `transfer` must not panic for *any* `(from, to, amount)`
//! triple. A correct implementation either performs the transfer (when
//! preconditions hold) or refuses safely (when they would overflow or
//! underflow). It must never invoke unchecked arithmetic that panics.
//!
//! The buggy implementation in `lib.rs` panics on underflow (`amount > from`)
//! and on overflow (`to + amount > u64::MAX`). Kani's job is to find one of
//! those counterexamples; the loop's job is to repair the implementation
//! until Kani is satisfied.

use crate::transfer;

#[kani::proof]
fn check_transfer_never_panics() {
    // Fully unconstrained inputs — Kani must explore the entire u64 cube.
    let mut from: u64 = kani::any();
    let mut to: u64 = kani::any();
    let amount: u64 = kani::any();

    // Calling transfer with any valid u64 triple must not panic. A correct
    // implementation handles overflow/underflow defensively (e.g.
    // `checked_sub` / `checked_add` returning early on `None`).
    transfer(&mut from, &mut to, amount);
}
