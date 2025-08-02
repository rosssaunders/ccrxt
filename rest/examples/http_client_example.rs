//! Example demonstrating how to use the HTTP client abstraction

use rest::{HttpClient, Method, NativeHttpClient, RequestBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create HTTP client
    let http_client = NativeHttpClient::new()?;

    // Example 1: Simple GET request
    let request = RequestBuilder::new(Method::Get, "https://api.github.com/users/rust-lang")
        .header("User-Agent", "rest-example/0.1.0")
        .build();

    let response = http_client.execute(request).await?;

    println!("Status: {}", response.status);
    println!("Response: {}", response.text()?);

    // Example 2: GET request with query parameters
    #[derive(serde::Serialize)]
    struct QueryParams {
        page: u32,
        per_page: u32,
    }

    let params = QueryParams {
        page: 1,
        per_page: 10,
    };

    let request = RequestBuilder::new(
        Method::Get,
        "https://api.github.com/repos/rust-lang/rust/issues",
    )
    .header("User-Agent", "rest-example/0.1.0")
    .query(&params)?
    .build();

    let response = http_client.execute(request).await?;

    if response.is_success() {
        // Parse as JSON
        #[derive(serde::Deserialize, Debug)]
        struct Issue {
            number: u32,
            title: String,
        }

        let issues: Vec<Issue> = response.json()?;
        println!("Found {} issues", issues.len());
        for issue in issues.iter().take(5) {
            println!("Issue #{}: {}", issue.number, issue.title);
        }
    }

    // Example 3: POST request with JSON body
    #[derive(serde::Serialize)]
    struct CreateIssue {
        title: String,
        body: String,
    }

    let new_issue = CreateIssue {
        title: "Test issue".to_string(),
        body: "This is a test issue created by the example".to_string(),
    };

    let _request = RequestBuilder::new(
        Method::Post,
        "https://api.github.com/repos/owner/repo/issues",
    )
    .header("User-Agent", "rest-example/0.1.0")
    .header("Authorization", "token YOUR_GITHUB_TOKEN")
    .json(&new_issue)?
    .build();

    // Don't actually execute this request as it would require authentication
    println!("\nWould send POST request to create issue (not executed)");

    Ok(())
}
