extern crate curl;
extern crate serde_json;

use curl::http;
use serde_json::Value;
use serde::{Deserialize, Serialize};

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
    let resp = http::handle()
        .get(url)
        .exec()
        .unwrap_or_else(|e| {
            panic!("Failed to get Stock; error is {}", e);
        });

    if resp.get_code() != 200 {
        let message = format!("Unable to handle HTTP response code {}", resp.get_code());
        return CustomResult {
            stock: None,
            success: false,
            message: message,
        };
    }

    let body: &str = std::str::from_utf8(resp.get_body()).unwrap_or_else(|e| {
        panic!("Failed to parse response from STOCKS; error is {}", e);
    });
    let object: Value = serde_json::from_str(body).unwrap();
    let stock_code: StockCode = serde_json::from_value(object).unwrap();

    if stock_code.status.rCode != 200 {
        return CustomResult {
            stock: None,
            success: false,
            message: "Symbol not exists".to_string(),
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
