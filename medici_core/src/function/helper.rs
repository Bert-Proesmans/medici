//! Core functionality helper methods.
//!
//! Expect to find small utilities here, but they are mostly used by the hidden parts of the core.
use std::convert::TryInto;

use marker::{Transaction, TransactionContainer};

/* Transaction helpers */
/// Transform a transaction into the wrapping variant.
pub fn pack_transaction<T, TC>(x: T) -> TC
where
    T: Transaction + Into<TC> + 'static,
    TC: TransactionContainer + 'static,
{
    x.into()
}

/// Unpack a wrapped transaction into an owned value.
///
/// It's of course necessary to
pub fn unpack_transaction<T, TC>(tc: TC) -> Result<T, TC::Error>
where
    T: Transaction + 'static,
    TC: TransactionContainer + TryInto<T> + 'static,
{
    tc.try_into()
}
