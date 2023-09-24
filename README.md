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
- In your brouser open https://localhost:8080/stocks

![Screen Recording 2023-09-23 at 23 31 26](https://github.com/ppzzmm/rust-pzm-project/assets/29339482/5fa898b7-4e43-44be-a68f-44c9c0c7a754)

## Documentation

## Instalation

## User guide

##

