#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Coin {
    BTC,
    ETH,
    SOL,
}


impl Coin {
   pub  fn from_binance_symbol(s: &str) -> Option<Self> {
        match s {
            "BTCUSDT" => Some(Coin::BTC),
            "ETHUSDT" => Some(Coin::ETH),
            "SOLUSDT" => Some(Coin::SOL),
            _ => None,
        }
    }

    pub fn from_hyperliquid(s: &str) -> Option<Self> {
        match s {
            "BTC" => Some(Coin::BTC),
            "ETH" => Some(Coin::ETH),
            "SOL" => Some(Coin::SOL),
            _ => None,
        }
    }
}


use std::collections::HashMap;

use serde::Deserialize;

#[derive(Default, Debug)]
pub struct Prices {
    pub  binance: HashMap<Coin, f64>,
   pub   hyperliquid: HashMap<Coin, f64>,
}



#[derive(Debug, Deserialize)]
pub struct BinanceCombinedMsg {
    pub stream: String,
    pub   data: BinanceTrade,
}

#[derive(Debug, Deserialize)]
pub struct BinanceTrade {
    pub  p: String,
    pub  s: String, // symbol, e.g. BTCUSDT
}


// hyper liquid parsing .. 

#[derive(Debug, Deserialize)]
pub struct HlTradesMsg {
    pub data: Vec<HlTrade>,
}

#[derive(Debug, Deserialize)]
pub struct HlTrade {
    pub coin: String,
    pub px: String,
}
