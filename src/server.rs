pub mod sync {
    tonic::include_proto!("syncd");
}

mod db;

use std::time::{SystemTime, UNIX_EPOCH};
use sync::sync_service_server::{SyncService, SyncServiceServer};
use sync::{
    AddItemRequest, CompleteItemRequest, DeleteItemRequest, DeleteItemResponse, Item,
    ListItemsRequest, ListItemsResponse,
};

use tonic::{transport::Server, Request, Response, Status};

pub struct MyService {
    pool: sqlx::SqlitePool,
}

//gives current time
fn now_unix() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tonic::async_trait]
impl SyncService for MyService {
    async fn add_item(&self, req: Request<AddItemRequest>) -> Result<Response<Item>, Status> {
        let title = req.into_inner().title;

        let item = Item {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            done: false,
            created_at: now_unix(),
        };

        db::insert_item(
            &self.pool,
            &item.id,
            &item.title,
            item.done,
            item.created_at,
        )
        .await
        .map_err(|error| Status::internal(error.to_string()))?;

        Ok(Response::new(item))
    }

    async fn list_items(
        &self,
        _req: Request<ListItemsRequest>,
    ) -> Result<Response<ListItemsResponse>, Status> {
        let rows = db::get_items(&self.pool)
            .await
            .map_err(|error| Status::internal(error.to_string()))?;

        let items = rows
            .into_iter()
            .map(|(id, title, done, created_at)| Item {
                id,
                title,
                done,
                created_at,
            })
            .collect();

        Ok(Response::new(ListItemsResponse { items }))
    }

    async fn complete_item(
        &self,
        req: Request<CompleteItemRequest>,
    ) -> Result<Response<Item>, Status> {
        let id = req.into_inner().id;

        db::complete_item(&self.pool, &id)
            .await
            .map_err(|error| Status::internal(error.to_string()))?;

        let item = Item {
            id,
            title: String::new(),
            done: true,
            created_at: 0,
        };

        Ok(Response::new(item))
    }

    async fn delete_item(
        &self,
        req: Request<DeleteItemRequest>,
    ) -> Result<Response<DeleteItemResponse>, Status> {
        let id = req.into_inner().id;

        let success = db::delete_item(&self.pool, &id)
            .await
            .map_err(|error| Status::internal(error.to_string()))?;

        Ok(Response::new(DeleteItemResponse { success }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::SqlitePool::connect("sqlite:./syncd.db").await?;

    db::init_table(&pool).await?;

    let service = MyService { pool: pool.clone() };

    let addr = "127.0.0.1:50051".parse()?;

    println!("syncd server listening on {}", addr);

    Server::builder()
        .add_service(SyncServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
