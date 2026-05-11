//! Behaviour gate. **Locked** — the Aegis loop is not allowed to rewrite
//! this file. `cargo test` must pass after a patch for the run to count as
//! successful, alongside the Kani gate and the bypass scanner.

use aegis_example_transfer::transfer;

#[test]
fn transfers_a_normal_amount() {
    let mut from = 100u64;
    let mut to = 50u64;
    transfer(&mut from, &mut to, 30);
    assert_eq!(from, 70);
    assert_eq!(to, 80);
}

#[test]
fn transfers_zero() {
    let mut from = 10u64;
    let mut to = 5u64;
    transfer(&mut from, &mut to, 0);
    assert_eq!(from, 10);
    assert_eq!(to, 5);
}

#[test]
fn transfers_entire_balance() {
    let mut from = 42u64;
    let mut to = 0u64;
    transfer(&mut from, &mut to, 42);
    assert_eq!(from, 0);
    assert_eq!(to, 42);
}
