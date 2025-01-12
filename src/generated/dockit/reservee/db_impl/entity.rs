use sea_orm::{entity::prelude::*, prelude::DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveEntityModel, Clone, Debug, PartialEq)]
#[sea_orm(table_name = "reservee")]
pub struct Model {
	#[sea_orm(primary_key)]
	id: Uuid,
	org_id: Uuid,
	reservee_user_id: Option<Uuid>,
	first_name: Option<String>,
	middle_names: Option<String>,
	last_name: Option<String>,
	email: Option<String>,
	phone: Option<String>,
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
        belongs_to = "crate::reservee_user::entity::Entity",
        from = "Column::ReserveeUserId",
        to = "crate::reservee_user::entity::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ReserveeUser,
}

impl Related<crate::org::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Org.def()
    }
}impl Related<crate::reservee_user::entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReserveeUser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
