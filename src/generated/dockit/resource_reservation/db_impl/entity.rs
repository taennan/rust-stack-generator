use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "resource_reservation")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	org_id: Uuid,
	resource_id: Uuid,
	reservation_id: Uuid,
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
        belongs_to = "crate::resource::entity::Entity",
        from = "Column::ResourceId",
        to = "crate::resource::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Resource,    #[sea_orm(
        belongs_to = "crate::reservation::entity::Entity",
        from = "Column::ReservationId",
        to = "crate::reservation::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Reservation,
}

impl Related<crate::org::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}impl Related<crate::resource::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Resource.def()
    }
}impl Related<crate::reservation::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reservation.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
