use space_traders_api::apis::{agents_api, configuration::Configuration, global_api};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Default configuration with no authorization.
    let mut config = Configuration::new();
    let mut is_auth = false;

    // Optionally use the environment variable SPACE_TRADERS_AGENT_TOKEN.
    if let Ok(token) = std::env::var("SPACE_TRADERS_AGENT_TOKEN") {
        config.bearer_access_token = Some(token);
        is_auth = true;
        eprintln!("Using token from SPACE_TRADERS_AGENT_TOKEN");
    } else {
        eprintln!("No token found; set SPACE_TRADERS_AGENT_TOKEN to get more details");
    }

    // Fetch server status.
    eprintln!();
    let status = global_api::get_status(&config).await?;
    println!("Status: {}", status.status);

    if is_auth {
        // Fetch agent status.
        let agent = agents_api::get_my_agent(&config).await?;
        println!("Agent: {}", agent.data.symbol);
    }

    Ok(())
}
