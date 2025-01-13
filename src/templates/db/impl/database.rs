use crate::{
    occupant::{
        {entity_lowercase}::{ActiveModel, Column, Entity},
        utils::{
            Create{entity}InputConverter, Search{entity}InputConverter,
            Update{entity}InputConverter,
        },
    },
    utils::client_impl,
};
pub use {project_lower}_db_interface::{entity}DBTrait;
use {project_lower}_db_models::{entity_lowercase}::{
    Create{entity}Input, {entity}, SearchMany{entity}sInput, Search{entity}Input,
    Update{entity}Input,
};
use sea_orm::{entity::prelude::*, QueryOrder};

client_impl::simple_client! {
    {entity}DB,
    {entity}DBTrait,
    {entity},
    { Create{entity}Input, Create{entity}InputConverter },
    { Search{entity}Input, Search{entity}InputConverter },
    { SearchMany{entity}sInput },
    { Update{entity}Input, Update{entity}InputConverter },
}
