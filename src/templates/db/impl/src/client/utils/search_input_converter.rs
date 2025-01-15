use crate::{
    {entity_lower}::entity::{Column, Entity},
    utils::input_converters::search_input_converter,
};
use {project_lower}_db_interface::occupant::Search{entity}Input;

search_input_converter!(
    Search{entity}InputConverter,
    Search{entity}Input => Entity,
    {
        {mapped_fields}
    }
);
