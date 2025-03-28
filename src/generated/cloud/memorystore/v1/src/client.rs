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

/// Implements a client for the Memorystore API.
///
/// # Service Description
///
/// Service describing handlers for resources
///
/// # Configuration
///
/// `Memorystore` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Memorystore` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Memorystore` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Memorystore {
    inner: Arc<dyn super::stub::dynamic::Memorystore>,
}

impl Memorystore {
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
        T: super::stub::Memorystore + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::Memorystore>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::Memorystore> {
        super::transport::Memorystore::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::Memorystore> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::Memorystore::new)
    }

    /// Lists Instances in a given project and location.
    pub fn list_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::memorystore::ListInstances {
        super::builder::memorystore::ListInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single Instance.
    pub fn get_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::GetInstance {
        super::builder::memorystore::GetInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a new Instance in a given project and location.
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
    pub fn create_instance(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::memorystore::CreateInstance {
        super::builder::memorystore::CreateInstance::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single Instance.
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
    pub fn update_instance(
        &self,
        instance: impl Into<crate::model::Instance>,
    ) -> super::builder::memorystore::UpdateInstance {
        super::builder::memorystore::UpdateInstance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Deletes a single Instance.
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
    pub fn delete_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::DeleteInstance {
        super::builder::memorystore::DeleteInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets details about the certificate authority for an Instance.
    pub fn get_certificate_authority(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::GetCertificateAuthority {
        super::builder::memorystore::GetCertificateAuthority::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::ListLocations {
        super::builder::memorystore::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::GetLocation {
        super::builder::memorystore::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::ListOperations {
        super::builder::memorystore::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::GetOperation {
        super::builder::memorystore::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::DeleteOperation {
        super::builder::memorystore::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::memorystore::CancelOperation {
        super::builder::memorystore::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
