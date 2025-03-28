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
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Cloud Scheduler API.
///
/// # Service Description
///
/// The Cloud Scheduler API allows external entities to reliably
/// schedule asynchronous jobs.
///
/// # Configuration
///
/// `CloudScheduler` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `CloudScheduler` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudScheduler` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudScheduler {
    inner: Arc<dyn super::stub::dynamic::CloudScheduler>,
}

impl CloudScheduler {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stub::CloudScheduler + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::CloudScheduler>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CloudScheduler> {
        super::transport::CloudScheduler::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::CloudScheduler> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CloudScheduler::new)
    }

    /// Lists jobs.
    pub fn list_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::ListJobs {
        super::builder::cloud_scheduler::ListJobs::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets a job.
    pub fn get_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::GetJob {
        super::builder::cloud_scheduler::GetJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a job.
    pub fn create_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::CreateJob {
        super::builder::cloud_scheduler::CreateJob::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a job.
    ///
    /// If successful, the updated [Job][google.cloud.scheduler.v1.Job] is
    /// returned. If the job does not exist, `NOT_FOUND` is returned.
    ///
    /// If UpdateJob does not successfully return, it is possible for the
    /// job to be in an
    /// [Job.State.UPDATE_FAILED][google.cloud.scheduler.v1.Job.State.UPDATE_FAILED]
    /// state. A job in this state may not be executed. If this happens, retry the
    /// UpdateJob request until a successful response is received.
    ///
    /// [google.cloud.scheduler.v1.Job]: crate::model::Job
    /// [google.cloud.scheduler.v1.Job.State.UPDATE_FAILED]: crate::model::job::state::UPDATE_FAILED
    pub fn update_job(
        &self,
        job: impl Into<crate::model::Job>,
    ) -> super::builder::cloud_scheduler::UpdateJob {
        super::builder::cloud_scheduler::UpdateJob::new(self.inner.clone()).set_job(job.into())
    }

    /// Deletes a job.
    pub fn delete_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::DeleteJob {
        super::builder::cloud_scheduler::DeleteJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Pauses a job.
    ///
    /// If a job is paused then the system will stop executing the job
    /// until it is re-enabled via
    /// [ResumeJob][google.cloud.scheduler.v1.CloudScheduler.ResumeJob]. The state
    /// of the job is stored in [state][google.cloud.scheduler.v1.Job.state]; if
    /// paused it will be set to
    /// [Job.State.PAUSED][google.cloud.scheduler.v1.Job.State.PAUSED]. A job must
    /// be in [Job.State.ENABLED][google.cloud.scheduler.v1.Job.State.ENABLED] to
    /// be paused.
    ///
    /// [google.cloud.scheduler.v1.CloudScheduler.ResumeJob]: crate::client::CloudScheduler::resume_job
    /// [google.cloud.scheduler.v1.Job.State.ENABLED]: crate::model::job::state::ENABLED
    /// [google.cloud.scheduler.v1.Job.State.PAUSED]: crate::model::job::state::PAUSED
    /// [google.cloud.scheduler.v1.Job.state]: crate::model::Job::state
    pub fn pause_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::PauseJob {
        super::builder::cloud_scheduler::PauseJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Resume a job.
    ///
    /// This method reenables a job after it has been
    /// [Job.State.PAUSED][google.cloud.scheduler.v1.Job.State.PAUSED]. The state
    /// of a job is stored in [Job.state][google.cloud.scheduler.v1.Job.state];
    /// after calling this method it will be set to
    /// [Job.State.ENABLED][google.cloud.scheduler.v1.Job.State.ENABLED]. A job
    /// must be in [Job.State.PAUSED][google.cloud.scheduler.v1.Job.State.PAUSED]
    /// to be resumed.
    ///
    /// [google.cloud.scheduler.v1.Job.State.ENABLED]: crate::model::job::state::ENABLED
    /// [google.cloud.scheduler.v1.Job.State.PAUSED]: crate::model::job::state::PAUSED
    /// [google.cloud.scheduler.v1.Job.state]: crate::model::Job::state
    pub fn resume_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::ResumeJob {
        super::builder::cloud_scheduler::ResumeJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Forces a job to run now.
    ///
    /// When this method is called, Cloud Scheduler will dispatch the job, even
    /// if the job is already running.
    pub fn run_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::RunJob {
        super::builder::cloud_scheduler::RunJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::ListLocations {
        super::builder::cloud_scheduler::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::cloud_scheduler::GetLocation {
        super::builder::cloud_scheduler::GetLocation::new(self.inner.clone()).set_name(name.into())
    }
}
