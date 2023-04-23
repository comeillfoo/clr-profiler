use std::sync::mpsc;

use crate::logcollector::{SessionStartRequest, SessionFinishRequest, SessionFinishReason, TimestampRequest};
use crate::logcollector::log_collector_client::LogCollectorClient;

pub enum ClientRequests {
    ClassLoadStartStamp(f64, String),
    ClassLoadFinishedStamp(f64, String),
    ClassUnloadStartStamp(f64, String),
    ClassUnloadFinishStamp(f64, String),
}

enum ClientStates {
    Pending,
    Running,
    Stopped
}

pub async fn client_routine(pid: u32, rx: mpsc::Receiver<ClientRequests>) {
    let mut channel = None;
    while channel.is_none() {
        match tonic::transport::Channel::from_static("localhost:50051")
            .connect()
            .await {
                Ok(connection) => {
                    channel = Some(connection);
                },
                Err(error) => {
                    eprintln!("Can't establish connection with localhost:50051: {error}");
                }
        }
    }
    let mut client = LogCollectorClient::new(channel.unwrap());

    let mut state = ClientStates::Pending;
    loop {
        match state {
            ClientStates::Pending => {
                // try to establish session
                let response = client.start_session(SessionStartRequest { pid }).await;
            },
            ClientStates::Running => {
                use ClientRequests::*;
                // process requests from queue
                let result = rx.try_recv();
                match result {
                    Ok(request) => match request {
                        ClassLoadStartStamp(time, payload) => {
                            let response = client.class_load_start_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                        ClassLoadFinishedStamp(time, payload) => {
                            let response = client.class_load_finished_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                        ClassUnloadStartStamp(time, payload) => {
                            let response = client.class_unload_start_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                        ClassUnloadFinishStamp(time, payload) => {
                            let response = client.class_unload_finished_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                    },
                    Err(error) => match error {
                        mpsc::TryRecvError::Disconnected => {
                            eprintln!("fatal error: disconnected from the main thread: {error}");
                            let response = client.finish_session(SessionFinishRequest { pid, reason: SessionFinishReason::ServerInterrupted as i32 }).await;
                            state = ClientStates::Stopped;
                        },
                        _ => ()
                    }
                }
            },
            ClientStates::Stopped => {
                break;
            }
        }
    }
}