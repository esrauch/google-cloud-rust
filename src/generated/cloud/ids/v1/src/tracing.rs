// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Code generated by sidekick. DO NOT EDIT.
use crate::Result;

/// Implements a [Ids](super::stub::Ids) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Ids<T>
where
    T: super::stub::Ids + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Ids<T>
where
    T: super::stub::Ids + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::Ids for Ids<T>
where
    T: super::stub::Ids + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_endpoints(
        &self,
        req: crate::model::ListEndpointsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEndpointsResponse> {
        self.inner.list_endpoints(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_endpoint(
        &self,
        req: crate::model::GetEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Endpoint> {
        self.inner.get_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_endpoint(
        &self,
        req: crate::model::CreateEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_endpoint(
        &self,
        req: crate::model::DeleteEndpointRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_endpoint(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
