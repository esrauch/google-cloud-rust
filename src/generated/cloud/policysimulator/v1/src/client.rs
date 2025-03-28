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

/// Implements a client for the Policy Simulator API.
///
/// # Service Description
///
/// Policy Simulator API service.
///
/// Policy Simulator is a collection of endpoints for creating, running, and
/// viewing a [Replay][google.cloud.policysimulator.v1.Replay]. A
/// [Replay][google.cloud.policysimulator.v1.Replay] is a type of simulation that
/// lets you see how your principals' access to resources might change if you
/// changed your IAM policy.
///
/// During a [Replay][google.cloud.policysimulator.v1.Replay], Policy Simulator
/// re-evaluates, or replays, past access attempts under both the current policy
/// and  your proposed policy, and compares those results to determine how your
/// principals' access might change under the proposed policy.
///
/// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
///
/// # Configuration
///
/// `Simulator` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Simulator` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Simulator` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Simulator {
    inner: Arc<dyn super::stub::dynamic::Simulator>,
}

impl Simulator {
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
        T: super::stub::Simulator + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::Simulator>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::Simulator> {
        super::transport::Simulator::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::Simulator> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Simulator::new)
    }

    /// Gets the specified [Replay][google.cloud.policysimulator.v1.Replay]. Each
    /// `Replay` is available for at least 7 days.
    ///
    /// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
    pub fn get_replay(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::simulator::GetReplay {
        super::builder::simulator::GetReplay::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates and starts a [Replay][google.cloud.policysimulator.v1.Replay] using
    /// the given [ReplayConfig][google.cloud.policysimulator.v1.ReplayConfig].
    ///
    /// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
    /// [google.cloud.policysimulator.v1.ReplayConfig]: crate::model::ReplayConfig
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_replay(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::simulator::CreateReplay {
        super::builder::simulator::CreateReplay::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Lists the results of running a
    /// [Replay][google.cloud.policysimulator.v1.Replay].
    ///
    /// [google.cloud.policysimulator.v1.Replay]: crate::model::Replay
    pub fn list_replay_results(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::simulator::ListReplayResults {
        super::builder::simulator::ListReplayResults::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::simulator::ListOperations {
        super::builder::simulator::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::simulator::GetOperation {
        super::builder::simulator::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
