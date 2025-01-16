use crate::{
    {entity_lower}::utils::{
        {entity}Converter, Update{entity}InputConverter,
        Create{entity}InputConverter, SearchMany{entity}sInputConverter,
        Search{entity}InputConverter,
    },
    utils::service_impl,
};
use async_trait::async_trait;
use {project_lower}_db_interface::{entity_lower}::{entity}DB;
use {project_lower}_error::{project_prefix}Result;
pub use {project_lower}_services_interface::{entity_lower}::{entity}Service as {entity}ServiceTrait;
use {project_lower}_common_models::{delete::DeleteOutput, search::CountOutput};
use {project_lower}_services_interface::{entity_lower}::{
    Create{entity}Input, {entity}, SearchMany{entity}sInput, Search{entity}Input,
    Update{entity}Input,
};
use uuid::Uuid;

pub struct {entity}Service<D> {
    {entity_lower}_db: D,
    org_id: Uuid,
}

impl<D> {entity}Service<D> {
    pub fn new({entity_lower}_db: D, org_id: Uuid) -> Self {
        Self {
            {entity_lower}_db,
            org_id,
        }
    }
}

#[async_trait]
impl<D> {entity}ServiceTrait for {entity}Service<D>
where
    D: {entity}DB + Sync,
{
    async fn create(&self, input: Create{entity}Input) -> {project_prefix}Result<{entity}> {
        service_impl::simple_create_impl!(
            self, 
            {entity_lower}_db, 
            input, 
            {entity}Converter, 
            Create{entity}InputConverter
        )
    }

    async fn get_by_id_or_throw(&self, id: Uuid) -> {project_prefix}Result<{entity}> {
        service_impl::simple_get_by_id_impl!(self, id, {entity}Converter, Search{entity}Input)
    }

    async fn get_one(&self, input: Search{entity}Input) -> {project_prefix}Result<Option<{entity}>> {
        service_impl::simple_get_one_impl!(
            self, 
            {entity_lower}_db, 
            input, 
            {entity}Converter, 
            Search{entity}InputConverter
        )
    }

    async fn get_many(&self, input: SearchMany{entity}sInput) -> {project_prefix}Result<Vec<{entity}>> {
        service_impl::simple_get_many_impl!(
            self,
            {entity_lower}_db,
            input,
            {entity}Converter,
            SearchMany{entity}sInputConverter
        )
    }

    async fn count(&self, input: Search{entity}Input) -> {project_prefix}Result<CountOutput> {
        service_impl::simple_count_impl!(
            self, 
            {entity_lower}_db, 
            input,
            Search{entity}InputConverter
        )
    }

    async fn update(&self, input: Update{entity}Input) -> {project_prefix}Result<{entity}> {
        service_impl::simple_update_impl!(self, {entity_lower}_db, input, {entity}Converter, Update{entity}InputConverter)
    }

    async fn delete_by_id(&self, id: Uuid) -> {project_prefix}Result<DeleteOutput> {
        service_impl::simple_delete_by_id_impl!(self, {entity_lower}_db, id)
    }
}
