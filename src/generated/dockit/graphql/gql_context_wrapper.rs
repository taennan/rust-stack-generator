use async_graphql::{Context, ErrorExtensions, FieldResult, Lookahead};
use db_postgres::{
	admin_api_key::{AdminApiKeyDB, AdminApiKeyDBTrait},
	occupant::{OccupantDB, OccupantDBTrait},
	org::{OrgDB, OrgDBTrait},
	org_user::{OrgUserDB, OrgUserDBTrait},
	reservation::{ReservationDB, ReservationDBTrait},
	reservee::{ReserveeDB, ReserveeDBTrait},
	reservee_user::{ReserveeUserDB, ReserveeUserDBTrait},
	resource::{ResourceDB, ResourceDBTrait},
	resource_reservation::{ResourceReservationDB, ResourceReservationDBTrait},
	resource_reservation_occupant::{ResourceReservationOccupantDB, ResourceReservationOccupantDBTrait},};
use core_services::{
	admin_api_key::{AdminApiKeyService, AdminApiKeyServiceTrait},
	occupant::{OccupantService, OccupantServiceTrait},
	org::{OrgService, OrgServiceTrait},
	org_user::{OrgUserService, OrgUserServiceTrait},
	reservation::{ReservationService, ReservationServiceTrait},
	reservee::{ReserveeService, ReserveeServiceTrait},
	reservee_user::{ReserveeUserService, ReserveeUserServiceTrait},
	resource::{ResourceService, ResourceServiceTrait},
	resource_reservation::{ResourceReservationService, ResourceReservationServiceTrait},
	resource_reservation_occupant::{ResourceReservationOccupantService, ResourceReservationOccupantServiceTrait},};
use sea_orm::DatabaseConnection;

pub struct GQLContextWrapper<'a> {
    ctx: &'a Context<'a>,
}

impl<'a> From<&'a Context<'a>> for GQLContext<'a> {
    fn from(ctx: &'a Context<'a>) -> Self {
        Self { ctx }
    }
}

impl<'a> GQLContextWrapper<'a> {
    fn lookahead(&self) -> Lookahead<'_> {
        self.ctx.look_ahead()
    }

    fn db_connection(&self) -> DatabaseConnection {
        self.ctx.data_unchecked::<DatabaseConnection>().clone()
    }


    pub fn admin_api_key_service(&self) -> impl AdminApiKeyServiceTrait {
        AdminApiKeyService::new(self.admin_api_key_db())
    }

    pub fn occupant_service(&self) -> impl OccupantServiceTrait {
        OccupantService::new(self.occupant_db())
    }

    pub fn org_service(&self) -> impl OrgServiceTrait {
        OrgService::new(self.org_db())
    }

    pub fn org_user_service(&self) -> impl OrgUserServiceTrait {
        OrgUserService::new(self.org_user_db())
    }

    pub fn reservation_service(&self) -> impl ReservationServiceTrait {
        ReservationService::new(self.reservation_db())
    }

    pub fn reservee_service(&self) -> impl ReserveeServiceTrait {
        ReserveeService::new(self.reservee_db())
    }

    pub fn reservee_user_service(&self) -> impl ReserveeUserServiceTrait {
        ReserveeUserService::new(self.reservee_user_db())
    }

    pub fn resource_service(&self) -> impl ResourceServiceTrait {
        ResourceService::new(self.resource_db())
    }

    pub fn resource_reservation_service(&self) -> impl ResourceReservationServiceTrait {
        ResourceReservationService::new(self.resource_reservation_db())
    }

    pub fn resource_reservation_occupant_service(&self) -> impl ResourceReservationOccupantServiceTrait {
        ResourceReservationOccupantService::new(self.resource_reservation_occupant_db())
    }

    fn admin_api_key_db(&self) -> impl AdminApiKeyDBTrait {
        AdminApiKeyDB::new(self.db_connection())
    }

    fn occupant_db(&self) -> impl OccupantDBTrait {
        OccupantDB::new(self.db_connection())
    }

    fn org_db(&self) -> impl OrgDBTrait {
        OrgDB::new(self.db_connection())
    }

    fn org_user_db(&self) -> impl OrgUserDBTrait {
        OrgUserDB::new(self.db_connection())
    }

    fn reservation_db(&self) -> impl ReservationDBTrait {
        ReservationDB::new(self.db_connection())
    }

    fn reservee_db(&self) -> impl ReserveeDBTrait {
        ReserveeDB::new(self.db_connection())
    }

    fn reservee_user_db(&self) -> impl ReserveeUserDBTrait {
        ReserveeUserDB::new(self.db_connection())
    }

    fn resource_db(&self) -> impl ResourceDBTrait {
        ResourceDB::new(self.db_connection())
    }

    fn resource_reservation_db(&self) -> impl ResourceReservationDBTrait {
        ResourceReservationDB::new(self.db_connection())
    }

    fn resource_reservation_occupant_db(&self) -> impl ResourceReservationOccupantDBTrait {
        ResourceReservationOccupantDB::new(self.db_connection())
    }

}
