use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "reservee_user")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	claw_auth_id: Uuid,
	email: String,
	description: String,
	created: DateTime,
	updated: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::claw_auth::entity::Entity",
        from = "Column::ClawAuthId",
        to = "crate::claw_auth::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ClawAuth,
}

impl Related<crate::claw_auth::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ClawAuth.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
