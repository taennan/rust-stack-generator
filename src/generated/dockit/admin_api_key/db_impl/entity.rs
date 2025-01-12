use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "admin_api_key")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	hash: String,
	created: DateTime,
}





impl ActiveModelBehavior for ActiveModel {}
