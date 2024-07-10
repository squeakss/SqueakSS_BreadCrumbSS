use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Write};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::{thread, time};
use thirtyfour::prelude::*;
use tokio;

// Function to pause execution for a given number of seconds
fn pause_with_delay(seconds: u64) {
    let pause_time = time::Duration::from_secs(seconds);
    thread::sleep(pause_time);
}

// Function to validate domain names using a regex pattern
fn is_valid_domain(domain: &str) -> bool {
    let domain_regex =
        Regex::new(r"^(?i:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+(?:[a-z]{2,})$").unwrap();
    domain_regex.is_match(domain)
}

// Function to validate IP addresses
fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok() || ip.parse::<Ipv6Addr>().is_ok()
}

// Function to sanitize user input to ensure it is a valid domain or IP address
fn sanitize_input(input: &str) -> Option<String> {
    let trimmed_input = input.trim();
    if is_valid_domain(trimmed_input) || is_valid_ip(trimmed_input) {
        Some(trimmed_input.to_string())
    } else {
        None
    }
}

// Function to extract text from an element found by XPath
async fn get_element_text(driver: &WebDriver, xpath: &str) -> WebDriverResult<Option<String>> {
    match driver.find(By::XPath(xpath)).await {
        Ok(element) => {
            let text = element.text().await?;
            Ok(Some(text))
        }
        Err(_) => Ok(None),
    }
}

// Function to scrape location data
async fn scrape_location_data(driver: &WebDriver) -> WebDriverResult<Option<String>> {
    let xpath =
        "//div[@id='location-data-wrapper']//span[@class='flag-icon flag-icon-us']/parent::td";
    get_element_text(driver, xpath).await
}

// Function to scrape owner details
async fn scrape_owner_details(driver: &WebDriver) -> WebDriverResult<HashMap<String, String>> {
    let mut data = HashMap::new();

    let sections = vec![
        (
            "IP ADDRESS",
            "//td[text()='IP Address']/following-sibling::td",
        ),
        (
            "FWD/REV DNS MATCH",
            "//td[text()='FWD/REV DNS MATCH']/following-sibling::td",
        ),
        ("HOSTNAME", "//td[text()='Hostname']/following-sibling::td"),
        ("DOMAIN", "//td[text()='Domain']/following-sibling::td"),
        (
            "NETWORK OWNER",
            "//td[text()='Network Owner']/following-sibling::td",
        ),
    ];

    for (key, xpath) in sections {
        if let Some(text) = get_element_text(driver, xpath).await? {
            data.insert(key.to_string(), text);
        }
    }

    Ok(data)
}

// Function to scrape reputation details
async fn scrape_reputation_details(driver: &WebDriver) -> WebDriverResult<HashMap<String, String>> {
    let mut data = HashMap::new();

    let sections = vec![
        (
            "SENDER IP REPUTATION",
            "//td[text()='Sender IP Reputation']/following-sibling::td",
        ),
        (
            "WEB REPUTATION",
            "//td[text()='Web Reputation']/following-sibling::td",
        ),
    ];

    for (key, xpath) in sections {
        if let Some(text) = get_element_text(driver, xpath).await? {
            data.insert(key.to_string(), text);
        }
    }

    Ok(data)
}

// Function to scrape email volume data
async fn scrape_email_volume_data(driver: &WebDriver) -> WebDriverResult<HashMap<String, String>> {
    let mut data = HashMap::new();

    let sections = vec![
        (
            "EMAIL VOLUME LAST DAY",
            "//td[text()='Email Volume Last Day']/following-sibling::td",
        ),
        (
            "EMAIL VOLUME LAST MONTH",
            "//td[text()='Email Volume Last Month']/following-sibling::td",
        ),
        (
            "VOLUME CHANGE",
            "//td[text()='Volume Change']/following-sibling::td",
        ),
        (
            "SPAM LEVEL",
            "//td[text()='Spam Level']/following-sibling::td",
        ),
    ];

    for (key, xpath) in sections {
        if let Some(text) = get_element_text(driver, xpath).await? {
            data.insert(key.to_string(), text);
        }
    }

    Ok(data)
}

// Function to scrape block lists
async fn scrape_block_lists(driver: &WebDriver) -> WebDriverResult<HashMap<String, String>> {
    let mut data = HashMap::new();

    let sections = vec![
        ("BL.SPAMCOP.NET", "//td[@class='chart-data-label col_left']/a[contains(@href, 'spamcop.net')]/../following-sibling::td"),
        ("CBL.ABUSEAT.ORG", "//td[@class='chart-data-label col_left']/a[contains(@href, 'abuseat.org')]/../following-sibling::td"),
        ("PBL.SPAMHAUS.ORG", "//td[@class='chart-data-label col_left']/a[contains(@href, 'spamhaus.org')]/../following-sibling::td"),
        ("SBL.SPAMHAUS.ORG", "//td[@class='chart-data-label col_left']/a[contains(@href, 'spamhaus.org')]/../following-sibling::td"),
        ("ADDED TO THE BLOCK LIST", "//td[text()='Added to the Block List']/following-sibling::td"),
    ];

    for (key, xpath) in sections {
        if let Some(text) = get_element_text(driver, xpath).await? {
            data.insert(key.to_string(), text);
        }
    }

    Ok(data)
}

// Main function to search Talos and display the results
async fn search_talos(query: &str) -> WebDriverResult<HashMap<String, HashMap<String, String>>> {
    let mut caps = DesiredCapabilities::chrome();
    let profile_path = "C:/Users/atteb/AppData/Local/Google/Chrome/User Data"; // Update this to the path of your profile
    caps.add_chrome_arg(&format!("--user-data-dir={}", profile_path))?;
    caps.add_chrome_arg("--homepage=about:blank")?;
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;
    let url = format!(
        "https://talosintelligence.com/reputation_center/lookup?search={}",
        query
    );
    driver.goto(&url).await?;

    // Added delay to allow the webpage to load
    pause_with_delay(5);

    let mut data = HashMap::new();

    if let Some(location) = scrape_location_data(&driver).await? {
        data.insert(
            "LOCATION DATA".to_string(),
            [("Location".to_string(), location)]
                .iter()
                .cloned()
                .collect(),
        );
    }

    let owner_details = scrape_owner_details(&driver).await?;
    if !owner_details.is_empty() {
        data.insert("OWNER DETAILS".to_string(), owner_details);
    }

    let reputation_details = scrape_reputation_details(&driver).await?;
    if !reputation_details.is_empty() {
        data.insert("REPUTATION DETAILS".to_string(), reputation_details);
    }

    let email_volume_data = scrape_email_volume_data(&driver).await?;
    if !email_volume_data.is_empty() {
        data.insert("EMAIL VOLUME DATA".to_string(), email_volume_data);
    }

    let block_lists = scrape_block_lists(&driver).await?;
    if !block_lists.is_empty() {
        data.insert("BLOCK LISTS".to_string(), block_lists);
    }

    driver.quit().await?;
    Ok(data)
}

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    env_logger::init();

    // Print ASCII art
    println!(" (\\(\\                              _____                        _     _____ _____                          ");
    println!("( -.-)                            / ____|                      | |   / ____/ ____|                         ");
    println!("o_(\")(\")                         | (___   __ _ _   _  ___  __ _| | _| (___| (___                           ");
    println!("            *                     \\___ \\ / _` | | | |/ _ \\/ _` | |/ /\\___ \\\\___ \\                          ");
    println!("                                  ____) | (_| | |_| |  __| (_| |   < ____) ____) |                         ");
    println!("            *                    |_____/ \\__, |\\__,_|\\___|\\__,______|_____|_____/        _     _____ _____ ");
    println!("                    *            |  _ \\     | |           | |/ ____|                    | |   / ____/ ____|");
    println!("                                 | |_) |_ __|___  __ _  __| | |     _ __ _   _ _ __ ___ | |__| (___| (___  ");
    println!("                    *            |  _ <| '__/ _ \\/ _` |/ _` | |    | '__| | | | '_ ` _ \\| '_ \\\\___ \\___  \\ ");
    println!("                            *    | |_) | | |  __| (_| | (_| | |____| |  | |_| | | | | | | |_) ____) ____) |");
    println!("                                 |____/|_|  \\___|\\__,_|\\__,_|\\_____|_|   \\__,_|_| |_| |_|_.__|_____|_____/ ");
    println!("                            *");
    println!("                                    *");
    println!("                                 _________");
    println!("                                (         )");
    println!("                                 |       |");
    println!("                                 |       |");
    println!("                                 (_______)");

    pause_with_delay(3);

    let sanitized_input: String;

    // Loop to get user input and validate it
    loop {
        let mut input = String::new();
        print!("Please enter an IP address or domain: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match sanitize_input(&input) {
            Some(valid_input) => {
                sanitized_input = valid_input;
                break;
            }
            None => {
                println!("Invalid input. Please enter a valid IP address or domain.");
            }
        }
    }

    // Search Talos and display the results
    match search_talos(&sanitized_input).await {
        Ok(data) => {
            if let Some(location_data) = data.get("LOCATION DATA") {
                println!("\nLOCATION DATA");
                if let Some(location) = location_data.get("Location") {
                    println!("{}", location);
                }
            }

            if let Some(owner_details) = data.get("OWNER DETAILS") {
                println!("\nOWNER DETAILS");
                for (key, value) in owner_details {
                    println!("{}:\t{}", key, value);
                }
            }

            if let Some(reputation_details) = data.get("REPUTATION DETAILS") {
                println!("\nREPUTATION DETAILS");
                for (key, value) in reputation_details {
                    println!("{}:\t{}", key, value);
                }
            }

            if let Some(email_volume_data) = data.get("EMAIL VOLUME DATA") {
                println!("\nEMAIL VOLUME DATA");
                for (key, value) in email_volume_data {
                    println!("{}:\t{}", key, value);
                }
            }

            if let Some(block_lists) = data.get("BLOCK LISTS") {
                println!("\nBLOCK LISTS");
                for (key, value) in block_lists {
                    println!("{}:\t{}", key, value);
                }
            }

            if let Some(talos_block_list) = data.get("TALOS SECURITY INTELLIGENCE BLOCK LIST") {
                println!("\nTALOS SECURITY INTELLIGENCE BLOCK LIST");
                for (key, value) in talos_block_list {
                    println!("{}:\t{}", key, value);
                }
            }
        }
        Err(err) => eprintln!("Error during search: {}", err),
    }

    Ok(())
}
