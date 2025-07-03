//! Simple test for KuCoin module access

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test basic kucoin module access
    println!("Testing basic kucoin access...");
    
    // Test public module access (we know this works)
    let _public_client = venues::kucoin::public::RestClient::new_default();
    println!("Public client created successfully");
    
    // NOW test if we can at least access the futures module
    // Let's create a futures client to see if the module loads
    println!("Attempting to create futures client...");
    
    let _futures_client = venues::kucoin::public::futures::RestClient::new_default();
    println!("Futures client created successfully!");
    
    Ok(())
}
