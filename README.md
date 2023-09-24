# Buy/Sale Stocks
## Stack of technologies
That are some of main technologies used in the project:
- [Rust](https://www.rust-lang.org/)
- [Async-graphql](https://async-graphql.github.io/async-graphql/en/index.html)
- [Apache Kafka](https://kafka.apache.org/)
- [Docker Compose](https://docs.docker.com/compose/)
- [PostgreSQL](https://www.postgresql.org/)
## Prerequisites
To run the project locally you only need Docker Compose. Without Docker, you might need to install the following:
- [Rust](https://www.rust-lang.org/tools/install)
- [Diesel CLI](https://diesel.rs/guides/getting-started.html)
- [CMake](https://cmake.org/install/) 
- [PostgreSQL](https://www.postgresql.org/download/)
- [Apache Kafka](https://kafka.apache.org/quickstart)
- [npm](https://docs.npmjs.com/getting-started)
## Clone the Repository 
```bash
$ git clone git@github.com:ppzzmm/rust-pzm-project.git && cd rust-pzm-project
```
## Run project
### With Docker
We have two options:
- Using locally built images:
```bash
$ docker-compose up --build
```
- Using released images:
```bash
$ docker-compose -f docker-compose.yml up
```
### Without Docker
#### Setting Kafka and Zookeeper
- First, download the latest Kafka release [here](https://www.apache.org/dyn/closer.cgi?path=/kafka/3.2.1/kafka_2.13-3.2.1.tgz)
- Extract the compressed file and open it, after that, we have to start the ZooKeeper server with this command:
```bash
$ bin/zookeeper-server-start.sh config/zookeeper.properties
```
- Then, open another terminal session in the decompressed folder and start the Kafka broker:
```bash
$ bin/kafka-server-start.sh config/server.properties
```
- Next we have to create a kafka topic and setting producers and consumers to publish our events.
  - On another terminal exec this command to create a topic:
```bash
$ bin/kafka-topics.sh --create --topic topic-stocks --bootstrap-server localhost:9092
```
- To open the consumer and producer that kafka provided, run this commands:
```bash
$ bin/kafka-console-consumer.sh --topic topic-stocks --from-beginning --bootstrap-server localhost:9092
$ bin/kafka-console-producer.sh --topic topic-stocks --bootstrap-server localhost:9092
```
#### Run the applications and services
- Placed within the project, first we have to run the **stocks-service** project because that contain the migrations to create the database tables, in this project we have the **GrapHQL** endpoints to get the information about the stocks:
```bash
$ cargo run -p stocks-service
```
- Placed inside the project but in another terminal, run the following command to load the **consumer-stocks-service** service, here we have the kafka consumer to process the stocks that the user bought or sold:
```bash
$ cargo run -p consumer-stocks-service
```
- To finish the project launch we have this **Rest API** in rust to buy or sale stocks, this endpoints send an event in the kafka topic to the consumer (**consumer-stocks-service**) process the information:
```bash
$ cargo run -p stocks-endpoints 
```
### Testing
- [Here](https://documenter.getpostman.com/view/2220937/2s9YJW55ye#4a2e2bf0-07ee-4066-84a8-db120f3dfb96) you can see how to run the services in postman:
  <img width="1657" alt="Screenshot 2023-09-23 at 23 47 41" src="https://github.com/ppzzmm/rust-pzm-project/assets/29339482/a9b7ab8e-031e-4c8f-9fe3-9c27e7c0b78f">
- If you already used the endpoints to buy or sale stocks, page this command Curl in a terminal to see the information:
```bash
$curl 'http://localhost:8001/stocks' -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H 'Origin: http://localhost:8001' --data-binary '{"query":"{\n  stocksSummary {\n    symbol\n    profitLoss\n    shares\n    totalValue\n    lowestPrice\n    highestPrice\n    averagePrice\n    priceByHours\n  }\n}"}' --compressed
```
- Or open your browser in this URL [http://localhost:8001/stocks](http://localhost:8001/stocks) and page this query to see the information about your stocks:
```bash
{
  stocksSummary {
    symbol
    profitLoss
    shares
    totalValue
    lowestPrice
    highestPrice
    averagePrice
    priceByHours
  }
}

or to buy stocks with mutation option:

mutation{
  buyStocks(
    stock: {
      symbol: "APP",
      shares: 212
    }
  )
  {
    id
  }
}
```

![Screen Recording 2023-09-23 at 23 31 26](https://github.com/ppzzmm/rust-pzm-project/assets/29339482/5fa898b7-4e43-44be-a68f-44c9c0c7a754)

