use clap::Parser;
use notify_rust::Notification;
#[cfg(not(target_os = "macos"))]
use notify_rust::{Timeout, Urgency};

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Notification title
    summary: String,

    /// Notification body
    body: Option<String>,

    /// Application name
    #[cfg(not(target_os = "macos"))]
    #[arg(short = 'a', long, default_value = "notify")]
    app_name: String,

    /// Notification icon
    #[cfg(not(target_os = "macos"))]
    #[arg(short, long)]
    icon: Option<String>,

    /// Timeout in milliseconds
    #[cfg(not(target_os = "macos"))]
    #[arg(short, long, default_value_t = -1)]
    expire_time: i32,

    /// Urgency: low, normal, critical
    #[cfg(not(target_os = "macos"))]
    #[arg(short, long, default_value = "normal")]
    urgency: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("notify: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    #[cfg(not(target_os = "macos"))]
    let urgency = match cli.urgency.as_str() {
        "low" => Urgency::Low,
        "normal" => Urgency::Normal,
        "critical" => Urgency::Critical,
        _ => return Err("invalid urgency".into()),
    };

    let mut notification = Notification::new();

    notification
        .summary(&cli.summary)
        .body(cli.body.as_deref().unwrap_or(""));

    #[cfg(not(target_os = "macos"))]
    {
        notification
            .appname(&cli.app_name)
            .urgency(urgency)
            .timeout(Timeout::Milliseconds(cli.expire_time as u32));

        if let Some(icon) = &cli.icon {
            notification.icon(icon);
        }
    }

    notification.show()?;

    Ok(())
}
