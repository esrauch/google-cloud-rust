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

/// Implements a client for the Confidential Computing API.
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// `ConfidentialComputing` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ConfidentialComputing` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ConfidentialComputing` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ConfidentialComputing {
    inner: Arc<dyn super::stub::dynamic::ConfidentialComputing>,
}

impl ConfidentialComputing {
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
        T: super::stub::ConfidentialComputing + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::ConfidentialComputing>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ConfidentialComputing> {
        super::transport::ConfidentialComputing::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ConfidentialComputing> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ConfidentialComputing::new)
    }

    /// Creates a new Challenge in a given project and location.
    pub fn create_challenge(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::confidential_computing::CreateChallenge {
        super::builder::confidential_computing::CreateChallenge::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Verifies the provided attestation info, returning a signed OIDC token.
    pub fn verify_attestation(
        &self,
        challenge: impl Into<std::string::String>,
    ) -> super::builder::confidential_computing::VerifyAttestation {
        super::builder::confidential_computing::VerifyAttestation::new(self.inner.clone())
            .set_challenge(challenge.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::confidential_computing::ListLocations {
        super::builder::confidential_computing::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::confidential_computing::GetLocation {
        super::builder::confidential_computing::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }
}
