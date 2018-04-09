//! Types used to convey transition related information.

use std::convert::TryFrom;

use medici_core::error::RuntimeConstraintError;
use medici_core::marker::{TransactionContainer, TransactionMarker};

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
    /// See [`PrintTransaction`]
    Print(PrintTransaction),
}

impl TransactionContainer for TransactionItem {}

impl From<Epsilon> for TransactionItem {
    fn from(x: Epsilon) -> Self {
        TransactionItem::Epsilon(x)
    }
}

impl TryFrom<TransactionItem> for Epsilon {
    type Error = RuntimeConstraintError;

    fn try_from(tc: TransactionItem) -> Result<Self, Self::Error> {
        match tc {
            TransactionItem::Epsilon(x) => Ok(x),
            e @ _ => {
                let expected = stringify!(TransactionItem::Epsilon);
                let factual = format!("{:?}", e);
                Err((expected, factual).into())
            }
        }
    }
}

/// Transaction to be received by states with printing behaviour.
///
/// This state is pure exemplary, I don't know what else to tell you
/// about it..
#[derive(Debug, Clone, Copy)]
pub struct PrintTransaction(pub &'static str);
impl TransactionMarker for PrintTransaction {}

impl From<PrintTransaction> for TransactionItem {
    fn from(x: PrintTransaction) -> Self {
        TransactionItem::Print(x)
    }
}

impl TryFrom<TransactionItem> for PrintTransaction {
    type Error = RuntimeConstraintError;

    fn try_from(tc: TransactionItem) -> Result<Self, Self::Error> {
        match tc {
            TransactionItem::Print(x) => Ok(x),
            e @ _ => {
                let expected = stringify!(TransactionItem::Print);
                let factual = format!("{:?}", e);
                Err((expected, factual).into())
            }
        }
    }
}
