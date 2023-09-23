// extern crate curl;
extern crate serde_json;

use curl::easy::{Easy2, Handler, WriteError};
struct Collector(Vec<u8>);
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}
use serde_json::Value;
use serde::{Deserialize, Serialize};
use kafka::producer::{Producer, Record};
#[derive(Debug, Serialize, Deserialize)]
pub struct StockCode {
    pub status: StockStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stock {
    pub data: StockData,
    pub status: StockStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StockStatus {
    pub rCode: i32,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct StockData {
    pub symbol: String,
    pub companyName: String,
    pub stockType: String,
    pub exchange: String,
    pub isNasdaqListed: bool,
    pub isNasdaq100: bool,
    pub isHeld: bool,
    pub primaryData: ComplementData,
    pub secondaryData: Option<ComplementData>,
    pub marketStatus: String,
    pub assetClass: String,
    pub keyStats: KeyStats,
    pub notifications: Vec<Notifications>
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ComplementData {
    pub lastSalePrice: String,
    pub netChange: String,
    pub percentageChange: String,
    pub deltaIndicator: String,
    pub lastTradeTimestamp: String,
    pub isRealTime: bool,
    pub bidPrice: String,
    pub askPrice: String,
    pub bidSize: String,
    pub askSize: String,
    pub volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyStats {
    pub fiftyTwoWeekHighLow: DefaultData,
    pub dayrange: DefaultData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DefaultData {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Notifications {
    pub headline: String,
    pub eventTypes: Vec<EventTypes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventTypes {
    pub message: String,
    pub eventName: String,
    pub url: DefaultData,
    pub id: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomResult {
    pub stock: Option<Stock>,
    pub success: bool,
    pub message: String,
}

pub fn get_stock_from_nasdaq(symbol: String) -> CustomResult  {
    let url = format!("https://api.nasdaq.com/api/quote/{}/info?assetclass=stocks", symbol);
    let mut easy = Easy2::new(Collector(Vec::new()));
    easy.get(true).unwrap();
    easy.url(&url).unwrap();
    easy.perform().unwrap();
    assert_eq!(easy.response_code().unwrap(), 200);
    let contents = easy.get_ref();

    let body: &str = std::str::from_utf8(&contents.0).unwrap_or_else(|e| {
        panic!("Failed to parse response from; error is {}", e);
    });
    let object: Value = serde_json::from_str(body).unwrap();
    let stock_code: StockCode = serde_json::from_value(object).unwrap();

    if stock_code.status.rCode != 200 {
        return CustomResult {
            stock: None,
            success: false,
            message: "Symbol not exists PZM 22121212".to_string(),
        };
    }

    let object: Value = serde_json::from_str(body).unwrap();
    let stock: Stock = serde_json::from_value(object).unwrap();
    return CustomResult {
        stock: Some(stock),
        success: true,
        message: "success!".to_string(),
    };
}

pub fn send_message_to_consumer(symbol: String, shares: i32) {
    // let url_kafka = "localhost:9092".to_string();
    let url_kafka = "kafka:9092".to_string();
    let hosts = vec![url_kafka.to_owned()];
    let mut producer =
    Producer::from_hosts(hosts)
        .create()
        .unwrap();

    let buf = format!("{},{}", symbol, shares);
    producer.send(&Record::from_value("topic-stocks", buf.as_bytes())).unwrap();
    println!("Symbol: {symbol}, Shares: {shares}");
}
