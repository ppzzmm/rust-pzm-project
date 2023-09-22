use kafka::consumer::{Consumer, FetchOffset};
use std::str;

fn main() {
    let hosts = vec!["kafka:9092".to_owned()];
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
        }
        let _ = consumer.consume_messageset(ms);
      }
      consumer.commit_consumed().unwrap();
    }
}