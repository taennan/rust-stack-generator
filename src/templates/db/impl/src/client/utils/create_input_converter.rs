use crate::{{entity_lower}::entity::ActiveModel, utils::input_converters::create_input_converter};
use {project_lower}_db_models::{entity_lower}::Create{entity}Input;

create_input_converter!(
    Create{entity}InputConverter,
    Create{entity}Input => ActiveModel,
    {
        {mapped_fields}
    }
);
