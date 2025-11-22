use serde::{Deserialize, Serialize};
use std::fmt;

/// API key credentials for CLOB authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClobApiKeyCreds {
    /// API key used for authentication
    pub key: String,

    /// API secret associated with the key
    pub secret: String,

    /// Passphrase required for authentication
    pub passphrase: String,
}

/// Authentication details for Gamma authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GammaAuth {
    /// Address used for authentication
    pub address: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Topic {
    /// Activity topic - trades and order matches
    Activity,
    /// Comments and reactions
    Comments,
    /// Request for Quote
    Rfq,
    /// Crypto price updates
    CryptoPrices,
    /// Chainlink crypto price updates
    CryptoPricesChainlink,
    /// Equity price updates
    EquityPrices,
    /// CLOB user updates (requires authentication)
    ClobUser,
    /// CLOB market updates
    ClobMarket,
}

impl Topic {
    pub fn as_str(&self) -> &'static str {
        match self {
            Topic::Activity => "activity",
            Topic::Comments => "comments",
            Topic::Rfq => "rfq",
            Topic::CryptoPrices => "crypto_prices",
            Topic::CryptoPricesChainlink => "crypto_prices_chainlink",
            Topic::EquityPrices => "equity_prices",
            Topic::ClobUser => "clob_user",
            Topic::ClobMarket => "clob_market",
        }
    }
}

impl fmt::Display for Topic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for Topic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "activity" => Ok(Topic::Activity),
            "comments" => Ok(Topic::Comments),
            "rfq" => Ok(Topic::Rfq),
            "crypto_prices" => Ok(Topic::CryptoPrices),
            "crypto_prices_chainlink" => Ok(Topic::CryptoPricesChainlink),
            "equity_prices" => Ok(Topic::EquityPrices),
            "clob_user" => Ok(Topic::ClobUser),
            "clob_market" => Ok(Topic::ClobMarket),
            _ => Err(serde::de::Error::custom(format!("Unknown topic: {}", s))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MessageType {
    /// All message types (wildcard)
    All,
    /// Activity
    Trades,
    OrdersMatched,
    /// Comments
    CommentCreated,
    CommentRemoved,
    ReactionCreated,
    ReactionRemoved,
    /// RFQ
    RequestCreated,
    RequestEdited,
    RequestCanceled,
    RequestExpired,
    QuoteCreated,
    QuoteEdited,
    QuoteCanceled,
    QuoteExpired,
    /// Prices
    Update,
    /// CLOB User
    Order,
    Trade,
    /// CLOB Market
    PriceChange,
    AggOrderbook,
    LastTradePrice,
    TickSizeChange,
    MarketCreated,
    MarketResolved,
}

impl MessageType {
    pub fn as_str(&self) -> &'static str {
        match self {
            MessageType::All => "*",
            MessageType::Trades => "trades",
            MessageType::OrdersMatched => "orders_matched",
            MessageType::CommentCreated => "comment_created",
            MessageType::CommentRemoved => "comment_removed",
            MessageType::ReactionCreated => "reaction_created",
            MessageType::ReactionRemoved => "reaction_removed",
            MessageType::RequestCreated => "request_created",
            MessageType::RequestEdited => "request_edited",
            MessageType::RequestCanceled => "request_canceled",
            MessageType::RequestExpired => "request_expired",
            MessageType::QuoteCreated => "quote_created",
            MessageType::QuoteEdited => "quote_edited",
            MessageType::QuoteCanceled => "quote_canceled",
            MessageType::QuoteExpired => "quote_expired",
            MessageType::Update => "update",
            MessageType::Order => "order",
            MessageType::Trade => "trade",
            MessageType::PriceChange => "price_change",
            MessageType::AggOrderbook => "agg_orderbook",
            MessageType::LastTradePrice => "last_trade_price",
            MessageType::TickSizeChange => "tick_size_change",
            MessageType::MarketCreated => "market_created",
            MessageType::MarketResolved => "market_resolved",
        }
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "*" => Ok(MessageType::All),
            "trades" => Ok(MessageType::Trades),
            "orders_matched" => Ok(MessageType::OrdersMatched),
            "comment_created" => Ok(MessageType::CommentCreated),
            "comment_removed" => Ok(MessageType::CommentRemoved),
            "reaction_created" => Ok(MessageType::ReactionCreated),
            "reaction_removed" => Ok(MessageType::ReactionRemoved),
            "request_created" => Ok(MessageType::RequestCreated),
            "request_edited" => Ok(MessageType::RequestEdited),
            "request_canceled" => Ok(MessageType::RequestCanceled),
            "request_expired" => Ok(MessageType::RequestExpired),
            "quote_created" => Ok(MessageType::QuoteCreated),
            "quote_edited" => Ok(MessageType::QuoteEdited),
            "quote_canceled" => Ok(MessageType::QuoteCanceled),
            "quote_expired" => Ok(MessageType::QuoteExpired),
            "update" => Ok(MessageType::Update),
            "order" => Ok(MessageType::Order),
            "trade" => Ok(MessageType::Trade),
            "price_change" => Ok(MessageType::PriceChange),
            "agg_orderbook" => Ok(MessageType::AggOrderbook),
            "last_trade_price" => Ok(MessageType::LastTradePrice),
            "tick_size_change" => Ok(MessageType::TickSizeChange),
            "market_created" => Ok(MessageType::MarketCreated),
            "market_resolved" => Ok(MessageType::MarketResolved),
            _ => Err(serde::de::Error::custom(format!(
                "Unknown message type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SubscriptionFilter {
    EventSlug {
        event_slug: String,
    },
    MarketSlug {
        market_slug: String,
    },
    ParentEntity {
        #[serde(rename = "parentEntityID")]
        parent_entity_id: u64,
        #[serde(rename = "parentEntityType")]
        parent_entity_type: String,
    },
    Symbol {
        symbol: String,
    },
    TokenIds(Vec<String>),
}

impl SubscriptionFilter {
    pub fn event_slug(slug: impl Into<String>) -> Self {
        SubscriptionFilter::EventSlug {
            event_slug: slug.into(),
        }
    }

    pub fn market_slug(slug: impl Into<String>) -> Self {
        SubscriptionFilter::MarketSlug {
            market_slug: slug.into(),
        }
    }

    pub fn parent_entity(id: u64, entity_type: impl Into<String>) -> Self {
        SubscriptionFilter::ParentEntity {
            parent_entity_id: id,
            parent_entity_type: entity_type.into(),
        }
    }

    pub fn symbol(symbol: impl Into<String>) -> Self {
        SubscriptionFilter::Symbol {
            symbol: symbol.into(),
        }
    }

    pub fn token_ids(ids: Vec<String>) -> Self {
        SubscriptionFilter::TokenIds(ids)
    }

    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Subscription details for a topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    /// Topic to subscribe to
    pub topic: Topic,

    /// Type of subscription
    #[serde(rename = "type")]
    pub subscription_type: MessageType,

    /// Optional filters for the subscription
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<String>,

    /// Optional CLOB authentication credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clob_auth: Option<ClobApiKeyCreds>,

    /// Optional Gamma authentication credentials
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma_auth: Option<GammaAuth>,
}

impl Subscription {
    /// Create a new subscription with typed topic and message type
    pub fn new(topic: Topic, message_type: MessageType) -> Self {
        Self {
            topic,
            subscription_type: message_type,
            filters: None,
            clob_auth: None,
            gamma_auth: None,
        }
    }

    /// Set the filter for this subscription
    pub fn with_filter(mut self, filter: SubscriptionFilter) -> Result<Self, serde_json::Error> {
        self.filters = Some(filter.to_json_string()?);
        Ok(self)
    }

    /// Set the filter from a raw JSON string
    pub fn with_raw_filter(mut self, filter: String) -> Self {
        self.filters = Some(filter);
        self
    }

    /// Set CLOB authentication credentials
    pub fn with_clob_auth(mut self, auth: ClobApiKeyCreds) -> Self {
        self.clob_auth = Some(auth);
        self
    }

    /// Set Gamma authentication credentials
    pub fn with_gamma_auth(mut self, auth: GammaAuth) -> Self {
        self.gamma_auth = Some(auth);
        self
    }
}

/// Message structure for subscription requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMessage {
    /// Array of subscriptions to subscribe to
    pub subscriptions: Vec<Subscription>,
}

/// Represents a real-time message received from the WebSocket server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Topic of the message
    pub topic: Topic,

    /// Type of the message
    #[serde(rename = "type")]
    pub message_type: MessageType,

    /// Timestamp of when the message was sent
    pub timestamp: u64,

    /// Payload containing the message data
    pub payload: serde_json::Value,

    /// Connection ID
    pub connection_id: String,
}

/// Represents websocket connection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    /// Attempting to establish a connection
    #[serde(rename = "CONNECTING")]
    Connecting,

    /// Successfully connected
    #[serde(rename = "CONNECTED")]
    Connected,

    /// Disconnected from the server
    #[serde(rename = "DISCONNECTED")]
    Disconnected,
}
