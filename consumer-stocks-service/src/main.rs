extern crate consumer_stocks_service;

use kafka::consumer::{Consumer, FetchOffset};
use std::str;
use consumer_stocks_service::{buy_stocks};

fn main() {
    let url_kafka = "kafka:9092".to_string();
    // let url_kafka = "localhost:9092".to_string();
    let hosts = vec![url_kafka.to_owned()];
    let mut consumer =
       Consumer::from_hosts(hosts)
          .with_topic("topic-stocks".to_owned())
          .with_fallback_offset(FetchOffset::Earliest)
          .create()
          .unwrap();
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          println!(";) ;) : {:?}", str::from_utf8(m.value).unwrap());
          let parts = str::from_utf8(m.value).unwrap().split(",");
          let collection = parts.collect::<Vec<&str>>();
          if collection.len() == 2 {
            let symbol = collection.first().unwrap();
            let shares = collection.last().unwrap();
            buy_stocks(symbol.to_string(), shares.to_string());
          }
        }
        let _ = consumer.consume_messageset(ms);
      }
      consumer.commit_consumed().unwrap();
    }
}