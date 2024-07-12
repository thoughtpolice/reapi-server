// SPDX-FileCopyrightText: © 2023 Austin Seipp
// SPDX-License-Identifier: MIT OR Apache-2.0

use tokio_stream::wrappers::ReceiverStream;

use crate::protos::build::bazel::remote::execution::v2::{
    content_addressable_storage_server, BatchReadBlobsRequest, BatchReadBlobsResponse,
    BatchUpdateBlobsRequest, BatchUpdateBlobsResponse, FindMissingBlobsRequest,
    FindMissingBlobsResponse, GetTreeRequest, GetTreeResponse,
};

// ---------------------------------------------------------------------------------------------------------------------

#[derive(Debug, Default)]
pub struct ContentAddressableStorageService {}

#[tonic::async_trait]
impl content_addressable_storage_server::ContentAddressableStorage
    for ContentAddressableStorageService
{
    async fn find_missing_blobs(
        &self,
        req: tonic::Request<FindMissingBlobsRequest>,
    ) -> Result<tonic::Response<FindMissingBlobsResponse>, tonic::Status> {
        println!("find_missing_blobs: {:?}", req);
        Err(tonic::Status::unimplemented(
            "find_missing_blobs is not implemented",
        ))
    }

    async fn batch_update_blobs(
        &self,
        req: tonic::Request<BatchUpdateBlobsRequest>,
    ) -> Result<tonic::Response<BatchUpdateBlobsResponse>, tonic::Status> {
        println!("batch_update_blobs: {:?}", req);
        Err(tonic::Status::unimplemented(
            "batch_update_blobs is not implemented",
        ))
    }

    async fn batch_read_blobs(
        &self,
        req: tonic::Request<BatchReadBlobsRequest>,
    ) -> Result<tonic::Response<BatchReadBlobsResponse>, tonic::Status> {
        println!("batch_read_blobs: {:?}", req);
        Err(tonic::Status::unimplemented(
            "batch_read_blobs is not implemented",
        ))
    }

    type GetTreeStream = ReceiverStream<Result<GetTreeResponse, tonic::Status>>;

    async fn get_tree(
        &self,
        req: tonic::Request<GetTreeRequest>,
    ) -> Result<tonic::Response<Self::GetTreeStream>, tonic::Status> {
        println!("get_tree: {:?}", req);
        Err(tonic::Status::unimplemented("get_tree is not implemented"))
    }
}

// ---------------------------------------------------------------------------------------------------------------------
