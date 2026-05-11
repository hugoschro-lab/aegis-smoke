# Contract — `transfer`

Move `amount` units from the balance pointed to by `from` to the
balance pointed to by `to`.

On a successful transfer, `*from` decreases by `amount` and `*to`
increases by `amount`. The total balance (`*from + *to`) is
preserved.

The function must not panic for any inputs. When the transfer
would underflow `*from` (i.e. `amount > *from`) or overflow `*to`
(i.e. `*to + amount > u64::MAX`), the function must leave both
balances unchanged — this is the *rejection* branch of the
contract, and a correct implementation must exercise it without
panicking.
