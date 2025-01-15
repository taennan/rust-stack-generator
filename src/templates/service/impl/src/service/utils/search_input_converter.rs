use crate::utils::input_converters::search_input_converter;
use {project_prefix}_db_interface::{entity_lower} as db;
use {project_prefix}_services_interface::{entity_lower} as service;

search_input_converter!(
    Search{entity}InputConverter,
    service::Search{entity}Input => db::Search{entity}Input,
    {
        {mapped_fields}
    }
);
