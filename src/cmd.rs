use console::{style, Emoji};
use reqwest::header::USER_AGENT;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
struct IsUpResponse {
    domain: String,
    port: u8,
    status_code: u8,
    response_ip: Option<String>,
    response_code: Option<u8>,
    response_time: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    aliases: HashMap<String, String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),
        }
    }
}

// get stored config in statconf.cfg or something similar and write out aliases
pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let cfg: Config = confy::load("staticli", None)?;

    if cfg.aliases.len() == 0 {
        println!("No saved aliases.");
        println!(
            "\nTo create a new alias, use the {} command.",
            style("add").cyan()
        );
        return Ok(());
    }

    for alias in cfg.aliases {
        println!(
            "Name: {}, Endpoint: {}",
            style(alias.0).cyan(),
            style(alias.1).cyan()
        )
    }
    Ok(())
}

// add an alias to the config in format [alias: string]: endpoint
pub fn add(alias: &String, endpoint: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg: Config = confy::load("staticli", None)?;
    println!("Adding alias: {} to configuration\n", style(alias).green());
    cfg.aliases.insert(alias.clone(), endpoint.clone());
    confy::store("staticli", None, cfg)?;
    println!("New alias successfully added to configuration.");

    Ok(())
}

pub fn update(alias: &String, endpoint: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg: Config = confy::load("staticli", None)?;

    if cfg.aliases.contains_key(alias) {
        println!(
            "Updating alias: {} to have endpoint value: {}\n",
            style(alias).green(),
            style(endpoint).green()
        );
        cfg.aliases
            .entry(alias.clone())
            .and_modify(|e| *e = endpoint.clone());
        confy::store("staticli", None, cfg)?;
        println!("Alias: {} successfully updated.", style(alias).green());
    } else {
        println!("Alias: {} not found.\n", style(alias).green());
        println!(
            "If you want to add this alias, use the {} command.",
            style("add").cyan()
        );
    }

    Ok(())
}

// delete an alias from the config
pub fn del(alias: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg: Config = confy::load("staticli", None)?;

    if cfg.aliases.contains_key(alias) {
        cfg.aliases.remove(alias);
        confy::store("staticli", None, cfg)?;
        println!("Alias: {} successfully removed.", style(alias).green());
    } else {
        println!("Alias: {} not found.\n", style(alias).green());
        println!(
            "If you want to add this alias, use the {} command.",
            style("add").cyan()
        );
    }

    Ok(())
}

// run a status check against the given alias' endpoint, print pretty to cli
pub fn check_saved(alias: &String) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = get_endpoint(alias).unwrap();

    if endpoint == None {
        println!("Alias: {} not found in saved config.", style(alias).green());
    } else {
        let _req = make_req(endpoint.unwrap());
    }

    Ok(())
}

// run a status check against the supplied endpoint
pub fn check_unsaved(endpoint: &String) -> Result<(), Box<dyn std::error::Error>> {
    let _req = make_req(endpoint.clone());

    Ok(())
}

// run a number of checks against a given endpoint, print each pretty to cli
// pub fn tail_saved(alias: &String, interval: &u8, count: &u8, out_file: &String) {}

// pub fn tail_unsaved(endpoint: &String, interval: &u8, count: &u8, out_file: &String) {}

// returns a saved endpoint from a given alias saved in statconf file
fn get_endpoint(alias: &String) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let cfg: Config = confy::load("staticli", None)?;

    if cfg.aliases.contains_key(alias) {
        let endpoint = cfg.aliases.get(alias).clone();
        return Ok(endpoint.cloned());
    } else {
        return Ok(None);
    }
}

// fn for making get request to isitup.org to check up status
#[tokio::main]
async fn make_req(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let parsed = Url::parse(&url)?;
    let host_name = parsed.host_str().unwrap();
    let client = reqwest::Client::new();
    println!("Running a check on: {}", style(host_name).cyan());

    // get response from endpoint
    let resp = client
        .get(format!("https://isitup.org/{}.json", host_name))
        .header(USER_AGENT, "https://github.com/austinkrueger/staticli")
        .send()
        .await?
        .text()
        .await?;

    // print out the results to the user
    println!("\n{} Results {} \n", Emoji("✨", "==="), Emoji("✨", "==="));
    let result: IsUpResponse = serde_json::from_str(resp.as_str()).unwrap();
    println!("Domain name: {}", style(&result.domain).magenta());
    println!("Port: {}", style(&result.port).magenta());

    if result.status_code == 1 {
        println!("Status: {} {}", style("Up").green(), Emoji("🚀", ""));
    } else if result.status_code == 2 {
        println!("Status: {} {}", style("Down").red(), Emoji("💥", ""));
        // we won't get the info we want if it is down, just return
        return Ok(());
    } else {
        println!(
            "Domain {} is invalid, please try again later.",
            style(&result.domain).red()
        );
        return Ok(());
    }

    println!("IP Address: {}", style(&result.response_ip.unwrap()).cyan());
    println!(
        "Status Code: {}",
        style(&result.response_code.unwrap()).cyan()
    );
    println!(
        "Response Time: {}",
        style(&result.response_time.unwrap()).cyan()
    );
    Ok(())
}
