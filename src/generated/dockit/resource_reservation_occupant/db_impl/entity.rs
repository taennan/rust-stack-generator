use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "resource_reservation_occupant")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	org_id: Uuid,
	resource_reservation_id: Uuid,
	occupant_id: Uuid,
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
        belongs_to = "crate::resource_reservation::entity::Entity",
        from = "Column::ResourceReservationId",
        to = "crate::resource_reservation::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ResourceReservation,    #[sea_orm(
        belongs_to = "crate::occupant::entity::Entity",
        from = "Column::OccupantId",
        to = "crate::occupant::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Occupant,
}

impl Related<crate::org::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}impl Related<crate::resource_reservation::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ResourceReservation.def()
    }
}impl Related<crate::occupant::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Occupant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
