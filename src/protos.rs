// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Protobuf files, reflected as a Rust module hierarchy. These basically map
//! 1-to-1 with the actual protobuf namespace.ea

// ---------------------------------------------------------------------------------------------------------------------

pub mod google {
    pub mod api {
        tonic::include_proto!("google.api");
    }
    pub mod bytestream {
        tonic::include_proto!("google.bytestream");
    }
    pub mod longrunning {
        tonic::include_proto!("google.longrunning");
    }
    pub mod rpc {
        tonic::include_proto!("google.rpc");
    }
}

pub mod build {
    pub mod bazel {
        pub mod remote {
            pub mod execution {
                pub mod v2 {
                    tonic::include_proto!("build.bazel.remote.execution.v2");
                }
            }
        }

        pub mod semver {
            tonic::include_proto!("build.bazel.semver");
        }
    }
}
