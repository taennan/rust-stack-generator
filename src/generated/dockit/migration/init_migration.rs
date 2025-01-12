use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};
use sea_query::extension::postgres::Type;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        //
        // Tables
        //


        manager
            .create_table(
                Table::create()
                    .table(AdminApiKey::Table)
                    .if_not_exists()
					.col(primary_key_col(AdminApiKey::Id))
					.col(string(AdminApiKey::Hash))
					.col(timestamp_col(AdminApiKey::Created))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(Occupant::Table)
                    .if_not_exists()
					.col(primary_key_col(Occupant::Id))
					.col(uuid(Occupant::OrgId))
					.col(string(Occupant::Name))
					.col(string(Occupant::Description))
					.col(timestamp_col(Occupant::Created))
					.col(timestamp_col(Occupant::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(Org::Table)
                    .if_not_exists()
					.col(primary_key_col(Org::Id))
					.col(string(Org::Name))
					.col(string(Org::Description))
					.col(timestamp_col(Org::Created))
					.col(timestamp_col(Org::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(OrgUser::Table)
                    .if_not_exists()
					.col(primary_key_col(OrgUser::Id))
					.col(uuid(OrgUser::OrgId))
					.col(uuid(OrgUser::ClawAuthId))
					.col(string(OrgUser::Email))
					.col(string(OrgUser::Description))
					.col(timestamp_col(OrgUser::Created))
					.col(timestamp_col(OrgUser::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(Reservation::Table)
                    .if_not_exists()
					.col(primary_key_col(Reservation::Id))
					.col(uuid(Reservation::OrgId))
					.col(uuid(Reservation::ReserveeId))
					.col(string(Reservation::Status))
					.col(string(Reservation::Description))
					.col(timestamp_col(Reservation::Created))
					.col(timestamp_col(Reservation::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(Reservee::Table)
                    .if_not_exists()
					.col(primary_key_col(Reservee::Id))
					.col(uuid(Reservee::OrgId))
					.col(uuid(Reservee::ReserveeUserId))
					.col(string(Reservee::FirstName))
					.col(string(Reservee::MiddleNames))
					.col(string(Reservee::LastName))
					.col(string(Reservee::Email))
					.col(string(Reservee::Phone))
					.col(string(Reservee::Description))
					.col(timestamp_col(Reservee::Created))
					.col(timestamp_col(Reservee::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(ReserveeUser::Table)
                    .if_not_exists()
					.col(primary_key_col(ReserveeUser::Id))
					.col(uuid(ReserveeUser::ClawAuthId))
					.col(string(ReserveeUser::Email))
					.col(string(ReserveeUser::Description))
					.col(timestamp_col(ReserveeUser::Created))
					.col(timestamp_col(ReserveeUser::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(Resource::Table)
                    .if_not_exists()
					.col(primary_key_col(Resource::Id))
					.col(uuid(Resource::OrgId))
					.col(string(Resource::Name))
					.col(string(Resource::Description))
					.col(timestamp_col(Resource::Created))
					.col(timestamp_col(Resource::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(ResourceReservation::Table)
                    .if_not_exists()
					.col(primary_key_col(ResourceReservation::Id))
					.col(uuid(ResourceReservation::OrgId))
					.col(uuid(ResourceReservation::ResourceId))
					.col(uuid(ResourceReservation::ReservationId))
					.col(string(ResourceReservation::Description))
					.col(timestamp_col(ResourceReservation::Created))
					.col(timestamp_col(ResourceReservation::Updated))
                    .to_owned(),
            )
            .await?;
    
        manager
            .create_table(
                Table::create()
                    .table(ResourceReservationOccupant::Table)
                    .if_not_exists()
					.col(primary_key_col(ResourceReservationOccupant::Id))
					.col(uuid(ResourceReservationOccupant::OrgId))
					.col(uuid(ResourceReservationOccupant::ResourceReservationId))
					.col(uuid(ResourceReservationOccupant::OccupantId))
					.col(timestamp_col(ResourceReservationOccupant::Created))
					.col(timestamp_col(ResourceReservationOccupant::Updated))
                    .to_owned(),
            )
            .await?;
    
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
					.table(AdminApiKey::Table)
					.table(Occupant::Table)
					.table(Org::Table)
					.table(OrgUser::Table)
					.table(Reservation::Table)
					.table(Reservee::Table)
					.table(ReserveeUser::Table)
					.table(Resource::Table)
					.table(ResourceReservation::Table)
					.table(ResourceReservationOccupant::Table)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}


#[derive(DeriveIden)]
enum AdminApiKey {
    Table,
	Id,
	Hash,
	Created,
}

#[derive(DeriveIden)]
enum Occupant {
    Table,
	Id,
	OrgId,
	Name,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum Org {
    Table,
	Id,
	Name,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum OrgUser {
    Table,
	Id,
	OrgId,
	ClawAuthId,
	Email,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum Reservation {
    Table,
	Id,
	OrgId,
	ReserveeId,
	Status,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum Reservee {
    Table,
	Id,
	OrgId,
	ReserveeUserId,
	FirstName,
	MiddleNames,
	LastName,
	Email,
	Phone,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum ReserveeUser {
    Table,
	Id,
	ClawAuthId,
	Email,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum Resource {
    Table,
	Id,
	OrgId,
	Name,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum ResourceReservation {
    Table,
	Id,
	OrgId,
	ResourceId,
	ReservationId,
	Description,
	Created,
	Updated,
}

#[derive(DeriveIden)]
enum ResourceReservationOccupant {
    Table,
	Id,
	OrgId,
	ResourceReservationId,
	OccupantId,
	Created,
	Updated,
}


fn primary_key_col<T>(col: T) -> ColumnDef
where
    T: IntoIden,
{
    pk_uuid(col)
        .default(Expr::custom_keyword(Alias::new("uuid_generate_v4()")))
        .unique_key()
        .to_owned()
}

fn timestamp_col<T>(col: T) -> ColumnDef
where
    T: IntoIden,
{
    date_time(col).default(Expr::current_timestamp()).to_owned()
}
