use crate::stock_functions::{save_stock, calculate_stock_summary};
pub mod stock_functions;
pub mod persistence;

pub fn buy_stocks(symbol: String, shares: String, action: String) {
    let result = common_utils::get_stock_from_nasdaq(symbol.to_string());
    if result.success == false {
        return;
    }
    let stock_from_nasdaq = result.stock.unwrap();
    let stock_data = stock_from_nasdaq.data;
    let last_sale_price = stock_data.primaryData.lastSalePrice.replace("$", "");
    let bid_price = stock_data.primaryData.bidPrice.replace("$", "");
    let price = if bid_price != "N/A" {
        bid_price
    } else {
        last_sale_price
    };
    let percentage_change = stock_data.primaryData.percentageChange
        .replace("%", "")
        .replace("+", "");
    save_stock(
        symbol.to_string(), 
        shares.parse::<i32>().unwrap(), 
        price.to_string(), 
        percentage_change.to_string(),
        action.to_string());
    calculate_stock_summary(
        symbol.to_string(),
        shares.parse::<i32>().unwrap(),
        price.to_string(),
        percentage_change.to_string()
    );
}

