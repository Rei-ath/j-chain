use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::{env, error::Error};

/// Initializes the MongoDB client and performs basic database operations.
///
/// This function initializes the MongoDB client using the connection URI specified
/// in the `MONGODB_URI` environment variable. It then prints the names of all databases
/// available in the MongoDB instance.
///
/// # Errors
///
/// Returns a boxed `dyn Error` trait object if any error occurs during database initialization.
///
/// # Examples
///
/// ```
/// use std::error::Error;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn Error>> {
///     db_init().await?;
///     Ok(())
/// }
/// ```
pub async fn db_init() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Retrieve MongoDB URI from environment variable
    let client_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
        panic!("Please set the MONGODB_URI environment variable");
    });

    // Print MongoDB URI for debugging
    println!("MongoDB URI: {}", client_uri);

    // Parse client options with Cloudflare resolver configuration
    let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;

    // Initialize MongoDB client
    let client = Client::with_options(options)?;

    // Print available databases
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    Ok(())
}
