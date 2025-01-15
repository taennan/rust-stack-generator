use crate::{{entity_lower}::entity::Model, utils::input_converters::struct_from_mapper};
use {project_lower}_db_interface::{entity_lower}::{entity};

struct_from_mapper!(
    Model => {entity},
    {
        {model_fields}
    }
);
