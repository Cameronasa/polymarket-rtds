use polymarket_rtds::{MessageType, RealTimeDataClient, Subscription, SubscriptionFilter, Topic};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Crypto Price Monitor Example ===\n");

    let mut client = RealTimeDataClient::default();
    println!("Connecting to Polymarket WebSocket...");
    client.connect().await?;
    println!("Connected successfully!\n");

    let symbols = vec!["BTCUSDT", "ETHUSDT", "SOLUSDT"];

    for symbol in &symbols {
        let subscription = Subscription::new(Topic::CryptoPrices, MessageType::Update)
            .with_filter(SubscriptionFilter::symbol(*symbol))?;
        client.subscribe(vec![subscription]).await?;
        println!("Subscribed to {} price updates", symbol);
    }

    println!("\nMonitoring crypto prices... (Press Ctrl+C to stop)\n");

    let mut latest_prices: HashMap<String, f64> = HashMap::new();
    let mut update_count = 0;

    while let Some(message) = client.recv().await {
        match message {
            Ok(msg) => {
                if msg.topic == Topic::CryptoPrices && msg.message_type == MessageType::Update {
                    if let Ok(crypto) =
                        serde_json::from_value::<polymarket_rtds::CryptoPrice>(msg.payload)
                    {
                        let previous_price = latest_prices.get(&crypto.symbol);

                        let change_indicator = match previous_price {
                            Some(&prev) if crypto.value > prev => "↑ UP  ",
                            Some(&prev) if crypto.value < prev => "↓ DOWN",
                            _ => "→ SAME",
                        };

                        println!(
                            "[{}] {} | ${:.2} | Timestamp: {}",
                            change_indicator,
                            crypto.symbol.to_uppercase(),
                            crypto.value,
                            crypto.timestamp
                        );

                        latest_prices.insert(crypto.symbol, crypto.value);
                        update_count += 1;

                        if update_count % 20 == 0 {
                            println!("\n--- Price Summary ---");
                            for (symbol, price) in &latest_prices {
                                println!("{}: ${:.2}", symbol.to_uppercase(), price);
                            }
                            println!("--------------------\n");
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(())
}
