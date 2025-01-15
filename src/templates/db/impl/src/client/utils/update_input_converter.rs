use crate::{{entity_lower}::entity::ActiveModel, utils::input_converters::update_input_converter};
use {project_lower}_db_interface::{entity_lower}::Update{entity}Input;

update_input_converter!(
    Update{entity}InputConverter,
    Update{entity}Input => ActiveModel,
    {
        {mapped_option_fields}
    },
    {
        {mapped_maybe_fields}
    }
);
