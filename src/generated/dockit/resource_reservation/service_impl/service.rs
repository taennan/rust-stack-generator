use crate::{
    resource_reservation::utils::{
        CreateResourceReservationInputConverter, SearchManyResourceReservationsInputConverter,
        SearchResourceReservationInputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use dockit_db_interface::ResourceReservationDB;
use dockit_error::DockitResult;
pub use dockit_services_interface::resource_reservation::ResourceReservationService as ResourceReservationServiceTrait;
use dockit_services_interface::{
    common::DeleteOutput,
    resource_reservation::{
        CreateResourceReservationInput, ResourceReservation, SearchManyResourceReservationsInput, SearchResourceReservationInput,
        UpdateResourceReservationInput,
    },
};
use uuid::Uuid;

pub struct ResourceReservationService<D> {
    resource_reservation_db: D,
    org_id: Uuid,
}

impl<D> ResourceReservationService<D> {
    pub fn new(resource_reservation_db: D, org_id: Uuid) -> Self {
        Self {
            resource_reservation_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> ResourceReservationServiceTrait for ResourceReservationService<D>
where
    D: ResourceReservationDB + Sync,
{
    async fn create(&self, input: CreateResourceReservationInput) -> DockitResult<ResourceReservation> {
        service_impl::simple_create_impl!(self, resource_reservation_db, input, CreateResourceReservationInputConverter)
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> DockitResult<ResourceReservation> {
        service_impl::simple_get_by_id_impl!(self, id, SearchResourceReservationInput)
    }

    async fn get_one(&self, input: SearchResourceReservationInput) -> DockitResult<Option<ResourceReservation>> {
        service_impl::simple_get_one_impl!(self, resource_reservation_db, input, SearchResourceReservationInputConverter)
    }

    async fn get_many(&self, input: SearchManyResourceReservationsInput) -> DockitResult<Vec<ResourceReservation>> {
        service_impl::simple_get_many_impl!(
            self,
            resource_reservation_db,
            input,
            SearchManyResourceReservationsInputConverter
        )
    }

    async fn update(&self, input: UpdateResourceReservationInput) -> DockitResult<ResourceReservation> {
        service_impl::simple_update_impl!(self, resource_reservation_db, input)
    }

    async fn delete_by_id(&self, id: Uuid) -> DockitResult<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, resource_reservation_db, id)
    }
}
