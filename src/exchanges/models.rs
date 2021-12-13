use crate::configuration::APP_CONFIG;

/// Available exchanges for the challenge
/*
 Note
 ----
 In a real situation, i would not use this hardcoded enum to get the endpoints, but rather write a
 .ron file gathering all endpoints and store it in the "live configuration" store (see store.rs).

 The goal would be to add(/remove) exchanges on the fly, if needed. (Otherwise this solution is viable)
 */
#[derive(Debug)]
#[repr(u32)]
pub enum ExchangeEndpoint {
    Binance = 0,
    Bitstamp = 1,
}

impl std::fmt::Display for ExchangeEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let currency_pair = APP_CONFIG.currency_pair.clone().to_lowercase();

        let exchange_endpoint: String = match *self {
            ExchangeEndpoint::Binance => format!("wss://stream.binance.com:9443/ws/{}@depth20@100ms", currency_pair),
            ExchangeEndpoint::Bitstamp => format!("wss://ws.bitstamp.net/"),
        };

        write!(f, "{}", exchange_endpoint)
    }
}
