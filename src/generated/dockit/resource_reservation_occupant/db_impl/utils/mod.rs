pub(crate) mod model_converter;
pub(crate) mod create_input_converter;
pub(crate) mod search_input_converter;
pub(crate) mod update_input_converter;

pub(crate) use create_input_converter::{CreateResourceReservationOccupantInputConverter};
pub(crate) use search_input_converter::{SearchResourceReservationOccupantInputConverter};
pub(crate) use update_input_converter::{UpdateResourceReservationOccupantInputConverter};
