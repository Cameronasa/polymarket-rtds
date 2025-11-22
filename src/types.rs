use serde::{Deserialize, Serialize};

// ============================================================================
// Main WebSocket Response Enum
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketResponse {
    // Activity topic
    #[serde(rename = "trades")]
    Trades(TradeActivity),
    #[serde(rename = "orders_matched")]
    OrdersMatched(TradeActivity),

    // Comments topic
    #[serde(rename = "comment_created")]
    CommentCreated(Comment),
    #[serde(rename = "comment_removed")]
    CommentRemoved(Comment),
    #[serde(rename = "reaction_created")]
    ReactionCreated(Reaction),
    #[serde(rename = "reaction_removed")]
    ReactionRemoved(Reaction),

    // RFQ topic
    #[serde(rename = "request_created")]
    RequestCreated(RfqRequest),
    #[serde(rename = "request_edited")]
    RequestEdited(RfqRequest),
    #[serde(rename = "request_canceled")]
    RequestCanceled(RfqRequest),
    #[serde(rename = "request_expired")]
    RequestExpired(RfqRequest),
    #[serde(rename = "quote_created")]
    QuoteCreated(RfqQuote),
    #[serde(rename = "quote_edited")]
    QuoteEdited(RfqQuote),
    #[serde(rename = "quote_canceled")]
    QuoteCanceled(RfqQuote),
    #[serde(rename = "quote_expired")]
    QuoteExpired(RfqQuote),

    // Crypto/Equity prices
    #[serde(rename = "update")]
    PriceUpdate(PriceUpdate),

    // CLOB User (authenticated)
    #[serde(rename = "order")]
    Order(ClobOrder),
    #[serde(rename = "trade")]
    Trade(ClobTrade),

    // CLOB Market
    #[serde(rename = "price_change")]
    PriceChange(PriceChanges),
    #[serde(rename = "agg_orderbook")]
    AggOrderbook(AggOrderbook),
    #[serde(rename = "last_trade_price")]
    LastTradePrice(LastTradePrice),
    #[serde(rename = "tick_size_change")]
    TickSizeChange(TickSizeChange),
    #[serde(rename = "market_created")]
    MarketCreated(ClobMarket),
    #[serde(rename = "market_resolved")]
    MarketResolved(ClobMarket),

    #[serde(other)]
    Unknown,
}

// ============================================================================
// Activity Topic - Trade
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeActivity {
    /// ERC1155 token ID of conditional token being traded
    pub asset: String,
    /// Bio of the user of the trade
    pub bio: String,
    /// Id of market which is also the CTF condition ID
    #[serde(rename = "conditionId")]
    pub condition_id: String,
    /// Slug of the event
    #[serde(rename = "eventSlug")]
    pub event_slug: String,
    /// URL to the market icon image
    pub icon: String,
    /// Name of the user of the trade
    pub name: String,
    /// Human readable outcome of the market
    pub outcome: String,
    /// Index of the outcome
    #[serde(rename = "outcomeIndex")]
    pub outcome_index: i32,
    /// Price of the trade
    pub price: f64,
    /// URL to the user profile image
    #[serde(rename = "profileImage")]
    pub profile_image: String,
    /// Address of the user proxy wallet
    #[serde(rename = "proxyWallet")]
    pub proxy_wallet: String,
    /// Pseudonym of the user
    pub pseudonym: String,
    /// Side of the trade (BUY/SELL)
    pub side: String,
    /// Size of the trade
    pub size: i64,
    /// Slug of the market
    pub slug: String,
    /// Timestamp of the trade
    pub timestamp: i64,
    /// Title of the event
    pub title: String,
    /// Hash of the transaction
    #[serde(rename = "transactionHash")]
    pub transaction_hash: String,
}

// ============================================================================
// Comments Topic
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    /// Unique identifier of comment
    pub id: String,
    /// Content of the comment
    pub body: String,
    /// Type of the parent entity (Event or Series)
    #[serde(rename = "parentEntityType")]
    pub parent_entity_type: String,
    /// ID of the parent entity
    #[serde(rename = "parentEntityID")]
    pub parent_entity_id: i64,
    /// ID of the parent comment
    #[serde(rename = "parentCommentID")]
    pub parent_comment_id: Option<String>,
    /// Address of the user
    #[serde(rename = "userAddress")]
    pub user_address: String,
    /// Address of the reply user
    #[serde(rename = "replyAddress")]
    pub reply_address: Option<String>,
    /// Creation timestamp
    #[serde(rename = "createdAt")]
    pub created_at: String,
    /// Last update timestamp
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    /// Unique identifier of reaction
    pub id: String,
    /// ID of the comment
    #[serde(rename = "commentID")]
    pub comment_id: i64,
    /// Type of the reaction
    #[serde(rename = "reactionType")]
    pub reaction_type: String,
    /// Icon representing the reaction
    pub icon: String,
    /// Address of the user
    #[serde(rename = "userAddress")]
    pub user_address: String,
    /// Creation timestamp
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

// ============================================================================
// RFQ Topic
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfqRequest {
    /// Unique identifier for the request
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// User proxy address
    #[serde(rename = "proxyAddress")]
    pub proxy_address: String,
    /// Id of market which is also the CTF condition ID
    pub market: String,
    /// ERC1155 token ID of conditional token being traded
    pub token: String,
    /// Complement ERC1155 token ID of conditional token being traded
    pub complement: String,
    /// Current state of the request
    pub state: String,
    /// Indicates buy or sell side
    pub side: String,
    /// Input size of the request
    #[serde(rename = "sizeIn")]
    pub size_in: f64,
    /// Output size of the request
    #[serde(rename = "sizeOut")]
    pub size_out: f64,
    /// Price from in/out sizes
    pub price: f64,
    /// Expiry timestamp (UNIX format)
    pub expiry: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfqQuote {
    /// Unique identifier for the quote
    #[serde(rename = "quoteId")]
    pub quote_id: String,
    /// Associated request identifier
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// User proxy address
    #[serde(rename = "proxyAddress")]
    pub proxy_address: String,
    /// ERC1155 token ID of conditional token being traded
    pub token: String,
    /// Current state of the quote
    pub state: String,
    /// Indicates buy or sell side
    pub side: String,
    /// Input size of the quote
    #[serde(rename = "sizeIn")]
    pub size_in: f64,
    /// Output size of the quote
    #[serde(rename = "sizeOut")]
    pub size_out: f64,
    /// Id of market which is also the CTF condition ID
    pub condition: String,
    /// Complement ERC1155 token ID of conditional token being traded
    pub complement: String,
    /// Expiry timestamp (UNIX format)
    pub expiry: i64,
}

// ============================================================================
// Crypto/Equity Prices
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriceUpdate {
    CryptoPrice(CryptoPrice),
    EquityPrice(EquityPrice),
    CryptoPriceHistorical(CryptoPriceHistorical),
    EquityPriceHistorical(EquityPriceHistorical),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPrice {
    /// Symbol of the asset
    pub symbol: String,
    /// Timestamp in milliseconds for the update
    pub timestamp: i64,
    /// Value at the time of update
    pub value: f64,
    /// Full accuracy value as string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_accuracy_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPrice {
    /// Symbol of the asset
    pub symbol: String,
    /// Timestamp in milliseconds for the update
    pub timestamp: i64,
    /// Value at the time of update
    pub value: f64,
    /// Full accuracy value as string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_accuracy_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoPriceHistorical {
    /// Symbol of the asset
    pub symbol: String,
    /// Array of price data objects
    pub data: Vec<PriceDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityPriceHistorical {
    /// Symbol of the asset
    pub symbol: String,
    /// Array of price data objects
    pub data: Vec<PriceDataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceDataPoint {
    pub timestamp: i64,
    pub value: f64,
}

// ============================================================================
// CLOB User (Authenticated)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClobOrder {
    /// Order's ERC1155 token ID of conditional token
    pub asset_id: String,
    /// Order's creation UNIX timestamp
    pub created_at: String,
    /// Order's expiration UNIX timestamp
    pub expiration: String,
    /// Unique order hash identifier
    pub id: String,
    /// Maker's address (funder)
    pub maker_address: String,
    /// Condition ID or market identifier
    pub market: String,
    /// Type of order: GTC, GTD, FOK, FAK
    pub order_type: String,
    /// Original size of the order at placement
    pub original_size: String,
    /// Order outcome: YES / NO
    pub outcome: String,
    /// UUID of the order owner
    pub owner: String,
    /// Order price (e.g., in decimals like 0.5)
    pub price: String,
    /// Side of the trade: BUY or SELL
    pub side: String,
    /// Amount of order that has been matched
    pub size_matched: String,
    /// Status of the order (e.g., MATCHED)
    pub status: String,
    /// Type of update: PLACEMENT, CANCELLATION, FILL, etc.
    #[serde(rename = "type")]
    pub update_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClobTrade {
    /// ERC1155 token ID of the conditional token involved in the trade
    pub asset_id: String,
    /// Fee rate in basis points (bps)
    pub fee_rate_bps: String,
    /// Unique identifier for the match record
    pub id: String,
    /// Last update timestamp (UNIX)
    pub last_update: String,
    /// Maker's address
    pub maker_address: String,
    /// List of maker orders
    pub maker_orders: Vec<MakerOrder>,
    /// Condition ID or market identifier
    pub market: String,
    /// Match execution timestamp (UNIX)
    pub match_time: String,
    /// Outcome of the market: YES / NO
    pub outcome: String,
    /// UUID of the taker (owner of the matched order)
    pub owner: String,
    /// Matched price (in decimal format, e.g., 0.5)
    pub price: String,
    /// Taker side of the trade: BUY or SELL
    pub side: String,
    /// Total matched size
    pub size: String,
    /// Status of the match: e.g., MINED
    pub status: String,
    /// ID of the taker's order
    pub taker_order_id: String,
    /// Transaction hash where the match was settled
    pub transaction_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MakerOrder {
    /// ERC1155 token ID of the conditional token of the maker's order
    pub asset_id: String,
    /// Maker's fee rate in basis points
    pub fee_rate_bps: String,
    /// Maker's address
    pub maker_address: String,
    /// Amount matched from the maker's order
    pub matched_amount: String,
    /// ID of the maker's order
    pub order_id: String,
    /// Outcome targeted by the maker's order (YES / NO)
    pub outcome: String,
    /// UUID of the maker
    pub owner: String,
    /// Order price
    pub price: String,
    /// Side of the maker: BUY or SELL
    pub side: String,
}

// ============================================================================
// CLOB Market
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChanges {
    /// Condition ID
    #[serde(rename = "m")]
    pub market: String,
    /// Price changes by book
    #[serde(rename = "pc")]
    pub price_change: Vec<PriceChange>,
    /// Timestamp in milliseconds since epoch (UNIX time * 1000)
    #[serde(rename = "t")]
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceChange {
    /// Asset identifier
    #[serde(rename = "a")]
    pub asset_id: String,
    /// Unique hash ID of the book snapshot
    #[serde(rename = "h")]
    pub hash: String,
    /// Price quoted (e.g., 0.5)
    #[serde(rename = "p")]
    pub price: String,
    /// Side of the quote: BUY or SELL
    #[serde(rename = "s")]
    pub side: String,
    /// Size or volume available at the quoted price
    #[serde(rename = "si")]
    pub size: String,
    /// Best ask price
    #[serde(rename = "ba")]
    pub best_ask: String,
    /// Best bid price
    #[serde(rename = "bb")]
    pub best_bid: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggOrderbook {
    /// List of ask aggregated orders (sell side)
    pub asks: Vec<OrderLevel>,
    /// Asset Id identifier
    pub asset_id: String,
    /// List of aggregated bid orders (buy side)
    pub bids: Vec<OrderLevel>,
    /// Unique hash ID for this orderbook snapshot
    pub hash: String,
    /// Market or condition ID
    pub market: String,
    /// Minimum allowed order size
    pub min_order_size: String,
    /// NegRisk or not
    pub neg_risk: bool,
    /// Minimum tick size
    pub tick_size: String,
    /// Timestamp in milliseconds since epoch (UNIX time * 1000)
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLevel {
    /// Price level
    pub price: String,
    /// Size at that price
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LastTradePrice {
    /// Asset Id identifier
    pub asset_id: String,
    /// Fee rate in basis points (bps)
    pub fee_rate_bps: String,
    /// Market or condition ID
    pub market: String,
    /// Trade price (e.g., 0.5)
    pub price: String,
    /// Side of the order: BUY or SELL
    pub side: String,
    /// Size of the trade
    pub size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickSizeChange {
    /// Market or condition ID
    pub market: String,
    /// Array of two ERC1155 asset ID
    pub asset_id: [String; 2],
    /// Previous tick size before the change
    pub old_tick_size: String,
    /// Updated tick size after the change
    pub new_tick_size: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClobMarket {
    /// Market or condition ID
    pub market: String,
    /// Array of two ERC1155 asset ID identifiers associated with market
    pub asset_ids: [String; 2],
    /// Minimum size allowed for an order
    pub min_order_size: String,
    /// Minimum allowable price increment
    pub tick_size: String,
    /// Indicates if the market is negative risk
    pub neg_risk: bool,
}
