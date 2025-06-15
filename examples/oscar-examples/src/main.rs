mod configs;
mod db;
mod entity;
mod logger;

use crate::entity::prelude::WmSolTransaction;
use crate::entity::wm_sol_transaction;
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use axum::{debug_handler, routing, Router};
use sea_orm::sea_query::{Alias, Expr, Func};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    let db_conn = db::init().await?;
    let router = Router::new()
        .route("/", routing::get(say_hello))
        .route("/trans", routing::get(list_trans))
        .with_state(db_conn);

    let server_config = configs::get_app_config().server_config();
    let ip = server_config.get_server_ip();
    let port = server_config.get_server_port();
    let listener = TcpListener::bind(format!("{ip}:{port}")).await?;
    tracing::info!(
        "server is listening on: {:?}",
        listener.local_addr()?.to_string()
    );
    axum::serve(listener, router).await?;

    Ok(())
}

#[debug_handler]
async fn say_hello() -> &'static str {
    "Hello Axum!!!"
}

#[derive(Debug, serde::Serialize, sea_orm::FromQueryResult)]
#[serde(rename_all = "camelCase")]
struct TransSummary {
    mint: String,
    counter: Option<i64>,
    token_amount_sum: f64,
    sol_amount_sum: f64,
    buy_count: Option<i64>,
    sell_count: Option<i64>,
}
#[derive(Debug, serde::Serialize)]
struct PaginatedResponse<T> {
    data: Vec<T>,
    total: u64,
    page: u64,
    page_size: u64,
    total_pages: u64,
}

#[derive(Debug, serde::Deserialize)]
struct PaginationParams {
    page: Option<u64>,
    page_size: Option<u64>,
}

#[debug_handler]
async fn list_trans(
    State(db_conn): State<DatabaseConnection>,
    Query(params): Query<PaginationParams>,
) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(10);

    let paginator = WmSolTransaction::find()
        .select_only()
        .group_by(wm_sol_transaction::Column::Mint)
        .column(wm_sol_transaction::Column::Mint)
        .column_as(wm_sol_transaction::Column::Mint.count(), "counter")
        .column_as(
            wm_sol_transaction::Column::TokenAmount.sum(),
            "token_amount_sum",
        )
        .column_as(
            wm_sol_transaction::Column::SolAmount.sum(),
            "sol_amount_sum",
        )
        .expr_as(
            Func::sum(
                Expr::case(
                    Expr::col(wm_sol_transaction::Column::IsBuy).eq(true),
                    Expr::val(1),
                )
                .finally(Expr::val(0)),
            ),
            "buy_count",
        )
        .expr_as(
            Func::sum(
                Expr::case(
                    Expr::col(wm_sol_transaction::Column::IsBuy).eq(false),
                    Expr::val(1),
                )
                .finally(Expr::val(0)),
            ),
            "sell_count",
        )
        .filter(
            wm_sol_transaction::Column::TransferType.eq(0)
        )
        .order_by_desc(Expr::col(Alias::new("counter")))
        .into_model::<TransSummary>()
        .paginate(&db_conn, page_size);

    // axum::Json(trans)
    // 获取总页数
    let total_pages = paginator.num_pages().await.unwrap();

    // 获取总记录数
    let total = paginator.num_items().await.unwrap();

    // 获取当前页数据
    let data = paginator.fetch_page(page - 1).await.unwrap(); // 注意：页码从0开始

    let response = PaginatedResponse {
        data,
        total,
        page,
        page_size,
        total_pages,
    };

    axum::Json(response)
}
