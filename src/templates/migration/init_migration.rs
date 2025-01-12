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

{table_create_calls}
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
{table_drop_methods}
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

{entity_enums}

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
