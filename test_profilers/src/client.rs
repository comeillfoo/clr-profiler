use std::error::Error;
use std::sync::mpsc;

use perf_monitor::cpu::{processor_numbers, ProcessStat};
use perf_monitor::io::get_process_io_stats;
use tonic::transport::Channel;

use crate::logcollector::{SessionStartRequest, SessionFinishRequest, SessionFinishReason, TimestampRequest, TimestampIdRequest, CommonStatisticsRequest};
use crate::logcollector::log_collector_client::LogCollectorClient;

pub enum ClientRequests {
    ClassLoadStartStamp(f64, String),
    ClassLoadFinishedStamp(f64, String),
    ClassUnloadStartStamp(f64, String),
    ClassUnloadFinishStamp(f64, String),
    ThreadCreatedStamp(f64, u64),
    ThreadDestroyedStamp(f64, u64),
    ThreadResumedStamp(f64, u64),
    ThreadSuspendedStamp(f64, u64)
}

pub enum ControlRequests {
    Shutdown
}

enum ClientStates {
    Pending,
    Running,
    Stopped
}

pub fn get_time() -> f64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

async fn send_stats(pid: u32, client: &mut Option<LogCollectorClient<Channel>>) -> Result<(), std::io::Error> {
    let cores = processor_numbers()?;
    let mut process_stat = ProcessStat::cur()?;
    let denormalized_cpu = process_stat.cpu()? * 100f64;
    let cpu = denormalized_cpu / (cores as f64);
    let io_stat = get_process_io_stats().unwrap();
    let response = client
        .as_mut()
        .unwrap()
        .stat(CommonStatisticsRequest {
            pid,
            time: get_time(),
            cpu,
            read_bytes: io_stat.read_bytes,
            write_bytes: io_stat.write_bytes })
        .await;
    Ok(())
}


pub async fn client_routine(pid: u32, cmd: String, path: String, rx: mpsc::Receiver<ClientRequests>, ctrl: mpsc::Receiver<ControlRequests>) {
    let mut client: Option<LogCollectorClient<Channel>>= None;
    let mut state = ClientStates::Pending;
    loop {
        let control = ctrl.try_recv();
        match control {
            Ok(request) => match request {
                ControlRequests::Shutdown => {
                    if client.is_some() {
                        let response = client
                            .as_mut()
                            .unwrap()
                            .finish_session(SessionFinishRequest {
                                pid,
                                reason: SessionFinishReason::ClientInterrupted as i32 })
                            .await;
                    }
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
                        let response = client
                            .as_mut()
                            .unwrap()
                            .start_session(SessionStartRequest {
                                pid,
                                cmd: cmd.clone(),
                                path: path.clone() })
                            .await;
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
                    Ok(request) => {
                        let response = send_stats(pid, &mut client).await;
                        match request {
                            ClassLoadStartStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_load_start_stamp(TimestampRequest { pid, time, payload }).await;
                            },
                            ClassLoadFinishedStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_load_finished_stamp(TimestampRequest { pid, time, payload }).await;
                            },
                            ClassUnloadStartStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_unload_start_stamp(TimestampRequest { pid, time, payload }).await;
                            },
                            ClassUnloadFinishStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_unload_finished_stamp(TimestampRequest { pid, time, payload }).await;
                            },
                            ThreadCreatedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_created(TimestampIdRequest { pid, time, id }).await;
                            },
                            ThreadDestroyedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_destroyed(TimestampIdRequest { pid, time, id }).await;
                            },
                            ThreadResumedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_resumed(TimestampIdRequest { pid, time, id }).await;
                            },
                            ThreadSuspendedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_suspended(TimestampIdRequest { pid, time, id }).await;
                            }
                        }
                    },
                    Err(error) => match error {
                        mpsc::TryRecvError::Disconnected => {
                            eprintln!("fatal error: disconnected from the main thread: {error}");
                            let response = client
                                .as_mut()
                                .unwrap()
                                .finish_session(SessionFinishRequest { pid, reason: SessionFinishReason::ServerInterrupted as i32 }).await;
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