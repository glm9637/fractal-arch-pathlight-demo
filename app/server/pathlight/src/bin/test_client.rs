use todo_api_client::v1::{
    request::GetTodoListRequest, service::todo_service_client::TodoServiceClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⏳ Connecting to Todo Backend...");

    // Connect to the server we just started
    let mut client = TodoServiceClient::connect("http://127.0.0.1:50051").await?;

    println!("✅ Connected! Sending request...");

    // Construct the generated request (leaving pagination empty)
    let request = tonic::Request::new(GetTodoListRequest { pagination: None });

    // Fire the request and await the response
    let response = client.get_todo_list(request).await?;

    println!("🎉 Success! Received response:");
    println!("{:#?}", response.into_inner());

    Ok(())
}
