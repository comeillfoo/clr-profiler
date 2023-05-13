use clr_profiler::{
    cil::{nop, Method},
    ffi::{CorOpenFlags, FunctionID, COR_PRF_MONITOR, E_FAIL, HRESULT},
    register, ClrProfiler, CorProfilerCallback, CorProfilerCallback2, CorProfilerCallback3,
    CorProfilerCallback4, CorProfilerCallback5, CorProfilerCallback6, CorProfilerCallback7,
    CorProfilerCallback8, CorProfilerCallback9, CorProfilerInfo, MetadataImportTrait, ProfilerInfo, CorProfilerInfo2,
};
use std::{slice, sync::mpsc::{Sender, SendError}, error::Error};
use std::process;
use uuid::Uuid;
use std::sync::mpsc;
use std::env;

use crate::client::{client_routine, ClientRequests, ControlRequests, get_time};

fn client_lost<T>(e: std::sync::mpsc::SendError<T>) -> Result<(), FFI_HRESULT> {
    eprintln!("connection with gRPC-client lost: {}", e);
    Err(E_FAIL)
}

#[derive(Clone)]
struct Profiler {
    clsid: Uuid,
    profiler_info: Option<ProfilerInfo>,
    tx: Option<Sender<ClientRequests>>,
    ctrl: Option<Sender<ControlRequests>>,
    client: Option<std::rc::Rc<std::thread::JoinHandle<()>>>
}
impl Profiler {
    fn profiler_info(&self) -> &ProfilerInfo {
        self.profiler_info.as_ref().unwrap()
    }

    fn send_request(tx: &Option<Sender<ClientRequests>>, request: ClientRequests) -> Result<(), FFI_HRESULT> {
        match tx.as_ref()
            .unwrap()
            .send(request) {
            Ok(_) => Ok(()),
            Err(e) => client_lost(e)
        }
    }

    fn get_class_name(&self, class_id: clr_profiler::ffi::ClassID) -> Result<String, FFI_HRESULT> {
        let class_info = self.profiler_info().get_class_id_info(class_id)?;
        let module_metadata = self
            .profiler_info()
            .get_module_metadata(class_info.module_id, CorOpenFlags::ofRead)?;
        let class_props = module_metadata.get_typedef_props(class_info.token)?;
        Ok(class_props.name)
    }

    fn get_method_name(&self, function_id: FunctionID) -> Result<String, FFI_HRESULT> {
        let function_info = self.profiler_info().get_function_info(function_id)?;
        let module_metadata = self
            .profiler_info()
            .get_module_metadata(function_info.module_id, CorOpenFlags::ofRead)?;
        let method_props = module_metadata.get_method_props(function_info.token)?;
        Ok(method_props.name)
    }
}
impl ClrProfiler for Profiler {
    fn new() -> Profiler {
        Profiler {
            clsid: Uuid::parse_str("DF63A541-5A33-4611-8829-F4E495985EE3").unwrap(),
            profiler_info: None,
            tx: None,
            ctrl: None,
            client: None
        }
    }
    fn clsid(&self) -> &Uuid {
        &self.clsid
    }
}
impl CorProfilerCallback for Profiler {
    fn initialize(&mut self, profiler_info: ProfilerInfo) -> Result<(), HRESULT> {
        // Initialize ICorProfilerInfo reference
        self.profiler_info = Some(profiler_info);

        // Set the event mask
        self.profiler_info()
            .set_event_mask(COR_PRF_MONITOR::COR_PRF_ALL)?; // COR_PRF_MONITOR_JIT_COMPILATION

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        self.client = Some(std::rc::Rc::new(std::thread::spawn(move || {
            match tokio::runtime::Runtime::new() {
                Ok(rt) => {
                    rt.block_on(async {
                        println!("client started");
                        let path = match env::var("PATH") {
                            Ok(p) => p,
                            Err(e) => String::from("")
                        };
                        client_routine(
                            process::id(),
                            env::args().collect::<Vec<String>>().join(" "),
                            path,
                            rx1,
                            rx2).await;
                    });
                },
                Err(e) => {
                    eprintln!("can't run gRPC client: {}", e);
                }
            }
        })));
        self.tx = Some(tx1);
        self.ctrl = Some(tx2);
        Ok(())
    }

    // jit handlers: START
    fn jit_compilation_started(
        &mut self,
        function_id: FunctionID,
        _is_safe_to_block: bool,
    ) -> Result<(), HRESULT> {
        let method_name = match self.get_method_name(function_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::JitCompilationStartStamp(get_time(), method_name))
    }

    fn jit_compilation_finished(
            &mut self,
            function_id: FunctionID,
            hr_status: FFI_HRESULT, // TODO: Create enum that actual encodes possible statuses instead of hresult param
            is_safe_to_block: bool,
        ) -> Result<(), FFI_HRESULT> {
        let method_name = match self.get_method_name(function_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::JitCompilationFinishStamp(get_time(), method_name))
    }
    // jit handlers: END

    fn shutdown(&mut self) -> Result<(), FFI_HRESULT> {
        match self.ctrl
            .as_ref()
            .unwrap()
            .send(ControlRequests::Shutdown) {
            Ok(_) => {
                println!("notified gRPC-client about shutdown");
                // TODO: consider more safe way
                std::rc::Rc::try_unwrap(self.client.take().unwrap()).unwrap().join();
                Ok(())
            },
            Err(e) => client_lost(e)
        }
    }

    // classes' handlers: START
    fn class_load_started(
            &mut self,
            class_id: clr_profiler::ffi::ClassID
        ) -> Result<(), FFI_HRESULT> {
        let class_name = match self.get_class_name(class_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::ClassLoadStartStamp(get_time(), class_name))
    }

    fn class_load_finished(
            &mut self,
            class_id: clr_profiler::ffi::ClassID,
            hr_status: FFI_HRESULT,
        ) -> Result<(), FFI_HRESULT> {
        let class_name = match self.get_class_name(class_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::ClassLoadFinishedStamp(get_time(), class_name))
    }

    fn class_unload_started(
            &mut self,
            class_id: clr_profiler::ffi::ClassID
        ) -> Result<(), FFI_HRESULT> {
        let class_name = match self.get_class_name(class_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::ClassUnloadStartStamp(get_time(), class_name))
    }

    fn class_unload_finished(
            &mut self,
            class_id: clr_profiler::ffi::ClassID,
            hr_status: FFI_HRESULT,
        ) -> Result<(), FFI_HRESULT> {
        let class_name = match self.get_class_name(class_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::ClassUnloadFinishStamp(get_time(), class_name))
    }
    // classes' handlers: END

    // objects' handlers: START
    // objects' handlers: END

    // threads' handlers: START
    fn thread_created(&mut self, thread_id: clr_profiler::ffi::ThreadID) -> Result<(), FFI_HRESULT> {
        let thread_os_id = self.profiler_info().get_thread_info(thread_id)?;
        Profiler::send_request(&self.tx,
            ClientRequests::ThreadCreatedStamp(get_time(), thread_os_id as u64))
    }

    fn thread_destroyed(&mut self, thread_id: clr_profiler::ffi::ThreadID) -> Result<(), FFI_HRESULT> {
        let thread_os_id = self.profiler_info().get_thread_info(thread_id)?;
        Profiler::send_request(&self.tx,
            ClientRequests::ThreadDestroyedStamp(get_time(), thread_os_id as u64))
    }

    fn runtime_thread_resumed(&mut self, thread_id: clr_profiler::ffi::ThreadID) -> Result<(), FFI_HRESULT> {
        let thread_os_id = self.profiler_info().get_thread_info(thread_id)?;
        Profiler::send_request(&self.tx,
            ClientRequests::ThreadResumedStamp(get_time(), thread_os_id as u64))
    }

    fn runtime_thread_suspended(&mut self, thread_id: clr_profiler::ffi::ThreadID) -> Result<(), FFI_HRESULT> {
        let thread_os_id = self.profiler_info().get_thread_info(thread_id)?;
        Profiler::send_request(&self.tx,
            ClientRequests::ThreadSuspendedStamp(get_time(), thread_os_id as u64))
    }
    // threads' handlers: END

    // exceptions' handlers: START
    fn exception_thrown(&mut self, thrown_object_id: clr_profiler::ffi::ObjectID) -> Result<(), FFI_HRESULT> {
        let class_id = self.profiler_info().get_class_from_object(thrown_object_id)?;
        let exception_class_name = match self.get_class_name(class_id) {
            Ok(name) => name,
            Err(_) => "Unknown".to_string()
        };
        Profiler::send_request(&self.tx,
            ClientRequests::ExceptionThrownStamp(get_time(), exception_class_name))
    }
    // exceptions' handlers: END
}
impl CorProfilerCallback2 for Profiler {}
impl CorProfilerCallback3 for Profiler {}
impl CorProfilerCallback4 for Profiler {}
impl CorProfilerCallback5 for Profiler {}
impl CorProfilerCallback6 for Profiler {}
impl CorProfilerCallback7 for Profiler {}
impl CorProfilerCallback8 for Profiler {}
impl CorProfilerCallback9 for Profiler {}

register!(Profiler);
