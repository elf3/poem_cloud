pub use sea_orm_migration::prelude::*;

mod m20220101_000001_user;
mod m20220718_223025_menu;
mod m20220718_223038_sys_config;
mod m20220718_223054_cloud_account;
mod m20220718_223109_vm_types;
mod m20220718_223115_vms;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_user::Migration),
            Box::new(m20220718_223025_menu::Migration),
            Box::new(m20220718_223038_sys_config::Migration),
            Box::new(m20220718_223054_cloud_account::Migration),
            Box::new(m20220718_223109_vm_types::Migration),
            Box::new(m20220718_223115_vms::Migration),
        ]
    }
}
