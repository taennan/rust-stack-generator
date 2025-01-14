#[cfg(feature = "async_graphql")]
use async_graphql::InputObject;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "async_graphql", derive(InputObject))]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PaginationInput {
    pub page: u64,
    pub take: u64,
}
