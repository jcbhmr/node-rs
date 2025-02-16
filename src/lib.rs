use autocxx::prelude::*;

include_cpp! {
    #include "v8.h"
    #include "uv.h"
    #include "node.h"
    safety!(unsafe)
    generate!("uv_setup_args")
    generate!("node::SnapshotData")
    generate!("node::IsolateData")
    generate!("node::Environment")
    generate!("node::MultiIsolatePlatform")
    generate!("node::Start")
    generate!("node::Stop")
    generate!("node::InitializeOncePerProcess")
    generate!("node::TearDownOncePerProcess")
    generate!("node::ProcessGlobalArgs")
    extern_cpp_opaque_type!("node::CommonEnvironmentSetup", ffi2_node::CommonEnvironmentSetup)
    extern_cpp_opaque_type!("node::EmbedderSnapshotData", ffi3_node::EmbedderSnapshotData)
}
#[cxx::bridge(namespace = "node")]
mod ffi2_node {
    unsafe extern "C++" {
        include!("node.h");
        type CommonEnvironmentSetup;
    }
    impl UniquePtr<CommonEnvironmentSetup> {}
}
#[cxx::bridge(namespace = "node")]
mod ffi3_node {
    unsafe extern "C++" {
        include!("node.h");
        type EmbedderSnapshotData;
    }
    impl UniquePtr<EmbedderSnapshotData> {}
}
pub use ffi::*;
pub use ffi2_node::*;
pub use ffi3_node::*;
