use tonic::Request;

mod proto {
    tonic::include_proto!("syncd");
}

use proto::sync_service_client::SyncServiceClient;
use proto::AddItemRequest;
use proto::ListItemsRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SyncServiceClient::connect("http://127.0.0.1:50051").await?;

    let add_response = client
        .add_item(Request::new(AddItemRequest {
            title: "i like to learn rust".to_string(),
        }))
        .await?;

    println!("Added: {:?}", add_response.into_inner());

    let list_response = client.list_items(Request::new(ListItemsRequest {})).await?;

    //take out the value out of the tonic::Response wrapper
    println!("{:?}", list_response.into_inner());

    Ok(())
}
