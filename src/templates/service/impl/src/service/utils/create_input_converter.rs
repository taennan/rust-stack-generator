use crate::utils::input_converters::create_input_converter;
use {project_lower}_db_interface::{entity_lower} as db;
use {project_lower}_services_interface::{entity_lower} as service;

create_input_converter!(
    Create{entity}InputConverter,
    service::Create{entity}Input => db::Create{entity}Input,
    {
        {mapped_fields}
    }
);
