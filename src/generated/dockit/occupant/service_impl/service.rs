use crate::{
    occupant::utils::{
        CreateOccupantInputConverter, SearchManyOccupantsInputConverter,
        SearchOccupantInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::OccupantDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::occupant::OccupantService as OccupantServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    occupant::{
        CreateOccupantInput, Occupant, SearchManyOccupantsInput, SearchOccupantInput,
        UpdateOccupantInput,
    },
};
use uuid::Uuid;

pub struct OccupantService<D> {
    occupant_db: D,
    org_id: Uuid,
}

impl<D> OccupantService<D> {
    pub fn new(occupant_db: D, org_id: Uuid) -> Self {
        Self {
            occupant_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> OccupantServiceTrait for OccupantService<D>
where
    D: OccupantDB + Sync,
{
    async fn create(&self, input: CreateOccupantInput) -> DockitResult<Occupant> {
        service_impl::simple_create_impl!(self, occupant_db, input, CreateOccupantInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<Occupant> {
        service_impl::simple_get_by_id_impl!(self, id, SearchOccupantInput)
    }

    async fn get_one(&self, input: SearchOccupantInput) -> DockitResult<Option<Occupant>> {
        service_impl::simple_get_one_impl!(self, occupant_db, input, SearchOccupantInputConverter)
    }

    async fn get_many(&self, input: SearchManyOccupantsInput) -> DockitResult<Vec<Occupant>> {
        service_impl::simple_get_many_impl!(
            self,
            occupant_db,
            input,
            SearchManyOccupantsInputConverter
        )
    }

    async fn update(&self, input: UpdateOccupantInput) -> DockitResult<Occupant> {
        service_impl::simple_update_impl!(self, occupant_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, occupant_db, id)
    }
}
