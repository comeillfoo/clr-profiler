use std::error::Error;
use std::sync::mpsc;

use clr_profiler::CorProfilerCallback;
use perf_monitor::cpu::{processor_numbers, ProcessStat};
use perf_monitor::io::get_process_io_stats;
use tonic::transport::Channel;

use crate::logcollector::{SessionStartRequest, SessionFinishRequest, SessionFinishReason,
    TimestampRequest, TimestampIdRequest, CommonStatistics,
    ObjectAllocatedStampRequest, ObjectGeneration, UpdateGenerationsRequest, OptionUint32};
use crate::logcollector::log_collector_client::LogCollectorClient;

pub enum ClientRequests {
    ClassLoadStartStamp(f64, String),
    ClassLoadFinishedStamp(f64, String),
    ClassUnloadStartStamp(f64, String),
    ClassUnloadFinishStamp(f64, String),
    ThreadCreatedStamp(f64, u64),
    ThreadDestroyedStamp(f64, u64),
    ThreadResumedStamp(f64, u64),
    ThreadSuspendedStamp(f64, u64),
    ExceptionThrownStamp(f64, String),
    JitCompilationStartStamp(f64, String),
    JitCompilationFinishStamp(f64, String),
    ObjectAllocatedStamp(f64, u64, u64, String, Option<u32>),
    GenerationsUpdateStamp(f64, Vec<(u64, Option<u32>)>),
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

fn get_stats() -> Option<CommonStatistics> {
    let cores = match processor_numbers() {
        Ok(n) => n,
        Err(_) => return None
    };
    let mut process_stat = match ProcessStat::cur() {
        Ok(cur) => cur,
        Err(_) => return None
    };
    let denormalized_cpu = match process_stat.cpu() {
        Ok(cpu) => cpu * 100f64,
        Err(_) => return None
    };
    let cpu = denormalized_cpu / (cores as f64);
    let io_stat = get_process_io_stats().unwrap();
    Some(CommonStatistics { cpu, read_bytes: io_stat.read_bytes, write_bytes: io_stat.write_bytes })
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
                        match request {
                            ClassLoadStartStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_load_start_stamp(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            ClassLoadFinishedStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_load_finished_stamp(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            ClassUnloadStartStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_unload_start_stamp(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            ClassUnloadFinishStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .class_unload_finished_stamp(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            ThreadCreatedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_created(TimestampIdRequest { pid, time, id, stats: get_stats() }).await;
                            },
                            ThreadDestroyedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_destroyed(TimestampIdRequest { pid, time, id, stats: get_stats() }).await;
                            },
                            ThreadResumedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_resumed(TimestampIdRequest { pid, time, id, stats: get_stats() }).await;
                            },
                            ThreadSuspendedStamp(time, id) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .thread_suspended(TimestampIdRequest { pid, time, id, stats: get_stats() }).await;
                            },
                            ExceptionThrownStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .exception_thrown(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            JitCompilationStartStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .jit_compilation_start_stamp(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            JitCompilationFinishStamp(time, payload) => {
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .jit_compilation_finished_stamp(TimestampRequest { pid, time, payload, stats: get_stats() }).await;
                            },
                            ObjectAllocatedStamp(time, id, size, class_name, generation) => {
                                let (value, is_valid) = match generation {
                                    Some(value) => (value, true),
                                    None => (42, false)
                                };
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .object_allocation_stamp(ObjectAllocatedStampRequest {
                                        pid,
                                        time,
                                        object_gen: Some(ObjectGeneration {
                                            id,
                                            generation: Some(OptionUint32 { is_valid, value }) }),
                                        size,
                                        class_name,
                                        stats: get_stats()
                                    }).await;
                            },
                            GenerationsUpdateStamp(time, updates) => {
                                let objects = updates.iter().map(|object| {
                                    let (value, is_valid) = match object.1 {
                                        Some(value) => (value, true),
                                        None => (42, false)
                                    };
                                    ObjectGeneration { id: object.0, generation: Some(OptionUint32 { is_valid, value }) }
                                }).collect();
                                let response = client
                                    .as_mut()
                                    .unwrap()
                                    .garbage_collection_finished_stamp(UpdateGenerationsRequest {
                                        pid,
                                        time,
                                        objects
                                    }).await;
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