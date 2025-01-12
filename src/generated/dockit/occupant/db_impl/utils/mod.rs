pub(crate) mod model_converter;
pub(crate) mod create_input_converter;
pub(crate) mod search_input_converter;
pub(crate) mod update_input_converter;

pub(crate) use create_input_converter::{CreateOccupantInputConverter};
pub(crate) use search_input_converter::{SearchOccupantInputConverter};
pub(crate) use update_input_converter::{UpdateOccupantInputConverter};
