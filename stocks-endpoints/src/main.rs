use std::net::{ TcpListener, TcpStream };
use std::io::{ Read, Write };

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct BuyStocks {
    symbol: String,
    shares: i32,
}

//constants
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//main function
fn main() {
    //start server and print port
    let listener = TcpListener::bind(format!("0.0.0.0:8080")).unwrap();
    println!("Server started at port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

//handle_client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("POST /buy_stocks") => handle_post_request_to_buy_stocks(r),
                r if r.starts_with("POST /sale_stocks") => handle_post_request_to_sale_stocks(r),
                _ => (NOT_FOUND.to_string(), "404 Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

//CONTROLLERS
fn handle_post_request_to_buy_stocks(request: &str) -> (String, String) {
    match get_stock_request_body(&request) {
        Ok(buy_stock) => {
            let resul = common_utils::get_stock_from_nasdaq(buy_stock.symbol.to_string());
            if !resul.success {
                return (INTERNAL_SERVER_ERROR.to_string(), resul.message.to_string());
            }
            common_utils::send_message_to_consumer(buy_stock.symbol.to_string(), buy_stock.shares, "buy".to_string());
            let response = format!("Successfully purchased shares with the symbol: {}", buy_stock.symbol.to_string());
            (OK_RESPONSE.to_string(), response)
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "The body does not mathc with the request".to_string()),
    }
}

fn handle_post_request_to_sale_stocks(request: &str) -> (String, String) {
    match get_stock_request_body(&request) {
        Ok(buy_stock) => {
            let resul = common_utils::get_stock_from_nasdaq(buy_stock.symbol.to_string());
            if !resul.success {
                return (INTERNAL_SERVER_ERROR.to_string(), resul.message.to_string());
            }
            common_utils::send_message_to_consumer(buy_stock.symbol.to_string(), buy_stock.shares, "sale".to_string());
            let response = format!("Successfully purchased shares with the symbol: {}", buy_stock.symbol.to_string());
            (OK_RESPONSE.to_string(), response)
        }
        _ => (INTERNAL_SERVER_ERROR.to_string(), "The body does not mathc with the request".to_string()),
    }
}

//deserialize stocks details from request body with the symbol and shares to buy
fn get_stock_request_body(request: &str) -> Result<BuyStocks, serde_json::Error> {
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}