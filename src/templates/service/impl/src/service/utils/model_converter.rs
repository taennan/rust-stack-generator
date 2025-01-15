use crate::utils::input_converters::struct_mapper;
use {project_lower}_services_interface::{entity_lower} as service;
use {project_lower}_db_interface::{entity_lower} as db;

struct_mapper(
    {entity}Converter,
    service::{entity} => db::{entity},
    {
        {mapped_fields}
    }
);
