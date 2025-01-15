use crate::{
    {entity_lower}::utils::Search{entity}InputConverter,
    utils::input_converters::search_many_input_converter,
};
use {project_prefix}_db_interface::{entity_lower} as db;
use {project_prefix}_services_interface::{entity_lower} as service;

search_many_input_converter!(
    SearchMany{entity}sInputConverter,
    service::SearchMany{entity}sInput => db::SearchMany{entity}sInput,
    Search{entity}InputConverter,
);
