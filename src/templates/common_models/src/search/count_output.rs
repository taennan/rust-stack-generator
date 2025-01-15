#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;

#[cfg_attr(feature = "async_graphql", derive(InputObject))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CountOutput {
    pub count: usize,
}

impl From<usize> for CountOutput {
    fn from(count: usize) -> Self {
        Self { count }
    }
}

impl From<u64> for CountOutput {
    fn from(count: u64) -> Self {
        Self {
            count: count as usize,
        }
    }
}
