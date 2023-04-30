use std::sync::mpsc;

use crate::logcollector::{SessionStartRequest, SessionFinishRequest, SessionFinishReason, TimestampRequest};
use crate::logcollector::log_collector_client::LogCollectorClient;

pub enum ClientRequests {
    ClassLoadStartStamp(f64, String),
    ClassLoadFinishedStamp(f64, String),
    ClassUnloadStartStamp(f64, String),
    ClassUnloadFinishStamp(f64, String),
}

pub enum ControlRequests {
    Shutdown
}

enum ClientStates {
    Pending,
    Running,
    Stopped
}


pub async fn client_routine(pid: u32, rx: mpsc::Receiver<ClientRequests>, ctrl: mpsc::Receiver<ControlRequests>) {
    let mut client = None;
    let mut state = ClientStates::Pending;
    loop {
        let control = ctrl.try_recv();
        match control {
            Ok(request) => match request {
                ControlRequests::Shutdown => {
                    // println!("shutdown consumed");
                    state = ClientStates::Stopped;
                },
                _ => {
                    println!("unknown control request");
                }
            },
            Err(e) => match e {
                mpsc::TryRecvError::Disconnected => {
                    eprintln!("control disconnected");
                },
                _ => ()
            }
        }
        match state {
            ClientStates::Pending => {
                let empty = client.is_none();
                match empty {
                    true => match tonic::transport::Channel::from_static("http://[::1]:50051")
                        .connect()
                        .await {
                            Ok(channel) => {
                                client = Some(LogCollectorClient::new(channel));
                                println!("Connection established");
                            },
                            Err(error) => {
                                eprintln!("Can't establish connection with localhost:50051: {}", error);
                            }
                    },
                    _ => {
                        // try to establish session
                        let response = client.as_mut().unwrap().start_session(SessionStartRequest { pid }).await;
                        match response {
                            Ok(resp) => {
                                let op_response = resp.into_inner();
                                if op_response.is_ok {
                                    println!("session started");
                                    state = ClientStates::Running;
                                } else {
                                    eprintln!("session start refused");
                                }
                            },
                            Err(e) => {
                                eprintln!("failed to start session: {}", e);
                                state = ClientStates::Stopped;
                            }
                        }
                    }
                }
            },
            ClientStates::Running => {
                use ClientRequests::*;
                // process requests from queue
                let result = rx.try_recv();
                match result {
                    Ok(request) => match request {
                        ClassLoadStartStamp(time, payload) => {
                            let response = client.as_mut().unwrap().class_load_start_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                        ClassLoadFinishedStamp(time, payload) => {
                            let response = client.as_mut().unwrap().class_load_finished_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                        ClassUnloadStartStamp(time, payload) => {
                            let response = client.as_mut().unwrap().class_unload_start_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                        ClassUnloadFinishStamp(time, payload) => {
                            let response = client.as_mut().unwrap().class_unload_finished_stamp(TimestampRequest { pid, time, payload }).await;
                        },
                    },
                    Err(error) => match error {
                        mpsc::TryRecvError::Disconnected => {
                            eprintln!("fatal error: disconnected from the main thread: {error}");
                            let response = client.as_mut().unwrap().finish_session(SessionFinishRequest { pid, reason: SessionFinishReason::ServerInterrupted as i32 }).await;
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
    // println!("client stopped");
}