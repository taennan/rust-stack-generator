use crate::{
    resource_reservation_occupant::utils::{
        CreateResourceReservationOccupantInputConverter, SearchManyResourceReservationOccupantsInputConverter,
        SearchResourceReservationOccupantInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::ResourceReservationOccupantDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::resource_reservation_occupant::ResourceReservationOccupantService as ResourceReservationOccupantServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    resource_reservation_occupant::{
        CreateResourceReservationOccupantInput, ResourceReservationOccupant, SearchManyResourceReservationOccupantsInput, SearchResourceReservationOccupantInput,
        UpdateResourceReservationOccupantInput,
    },
};
use uuid::Uuid;

pub struct ResourceReservationOccupantService<D> {
    resource_reservation_occupant_db: D,
    org_id: Uuid,
}

impl<D> ResourceReservationOccupantService<D> {
    pub fn new(resource_reservation_occupant_db: D, org_id: Uuid) -> Self {
        Self {
            resource_reservation_occupant_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> ResourceReservationOccupantServiceTrait for ResourceReservationOccupantService<D>
where
    D: ResourceReservationOccupantDB + Sync,
{
    async fn create(&self, input: CreateResourceReservationOccupantInput) -> DockitResult<ResourceReservationOccupant> {
        service_impl::simple_create_impl!(self, resource_reservation_occupant_db, input, CreateResourceReservationOccupantInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<ResourceReservationOccupant> {
        service_impl::simple_get_by_id_impl!(self, id, SearchResourceReservationOccupantInput)
    }

    async fn get_one(&self, input: SearchResourceReservationOccupantInput) -> DockitResult<Option<ResourceReservationOccupant>> {
        service_impl::simple_get_one_impl!(self, resource_reservation_occupant_db, input, SearchResourceReservationOccupantInputConverter)
    }

    async fn get_many(&self, input: SearchManyResourceReservationOccupantsInput) -> DockitResult<Vec<ResourceReservationOccupant>> {
        service_impl::simple_get_many_impl!(
            self,
            resource_reservation_occupant_db,
            input,
            SearchManyResourceReservationOccupantsInputConverter
        )
    }

    async fn update(&self, input: UpdateResourceReservationOccupantInput) -> DockitResult<ResourceReservationOccupant> {
        service_impl::simple_update_impl!(self, resource_reservation_occupant_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, resource_reservation_occupant_db, id)
    }
}
