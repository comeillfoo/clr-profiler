pub mod logcollector {
    tonic::include_proto!("logcollector");
}

mod client;

// #[cfg(feature = "basic_integration_test")]
mod basic_integration_test;
