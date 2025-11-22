use polymarket_rtds::{MessageType, RealTimeDataClient, Subscription, SubscriptionFilter, Topic};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Polymarket Real-Time Data Streaming Example ===\n");

    // Create a new client
    let mut client = RealTimeDataClient::new();

    // Connect to the WebSocket service
    println!("Connecting to Polymarket WebSocket...");
    client.connect().await?;
    println!("Connected successfully!\n");

    // Example 1: Subscribe to all market activity trades
    println!("Example 1: Subscribing to all market trades...");
    let trades_sub = Subscription::new(Topic::Activity, MessageType::Trades);
    client.subscribe(vec![trades_sub]).await?;
    println!("Subscribed to activity/trades\n");

    // Example 2: Subscribe to crypto price updates for Bitcoin
    println!("Example 2: Subscribing to Bitcoin price updates...");
    let btc_sub = Subscription::new(Topic::CryptoPrices, MessageType::Update)
        .with_filter(SubscriptionFilter::symbol("BTCUSDT"))?;
    client.subscribe(vec![btc_sub]).await?;
    println!("Subscribed to BTC price updates\n");

    // Example 3: Subscribe to a specific market by slug
    println!("Example 3: Subscribing to specific market activity...");
    let market_sub = Subscription::new(Topic::Activity, MessageType::Trades)
        .with_filter(SubscriptionFilter::market_slug("will-bitcoin-hit-100k"))?;
    client.subscribe(vec![market_sub]).await?;
    println!("Subscribed to specific market\n");

    // Example 4: Subscribe to comments for an event
    println!("Example 4: Subscribing to event comments...");
    let comments_sub = Subscription::new(Topic::Comments, MessageType::All)
        .with_filter(SubscriptionFilter::parent_entity(100, "Event"))?;
    client.subscribe(vec![comments_sub]).await?;
    println!("Subscribed to event comments\n");

    println!("Listening for messages... (Press Ctrl+C to stop)\n");

    // Handle incoming messages
    let mut message_count = 0;
    while let Some(message) = client.recv().await {
        match message {
            Ok(msg) => {
                message_count += 1;
                println!(
                    "[Message #{}] Topic: {} | Type: {}",
                    message_count, msg.topic, msg.message_type
                );
                println!("Payload: {}\n", msg.payload);

                // Stop after receiving 10 messages for this example
                if message_count >= 10 {
                    println!("Received 10 messages. Stopping...");
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
            }
        }
    }

    // Unsubscribe from topics (optional, connection close will also unsubscribe)
    println!("\nUnsubscribing from topics...");
    let unsubscribe_trades = Subscription::new(Topic::Activity, MessageType::Trades);
    client.unsubscribe(vec![unsubscribe_trades]).await?;
    println!("Unsubscribed from trades");

    // Disconnect from the service
    println!("\nDisconnecting...");
    client.disconnect().await?;
    println!("Disconnected successfully!");

    Ok(())
}
