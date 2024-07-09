use std::net::{Ipv4Addr, Ipv6Addr};
use regex::Regex;
use std::io::{self, Write};
use thirtyfour::prelude::*;
use tokio;
use std::{thread, time};
use std::collections::HashMap;

fn pause_with_delay(seconds: u64) {
    let pause_time = time::Duration::from_secs(seconds);
    thread::sleep(pause_time);
}

fn is_valid_domain(domain: &str) -> bool {
    let domain_regex = Regex::new(r"^(?i:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+(?:[a-z]{2,})$").unwrap();
    domain_regex.is_match(domain)
}

fn is_valid_ip(ip: &str) -> bool {
    ip.parse::<Ipv4Addr>().is_ok() || ip.parse::<Ipv6Addr>().is_ok()
}

fn sanitize_input(input: &str) -> Option<String> {
    let trimmed_input = input.trim();
    if is_valid_domain(trimmed_input) || is_valid_ip(trimmed_input) {
        Some(trimmed_input.to_string())
    } else {
        None
    }
}

async fn search_talos(query: &str) -> WebDriverResult<HashMap<String, String>> {
    let mut caps = DesiredCapabilities::chrome();
    let profile_path = ""; // Update this to the path of your profile
    caps.add_chrome_arg(&format!("--user-data-dir={}", profile_path))?;

    caps.add_chrome_arg("--homepage=about:blank")?;
    caps.add_chrome_arg("--no-sandbox")?;
    caps.add_chrome_arg("--disable-dev-shm-usage")?;

    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    let url = format!("https://talosintelligence.com/reputation_center/lookup?search={}", query);
    driver.goto(&url).await?;

    // Added delay to allow the webpage to load
    pause_with_delay(5);

    let mut data = HashMap::new();

    let sections = vec![
        ("LOCATION DATA", "Location Data"),
        ("IP ADDRESS", "IP Address"),
        ("FWD/REV DNS MATCH", "FWD/REV DNS MATCH"),
        ("HOSTNAME", "Hostname"),
        ("DOMAIN", "Domain"),
        ("NETWORK OWNER", "Network Owner"),
        ("SENDER IP REPUTATION", "Sender IP Reputation"),
        ("WEB REPUTATION", "Web Reputation"),
        ("EMAIL VOLUME LAST DAY", "Email Volume Last Day"),
        ("EMAIL VOLUME LAST MONTH", "Email Volume Last Month"),
        ("VOLUME CHANGE", "Volume Change"),
        ("SPAM LEVEL", "Spam Level"),
        ("BL.SPAMCOP.NET", "BL.SPAMCOP.NET"),
        ("CBL.ABUSEAT.ORG", "CBL.ABUSEAT.ORG"),
        ("PBL.SPAMHAUS.ORG", "PBL.SPAMHAUS.ORG"),
        ("SBL.SPAMHAUS.ORG", "SBL.SPAMHAUS.ORG"),
        ("ADDED TO THE BLOCK LIST", "Added To The Block List"),
    ];

    for (key, text) in sections {
        let xpath = if key == "LOCATION DATA" {
            "//div[@id='location-data-wrapper']//span[@class='flag-icon flag-icon-us']/parent::td".to_string()
        } else {
            format!("//td[text()='{}']/following-sibling::td", text)
        };

        if let Ok(element) = driver.find(By::XPath(&xpath)).await {
            if let Ok(element_text) = element.text().await {
                data.insert(key.to_string(), element_text);
            }
        }
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
    println!("	    *                     \\___ \\ / _` | | | |/ _ \\/ _` | |/ /\\___ \\\\___ \\                          ");
    println!("                                  ____) | (_| | |_| |  __| (_| |   < ____) ____) |                         ");
    println!("	    *                    |_____/ \\__, |\\__,_|\\___|\\__,______|_____|_____/        _     _____ _____ ");
    println!("		    *            |  _ \\     | |           | |/ ____|                    | |   / ____/ ____|");
    println!("                                 | |_) |_ __|___  __ _  __| | |     _ __ _   _ _ __ ___ | |__| (___| (___  ");
    println!("		    *            |  _ <| '__/ _ \\/ _` |/ _` | |    | '__| | | | '_ ` _ \\| '_ \\\\___ \\\\___ \\ ");
    println!("			    *    | |_) | | |  __| (_| | (_| | |____| |  | |_| | | | | | | |_) ____) ____) |");
    println!("			         |____/|_|  \\___|\\__,_|\\__,_|\\_____|_|   \\__,_|_| |_| |_|_.__|_____|_____/ ");
    println!("			    *");
    println!("				    *");
    println!("			    	 _________");
    println!("			    	(         )");
    println!("			    	 |       |");
    println!("			    	 |       |");
    println!("			    	 (_______)");

    pause_with_delay(3);

    let sanitized_input: String;

    loop {
        let mut input = String::new();
        print!("Please enter an IP address or domain: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        match sanitize_input(&input) {
            Some(valid_input) => {
                sanitized_input = valid_input;
                break;
            },
            None => {
                println!("Invalid input. Please enter a valid IP address or domain.");
            }
        }
    }

    match search_talos(&sanitized_input).await {
        Ok(data) => {
            println!("\nLOCATION DATA");
            println!("{}", data.get("LOCATION DATA").unwrap_or(&"".to_string()));

            println!("\nOWNER DETAILS");
            println!("IP ADDRESS:\t{}", data.get("IP ADDRESS").unwrap_or(&"".to_string()));
            println!("FWD/REV DNS MATCH:\t{}", data.get("FWD/REV DNS MATCH").unwrap_or(&"".to_string()));
            println!("HOSTNAME:\t{}", data.get("HOSTNAME").unwrap_or(&"".to_string()));
            println!("DOMAIN:\t{}", data.get("DOMAIN").unwrap_or(&"".to_string()));
            println!("NETWORK OWNER:\t{}", data.get("NETWORK OWNER").unwrap_or(&"".to_string()));

            println!("\nREPUTATION DETAILS");
            println!("SENDER IP REPUTATION:\t{}", data.get("SENDER IP REPUTATION").unwrap_or(&"".to_string()));
            println!("WEB REPUTATION:\t{}", data.get("WEB REPUTATION").unwrap_or(&"".to_string()));

            println!("\nEMAIL VOLUME DATA");
            println!("EMAIL VOLUME:\t{} {}", data.get("EMAIL VOLUME LAST DAY").unwrap_or(&"".to_string()), data.get("EMAIL VOLUME LAST MONTH").unwrap_or(&"".to_string()));
            println!("VOLUME CHANGE:\t{}", data.get("VOLUME CHANGE").unwrap_or(&"".to_string()));
            println!("SPAM LEVEL:\t{}", data.get("SPAM LEVEL").unwrap_or(&"".to_string()));

            println!("\nBLOCK LISTS");
            println!("BL.SPAMCOP.NET:\t{}", data.get("BL.SPAMCOP.NET").unwrap_or(&"".to_string()));
            println!("CBL.ABUSEAT.ORG:\t{}", data.get("CBL.ABUSEAT.ORG").unwrap_or(&"".to_string()));
            println!("PBL.SPAMHAUS.ORG:\t{}", data.get("PBL.SPAMHAUS.ORG").unwrap_or(&"".to_string()));
            println!("SBL.SPAMHAUS.ORG:\t{}", data.get("SBL.SPAMHAUS.ORG").unwrap_or(&"".to_string()));

            println!("\nTALOS SECURITY INTELLIGENCE BLOCK LIST");
            println!("ADDED TO THE BLOCK LIST:\t{}", data.get("ADDED TO THE BLOCK LIST").unwrap_or(&"".to_string()));
        },
        Err(err) => eprintln!("Error during search: {}", err),
    }

    Ok(())
}
