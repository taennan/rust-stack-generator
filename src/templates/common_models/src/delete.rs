#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;

#[cfg_attr(feature = "async_graphql", derive(InputObject))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DeleteOutput {
    pub amount_deleted: usize,
}

impl From<usize> for DeleteOutput {
    fn from(amount_deleted: usize) -> Self {
        Self { amount_deleted }
    }
}
