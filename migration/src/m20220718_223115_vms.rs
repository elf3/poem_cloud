use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Vms::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Vms::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Vms::AccountId).big_integer().not_null())
                    .col(ColumnDef::new(Vms::SourceId).string().not_null())
                    .col(ColumnDef::new(Vms::Name).string().not_null())
                    .col(ColumnDef::new(Vms::Status).tiny_integer().not_null())
                    .col(ColumnDef::new(Vms::Group).string().not_null())
                    .col(ColumnDef::new(Vms::Area).string().not_null())
                    .col(ColumnDef::new(Vms::Spec).string().not_null())
                    .col(ColumnDef::new(Vms::Ip).string().not_null())
                    .col(ColumnDef::new(Vms::System).string().not_null())
                    .col(ColumnDef::new(Vms::Disk).string().not_null())
                    .col(ColumnDef::new(Vms::Remarks).string().not_null())
                    .col(ColumnDef::new(Vms::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Vms::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Vms {
    Table,
    Id,
    AccountId,
    SourceId,
    Name,
    Status,
    Group,
    Area,
    Spec,
    Ip,
    System,
    Disk,
    Remarks,
    CreatedAt,
}
