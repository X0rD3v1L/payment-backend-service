use sea_orm_migration::prelude::*;

use super::m20250521_135328_create_users_table::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Account::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Account::AccountId)
                            .string()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("concat('acc-', gen_random_uuid()::text)")),
                    )
                    .col(ColumnDef::new(Account::UserId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-account-user_id")
                            .from(Account::Table, Account::UserId)
                            .to(Users::Table, Users::UserId),
                    )
                    .col(ColumnDef::new(Account::CurrencyCode).string().not_null())
                    .col(ColumnDef::new(Account::Balance).float().not_null())
                    .col(ColumnDef::new(Account::LockedBalance).float().not_null())
                    .col(
                        ColumnDef::new(Account::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Account::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Account {
    Table,
    AccountId,
    UserId,
    CurrencyCode,
    Balance,
    LockedBalance,
    UpdatedAt,
}
