use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "org")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	name: String,
	description: String,
	created: DateTime,
	updated: DateTime,
}





impl ActiveModelBehavior for ActiveModel {}
