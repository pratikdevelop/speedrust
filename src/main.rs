use clap::Parser;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rand::RngCore;
use reqwest::Client;
use std::time::{Duration, Instant};
use chrono::Local;

#[derive(Parser, Debug)]
#[command(version, about = "SpeedRust – Fast Internet Speed Test CLI (optimized for India)")]
struct Args {
    /// Number of test iterations (default: 3)
    #[arg(short, long, default_value_t = 3)]
    iterations: u32,

    /// Custom download server URL (optional)
    #[arg(short, long)]
    server: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("{}", "Starting SpeedRust Test...".bright_cyan().bold());
    println!("Date: {}", Local::now().format("%Y-%m-%d %H:%M:%S %Z"));
    println!("Location hint: Ujjain, Madhya Pradesh (Jio 5G optimized)");

    let client = Client::builder()
        .timeout(Duration::from_secs(300))
        .connect_timeout(Duration::from_secs(30))
        .build()?;

    let download_url = args.server.unwrap_or_else(|| {
        "http://speedtest.tele2.net/100MB.zip".to_string()  // Reliable Tele2 100MB
    });

    println!("Using download server: {}", download_url.bright_cyan());
    println!("Test file: ~100 MB → accurate sustained speed");
    println!("Note for Jio 5G: Run near window, use Ethernet if possible, test off-peak for stability.\n");

    let upload_url = "https://file.io".to_string();  // Better public upload endpoint (temp file)

    let mut ping_times: Vec<u128> = Vec::new();
    let mut download_speeds: Vec<f64> = Vec::new();
    let mut upload_speeds: Vec<f64> = Vec::new();

    for i in 1..=args.iterations {
        println!("{}", format!("\n┌──── Test {}/{} ─────┐", i, args.iterations).bright_blue());

        let ping = measure_ping(&client, &download_url).await?;
        println!("  {} {}", "Ping:".bright_white(), format!("{} ms", ping).bright_green());
        ping_times.push(ping);

        let dl_speed = measure_download_with_progress(&client, &download_url).await?;
        println!("  {} {}", "Download:".bright_white(), format!("{:.2} Mbps", dl_speed).bright_green());
        download_speeds.push(dl_speed);

        let ul_speed = measure_upload_with_progress(&client, &upload_url).await?;
        println!("  {} {}", "Upload:  ".bright_white(), format!("{:.2} Mbps", ul_speed).bright_green());
        upload_speeds.push(ul_speed);

        println!("{}", "└──────────────────────┘".bright_blue());
    }

    let avg_ping = ping_times.iter().sum::<u128>() as f64 / args.iterations as f64;
    let avg_dl = download_speeds.iter().sum::<f64>() / args.iterations as f64;
    let avg_ul = upload_speeds.iter().sum::<f64>() / args.iterations as f64;

    println!("\n{}", "═".repeat(40).bright_magenta());
    println!("{}", "         FINAL RESULTS          ".bright_magenta().bold().on_black());
    println!("{}", "═".repeat(40).bright_magenta());

    println!("  {} {}", "Average Ping:    ".bright_white(), format!("{:.0} ms", avg_ping).bright_yellow());
    println!("  {} {}", "Average Download:".bright_white(), format!("{:.2} Mbps", avg_dl).bright_green());
    println!("  {} {}", "Average Upload:  ".bright_white(), format!("{:.2} Mbps", avg_ul).bright_green());

    if avg_dl > 25.0 {
        println!("\n{}", "Great performance! Your Jio 5G is flying 🚀".bright_green().bold());
    } else if avg_dl > 10.0 {
        println!("\n{}", "Solid speeds for everyday use ✓".bright_yellow());
    } else {
        println!("\n{}", "Check 5G signal/router – try near window or Ethernet".bright_red());
    }

    Ok(())
}

async fn measure_ping(client: &Client, url: &str) -> Result<u128, reqwest::Error> {
    let start = Instant::now();
    client.head(url).send().await?;
    Ok(start.elapsed().as_millis())
}

async fn measure_download_with_progress(client: &Client, url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("  {}", "Downloading test file...".yellow());

    let mut attempts = 0;
    let max_attempts = 3;

    loop {
        attempts += 1;

        let response = match client.get(url).send().await {
            Ok(resp) => resp,
            Err(e) if e.is_timeout() || e.is_connect() => {
                if attempts >= max_attempts {
                    println!("{}", format!("Failed after {} attempts (timeout/connect). Try smaller file or better signal.", max_attempts).bright_red());
                    return Err(Box::new(e));
                }
                println!("{}", format!("Connection issue... retrying {}/{}", attempts, max_attempts).bright_yellow());
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        };

        let total_size = response.content_length().unwrap_or(0);

        let pb = if total_size > 0 {
            let pb = ProgressBar::new(total_size);
            pb.set_style(
                ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                    .unwrap()
                    .progress_chars("#>-")
            );
            pb.enable_steady_tick(Duration::from_millis(100));
            pb
        } else {
            ProgressBar::hidden()
        };

        let start = Instant::now();
        let bytes_result = response.bytes().await;

        match bytes_result {
            Ok(bytes) => {
                pb.finish_with_message("Download complete!");
                let duration = start.elapsed().as_secs_f64();
                let size_mb = bytes.len() as f64 / 1_048_576.0;
                let speed_mbps = (size_mb * 8.0) / duration;
                return Ok(speed_mbps);
            }
            Err(e) if e.to_string().contains("reset") || e.is_timeout() => {
                pb.abandon_with_message("Connection reset – retrying...");
                if attempts >= max_attempts {
                    println!("{}", format!("Failed after {} retries (reset). Try smaller file.", max_attempts).bright_red());
                    return Err(Box::new(e));
                }
                println!("{}", format!("Connection reset... retrying {}/{}", attempts, max_attempts).bright_yellow());
                continue;
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
}

async fn measure_upload_with_progress(client: &Client, url: &str) -> Result<f64, Box<dyn std::error::Error>> {
    println!("  {}", "Uploading test data (10 MB)... (public temp endpoint)".yellow());

    let data_size: usize = 10 * 1_048_576;  // Reduced for reliability on public endpoints
    let mut random_data = vec![0u8; data_size];
    rand::rng().fill_bytes(&mut random_data);

    let pb = ProgressBar::new(data_size as u64);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    let start = Instant::now();

    let progress_handle = tokio::spawn({
        let pb = pb.clone();
        async move {
            let mut uploaded = 0u64;
            let step = (data_size as u64 / 50).max(1);
            while uploaded < data_size as u64 {
                tokio::time::sleep(Duration::from_millis(150)).await;
                let chunk = step.min(data_size as u64 - uploaded);
                uploaded += chunk;
                pb.inc(chunk);
            }
        }
    });

    let mut response = client.post(url).body(random_data).send().await?;
    while let Some(_chunk) = response.chunk().await? {}

    progress_handle.await.ok();
    let duration = start.elapsed().as_secs_f64();
    pb.finish_with_message("Upload complete!");

    let size_mb = data_size as f64 / 1_048_576.0;
    let speed_mbps = (size_mb * 8.0) / duration;

    Ok(speed_mbps)
}