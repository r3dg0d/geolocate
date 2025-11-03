use clap::Parser;
use reqwest;
use serde::Deserialize;
use std::net::IpAddr;

#[derive(Parser)]
#[command(name = "geolocate")]
#[command(about = "Geolocate an IP address or URL")]
struct Args {
    #[arg(help = "IP address or URL to geolocate")]
    target: String,
}

#[derive(Deserialize)]
struct IpApiResponse {
    status: String,
    message: Option<String>,
    country: String,
    #[serde(alias = "regionName")]
    region_name: Option<String>,
    city: String,
    lat: f64,
    lon: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Determine if target is a URL or IP
    let ip: IpAddr = if args.target.contains("://") || args.target.contains(".") && !args.target.chars().all(|c| c.is_ascii_digit() || c == '.') {
        // It's likely a URL or domain name
        let url_str = if args.target.contains("://") {
            args.target.clone()
        } else {
            format!("https://{}", args.target)
        };
        let url = url::Url::parse(&url_str)?;
        let host = url.host_str().ok_or("Invalid host in URL")?;
        let ips = dns_lookup::lookup_host(host)?;
        ips.iter().find(|ip| ip.is_ipv4()).copied().unwrap_or(ips[0])
    } else {
        // Assume it's an IP
        args.target.parse()?
    };

    // Query ip-api.com
    let url = format!("http://ip-api.com/json/{}", ip);
    let response: IpApiResponse = reqwest::get(&url).await?.json().await?;

    if response.status != "success" {
        eprintln!("Geolocation failed: {}", response.message.unwrap_or("Unknown error".to_string()));
        std::process::exit(1);
    }

    // Output results
    println!("Target: {}", args.target);
    println!("IP: {}", ip);
    let region = response.region_name.as_deref().unwrap_or("N/A");
    println!("Location: {}, {}, {}", response.city, region, response.country);
    println!("Coordinates: {}, {}", response.lat, response.lon);
    println!("Google Maps URL: https://maps.google.com/maps?q={},{}", response.lat, response.lon);

    Ok(())
}

