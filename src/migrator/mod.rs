pub use sea_orm_migration::prelude::*;

mod m20250521_135328_create_users_table;
mod m20250521_135711_create_accounts_table;
mod m20250521_135737_store_transactions_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250521_135328_create_users_table::Migration),
            Box::new(m20250521_135711_create_accounts_table::Migration),
            Box::new(m20250521_135737_store_transactions_table::Migration),
        ]
    }
}
