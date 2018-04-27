//! Types used to convey transition related information.

use std::convert::TryFrom;

use medici_core::error::custom_type::TransactionUnpackError;
use medici_core::marker;

// Epsilon is re-exported here so implementers can do
// 'use transaction::*' and have both the medici_core transactions
// as well as newly defined transactions imported.
pub use medici_core::prefab::transaction::Epsilon;

/// Collection of known Transaction structures wrapped into a Sized
/// item.
#[derive(Debug, Clone)]
pub enum TransactionItem {
    /// See [`Epsilon`]
    Epsilon(Epsilon),
}

impl marker::TransactionContainer for TransactionItem {}

impl From<Epsilon> for TransactionItem {
    fn from(x: Epsilon) -> Self {
        TransactionItem::Epsilon(x)
    }
}

impl TryFrom<TransactionItem> for Epsilon {
    type Error = TransactionUnpackError;

    fn try_from(tc: TransactionItem) -> Result<Self, Self::Error> {
        match tc {
            TransactionItem::Epsilon(x) => Ok(x),
            /*
            e @ _ => {
                let expected = stringify!(TransactionItem::Epsilon);
                let factual = format!("{:?}", e);
                Err((expected, factual).into())
            }
            */
        }
    }
}
