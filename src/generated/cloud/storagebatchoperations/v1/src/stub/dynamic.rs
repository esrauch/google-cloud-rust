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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::StorageBatchOperations].
#[async_trait::async_trait]
pub trait StorageBatchOperations: std::fmt::Debug + Send + Sync {
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListJobsResponse>;

    async fn get_job(
        &self,
        req: crate::model::GetJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job>;

    async fn create_job(
        &self,
        req: crate::model::CreateJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn cancel_job(
        &self,
        req: crate::model::CancelJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CancelJobResponse>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()>;

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [super::StorageBatchOperations] also implement [StorageBatchOperations].
#[async_trait::async_trait]
impl<T: super::StorageBatchOperations> StorageBatchOperations for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_jobs(
        &self,
        req: crate::model::ListJobsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListJobsResponse> {
        T::list_jobs(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_job(
        &self,
        req: crate::model::GetJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Job> {
        T::get_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_job(
        &self,
        req: crate::model::CreateJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_job(
        &self,
        req: crate::model::DeleteJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_job(
        &self,
        req: crate::model::CancelJobRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::CancelJobResponse> {
        T::cancel_job(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<()> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        T::get_polling_error_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
