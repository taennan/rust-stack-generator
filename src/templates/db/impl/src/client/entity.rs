use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "{entity_lower}")]
pub struct Model {
{model_fields}
}

{relation_enum}

{related_impls}

impl ActiveModelBehavior for ActiveModel {}
