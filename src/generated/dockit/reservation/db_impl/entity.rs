use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "reservation")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	org_id: Uuid,
	reservee_id: Uuid,
	status: String,
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
    Org,    #[sea_orm(
        belongs_to = "crate::reservee::entity::Entity",
        from = "Column::ReserveeId",
        to = "crate::reservee::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Reservee,
}

impl Related<crate::org::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}impl Related<crate::reservee::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reservee.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
