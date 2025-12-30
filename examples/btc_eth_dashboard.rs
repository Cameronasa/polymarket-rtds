use polymarket_rtds::{MessageType, RealTimeDataClient, Subscription, Topic};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n╔══════════════════════════════════════════════════════════════════════════╗");
    println!("║                    📊 LIVE CRYPTO PRICE FEED 📊                          ║");
    println!("╠══════════════════════════════════════════════════════════════════════════╣");
    println!("║  SYMBOL  │     PRICE      │   CHANGE   │   $ MOVE   │    TIME    │  #    ║");
    println!("╚══════════════════════════════════════════════════════════════════════════╝\n");

    let mut client = RealTimeDataClient::default();
    println!("⏳ Connecting to Polymarket...");
    client.connect().await?;
    println!("✅ Connected! Live feed starting...\n");

    let crypto_sub = Subscription::new(Topic::CryptoPrices, MessageType::Update);
    client.subscribe(vec![crypto_sub]).await?;

    let mut latest_prices: HashMap<String, f64> = HashMap::new();
    let mut update_count: u64 = 0;

    while let Some(message) = client.recv().await {
        match message {
            Ok(msg) => {
                if msg.topic == Topic::CryptoPrices && msg.message_type == MessageType::Update {
                    if let Ok(crypto) =
                        serde_json::from_value::<polymarket_rtds::CryptoPrice>(msg.payload)
                    {
                        let symbol_lower = crypto.symbol.to_lowercase();
                        
                        if symbol_lower != "btcusdt" && symbol_lower != "ethusdt" {
                            continue;
                        }
                        
                        update_count += 1;
                        let prev_price = latest_prices.get(&symbol_lower).copied().unwrap_or(crypto.value);
                        let dollar_change = crypto.value - prev_price;
                        let pct_change = if prev_price != 0.0 { (dollar_change / prev_price) * 100.0 } else { 0.0 };
                        
                        latest_prices.insert(symbol_lower.clone(), crypto.value);

                        let time = chrono::DateTime::from_timestamp_millis(crypto.timestamp as i64)
                            .map(|dt| dt.format("%H:%M:%S").to_string())
                            .unwrap_or_default();

                        // Format the output
                        let symbol_display = if symbol_lower == "btcusdt" { "₿ BTC" } else { "Ξ ETH" };
                        
                        let (arrow, pct_str) = if dollar_change > 0.001 {
                            ("▲", format!("+{:.3}%", pct_change))
                        } else if dollar_change < -0.001 {
                            ("▼", format!("{:.3}%", pct_change))
                        } else {
                            ("━", " 0.000%".to_string())
                        };

                        let dollar_str = if dollar_change > 0.001 {
                            format!("+${:.2}", dollar_change)
                        } else if dollar_change < -0.001 {
                            format!("-${:.2}", dollar_change.abs())
                        } else {
                            " $0.00".to_string()
                        };

                        println!(
                            "  {}  │  ${:>11.2}  │  {} {:>7} │ {:>9} │  {}  │ {:>4}",
                            symbol_display,
                            crypto.value,
                            arrow,
                            pct_str,
                            dollar_str,
                            time,
                            update_count
                        );
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }
    }

    Ok(())
}
