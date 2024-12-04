// @generated
pub mod google {
    // @@protoc_insertion_point(attribute:google.api)
    pub mod api {
        include!("google.api.rs");
        // @@protoc_insertion_point(google.api)
    }
    pub mod bigtable {
        // @@protoc_insertion_point(attribute:google.bigtable.v2)
        pub mod v2 {
            include!("google.bigtable.v2.rs");
            // @@protoc_insertion_point(google.bigtable.v2)
        }
    }
    pub mod cloud {
        pub mod conformance {
            pub mod bigtable {
                // @@protoc_insertion_point(attribute:google.cloud.conformance.bigtable.v2)
                pub mod v2 {
                    include!("google.cloud.conformance.bigtable.v2.rs");
                    // @@protoc_insertion_point(google.cloud.conformance.bigtable.v2)
                }
            }
        }
    }
    pub mod logging {
        // @@protoc_insertion_point(attribute:google.logging.type)
        pub mod r#type {
            include!("google.logging.type.rs");
            // @@protoc_insertion_point(google.logging.type)
        }
        // @@protoc_insertion_point(attribute:google.logging.v2)
        pub mod v2 {
            include!("google.logging.v2.rs");
            // @@protoc_insertion_point(google.logging.v2)
        }
    }
    // @@protoc_insertion_point(attribute:google.longrunning)
    pub mod longrunning {
        include!("google.longrunning.rs");
        // @@protoc_insertion_point(google.longrunning)
    }
    // @@protoc_insertion_point(attribute:google.type)
    pub mod r#type {
        include!("google.type.rs");
        // @@protoc_insertion_point(google.type)
    }
    // @@protoc_insertion_point(attribute:google.rpc)
    pub mod rpc {
        include!("google.rpc.rs");
        // @@protoc_insertion_point(google.rpc)
        // @@protoc_insertion_point(attribute:google.rpc.context)
        pub mod context {
            include!("google.rpc.context.rs");
            // @@protoc_insertion_point(google.rpc.context)
        }
    }
}
