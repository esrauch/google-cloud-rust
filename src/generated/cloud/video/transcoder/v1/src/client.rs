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

/// Implements a client for the Transcoder API.
///
/// # Service Description
///
/// Using the Transcoder API, you can queue asynchronous jobs for transcoding
/// media into various output formats. Output formats may include different
/// streaming standards such as HTTP Live Streaming (HLS) and Dynamic Adaptive
/// Streaming over HTTP (DASH). You can also customize jobs using advanced
/// features such as Digital Rights Management (DRM), audio equalization, content
/// concatenation, and digital ad-stitch ready content generation.
///
/// # Configuration
///
/// `TranscoderService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `TranscoderService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `TranscoderService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct TranscoderService {
    inner: Arc<dyn super::stub::dynamic::TranscoderService>,
}

impl TranscoderService {
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
        T: super::stub::TranscoderService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::TranscoderService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::TranscoderService> {
        super::transport::TranscoderService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::TranscoderService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::TranscoderService::new)
    }

    /// Creates a job in the specified region.
    pub fn create_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::CreateJob {
        super::builder::transcoder_service::CreateJob::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists jobs in the specified region.
    pub fn list_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::ListJobs {
        super::builder::transcoder_service::ListJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the job data.
    pub fn get_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::GetJob {
        super::builder::transcoder_service::GetJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Deletes a job.
    pub fn delete_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::DeleteJob {
        super::builder::transcoder_service::DeleteJob::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a job template in the specified region.
    pub fn create_job_template(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::CreateJobTemplate {
        super::builder::transcoder_service::CreateJobTemplate::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists job templates in the specified region.
    pub fn list_job_templates(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::ListJobTemplates {
        super::builder::transcoder_service::ListJobTemplates::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns the job template data.
    pub fn get_job_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::GetJobTemplate {
        super::builder::transcoder_service::GetJobTemplate::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a job template.
    pub fn delete_job_template(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::transcoder_service::DeleteJobTemplate {
        super::builder::transcoder_service::DeleteJobTemplate::new(self.inner.clone())
            .set_name(name.into())
    }
}
