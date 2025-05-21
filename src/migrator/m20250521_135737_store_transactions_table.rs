use super::m20250521_135711_create_accounts_table::Account;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Txns::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Txns::TxnId)
                            .string()
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("concat('tx-', gen_random_uuid()::text)")),
                    )
                    .col(ColumnDef::new(Txns::AccountId).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-txns-account_id")
                            .from(Txns::Table, Txns::AccountId)
                            .to(Account::Table, Account::AccountId),
                    )
                    .col(ColumnDef::new(Txns::Amount).float().not_null())
                    .col(ColumnDef::new(Txns::CurrencyCode).string().not_null())
                    .col(ColumnDef::new(Txns::TxnType).string().not_null())
                    .col(
                        ColumnDef::new(Txns::Status)
                            .string()
                            .not_null()
                            .default("pending"), // Default value for txn status
                    )
                    .col(
                        ColumnDef::new(Txns::CreatedAt)
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
            .drop_table(Table::drop().table(Txns::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Txns {
    Table,
    TxnId,
    AccountId,
    Amount,
    CurrencyCode,
    TxnType,
    Status,
    CreatedAt,
}
