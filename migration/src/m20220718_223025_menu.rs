use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Menu::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menu::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Menu::Name).string().not_null())
                    .col(ColumnDef::new(Menu::Path).string().not_null())
                    .col(ColumnDef::new(Menu::Method).string().not_null())
                    .col(ColumnDef::new(Menu::ParentId).big_integer().not_null())
                    .col(ColumnDef::new(Menu::Status).tiny_integer().not_null())
                    .col(ColumnDef::new(Menu::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Menu {
    Table,
    Id,
    Name,
    Path,
    Method,
    ParentId,
    Status,
    CreatedAt,
}
