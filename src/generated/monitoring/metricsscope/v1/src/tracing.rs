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

/// Implements a [MetricsScopes](crate::traits::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct MetricsScopes<T>
where
    T: crate::traits::MetricsScopes + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> MetricsScopes<T>
where
    T: crate::traits::MetricsScopes + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::traits::MetricsScopes for MetricsScopes<T>
where
    T: crate::traits::MetricsScopes + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_metrics_scope(
        &self,
        req: crate::model::GetMetricsScopeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MetricsScope> {
        self.inner.get_metrics_scope(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_metrics_scopes_by_monitored_project(
        &self,
        req: crate::model::ListMetricsScopesByMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMetricsScopesByMonitoredProjectResponse> {
        self.inner
            .list_metrics_scopes_by_monitored_project(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn create_monitored_project(
        &self,
        req: crate::model::CreateMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_monitored_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_monitored_project(
        &self,
        req: crate::model::DeleteMonitoredProjectRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_monitored_project(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
