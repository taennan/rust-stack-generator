#![cfg(test)]
#![cfg(feature = "mock")]

use crate::{
    {entity_lowercase}::{
        database::{{entity}DB, {entity}DBTrait},
        entity::Model,
    },
    utils::test_utils::test_macros,
};
use {project_lower}_db_models::occupant::{
    Create{entity}Input, {entity}, Search{entity}Input, Update{entity}Input,
};

test_macros::create!({entity}DB, Model, {entity}, Create{entity}Input);
test_macros::get_one!({entity}DB, Model, {entity}, Search{entity}Input);
test_macros::update!({entity}DB, Model, {entity}, Update{entity}Input);
test_macros::delete_by_id!({entity}DB);
