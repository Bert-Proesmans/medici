//! Core functionality helper methods.
//!
//! Expect to find small utilities here, but they are mostly used by the hidden parts of the core.
use std::convert::TryInto;

use error::custom_type::TransactionUnpackError;
use marker;

/* Transaction helpers */
/// Transform a transaction into the wrapping variant.
pub fn pack_transaction<T, TC>(x: T) -> TC
where
    T: marker::Transaction + Into<TC> + 'static,
    TC: marker::TransactionContainer + 'static,
{
    x.into()
}

/// Unpack a wrapped transaction into an owned value.
///
/// It's of course necessary to
pub fn unpack_transaction<T, TC>(tc: TC) -> Result<T, TransactionUnpackError>
where
    T: marker::Transaction + 'static,
    TC: marker::TransactionContainer + TryInto<T, Error = TransactionUnpackError> + 'static,
{
    tc.try_into()
}
