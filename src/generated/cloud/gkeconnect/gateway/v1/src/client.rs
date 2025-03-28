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

/// Implements a client for the Connect Gateway API.
///
/// # Service Description
///
/// GatewayControl is the control plane API for Connect Gateway.
///
/// # Configuration
///
/// `GatewayControl` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `GatewayControl` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `GatewayControl` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct GatewayControl {
    inner: Arc<dyn super::stub::dynamic::GatewayControl>,
}

impl GatewayControl {
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
        T: super::stub::GatewayControl + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::GatewayControl>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::GatewayControl> {
        super::transport::GatewayControl::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::GatewayControl> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::GatewayControl::new)
    }

    /// GenerateCredentials provides connection information that allows a user to
    /// access the specified membership using Connect Gateway.
    pub fn generate_credentials(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::gateway_control::GenerateCredentials {
        super::builder::gateway_control::GenerateCredentials::new(self.inner.clone())
            .set_name(name.into())
    }
}
