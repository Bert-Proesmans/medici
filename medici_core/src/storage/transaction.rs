use error;
use function::StackStorageCompliance;
use marker;

#[derive(Debug, Clone)]
/// Object for storing [`Transaction`] objects.
///
/// This structure implements a stack contract to allow pushing
/// and popping items in FIFO order.
pub struct TransactionStorage<TTC>
where
    TTC: marker::TransactionContainer + Clone,
{
    transactions: Vec<TTC>,
}

impl<TTC> TransactionStorage<TTC>
where
    TTC: marker::TransactionContainer + Clone,
{
    /// Creates a new instance of for storage.
    pub fn new() -> Self {
        Self {
            transactions: vec![],
        }
    }
}

impl<TTC> StackStorageCompliance for TransactionStorage<TTC>
where
    TTC: marker::TransactionContainer + Clone,
{
    type Item = TTC;

    fn push<I: Into<Self::Item>>(&mut self, item: I) {
        self.transactions.push(item.into());
    }

    fn pop(&mut self) -> Option<Self::Item> {
        self.transactions.pop()
    }
}
