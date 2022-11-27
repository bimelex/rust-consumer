mod service;

use std::alloc::System;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use serde::{Serialize, Deserialize};
use kafka::error::Error as KafkaError;
use serde_json;
use chrono::{DateTime, Utc, Local, TimeZone};
use std::thread;
use threadpool::ThreadPool;
use std::str;
use std::str::Utf8Error;
use std::thread::Thread;
use std::time;
use std::time::{Duration, UNIX_EPOCH};

use service::*;

fn main() {
    env_logger::init();

    thread::spawn(|| {

        let broker : String = "localhost:9093".to_owned();
        let real_time_topic = "realtimeperf".to_owned();
        let group : String = "ontune".to_owned();

        if let Err(e1) =  perf::real_time_message(group, real_time_topic, vec![broker]) {
            println!("Failed Real Time messages: {}", e1);
        }
    });

    thread::spawn(|| {
        let broker : String = "localhost:9093".to_owned();
        let test_topic = "test".to_owned();
        let group : String = "ontune".to_owned();

        if let Err(e2) =  tsts::test_consumer(group, test_topic, vec![broker]) {
            println!("Failed test messages: {}", e2);
        }
    });

    let broker : String = "localhost:9093".to_owned();
    let agent_info_topic = "agentinfo".to_owned();
    let group : String = "ontune".to_owned();

    if let Err(e3) =  agent::agent_info_message(group, agent_info_topic, vec![broker]) {
        println!("Failed Agent Info messages: {}", e3);
    }

}