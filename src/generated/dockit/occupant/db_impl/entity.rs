use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "occupant")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	org_id: Uuid,
	name: String,
	description: String,
	created: DateTime,
	updated: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::org::entity::Entity",
        from = "Column::OrgId",
        to = "crate::org::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Org,
}

impl Related<crate::org::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
