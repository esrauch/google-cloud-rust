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

/// Implements a client for the Managed Service for Microsoft Active Directory API.
///
/// # Service Description
///
/// API Overview
///
/// The `managedidentites.googleapis.com` service implements the Google Cloud
/// Managed Identites API for identity services
/// (e.g. Microsoft Active Directory).
///
/// The Managed Identities service provides methods to manage
/// (create/read/update/delete) domains, reset managed identities admin password,
/// add/remove domain controllers in GCP regions and add/remove VPC peering.
///
/// Data Model
///
/// The Managed Identities service exposes the following resources:
///
/// * Locations as global, named as follows:
///   `projects/{project_id}/locations/global`.
///
/// * Domains, named as follows:
///   `/projects/{project_id}/locations/global/domain/{domain_name}`.
///
///
/// The `{domain_name}` refers to fully qualified domain name in the customer
/// project e.g. mydomain.myorganization.com, with the following restrictions:
///
/// * Must contain only lowercase letters, numbers, periods and hyphens.
/// * Must start with a letter.
/// * Must contain between 2-64 characters.
/// * Must end with a number or a letter.
/// * Must not start with period.
/// * First segement length (mydomain form example above) shouldn't exceed
///   15 chars.
/// * The last segment cannot be fully numeric.
/// * Must be unique within the customer project.
///
/// # Configuration
///
/// `ManagedIdentitiesService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `ManagedIdentitiesService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `ManagedIdentitiesService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct ManagedIdentitiesService {
    inner: Arc<dyn super::stub::dynamic::ManagedIdentitiesService>,
}

impl ManagedIdentitiesService {
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
        T: super::stub::ManagedIdentitiesService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::ManagedIdentitiesService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ManagedIdentitiesService> {
        super::transport::ManagedIdentitiesService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::ManagedIdentitiesService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::ManagedIdentitiesService::new)
    }

    /// Creates a Microsoft AD domain.
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
    pub fn create_microsoft_ad_domain(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::CreateMicrosoftAdDomain {
        super::builder::managed_identities_service::CreateMicrosoftAdDomain::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Resets a domain's administrator password.
    pub fn reset_admin_password(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::ResetAdminPassword {
        super::builder::managed_identities_service::ResetAdminPassword::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists domains in a project.
    pub fn list_domains(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::ListDomains {
        super::builder::managed_identities_service::ListDomains::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets information about a domain.
    pub fn get_domain(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::GetDomain {
        super::builder::managed_identities_service::GetDomain::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the metadata and configuration of a domain.
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
    pub fn update_domain(
        &self,
        domain: impl Into<crate::model::Domain>,
    ) -> super::builder::managed_identities_service::UpdateDomain {
        super::builder::managed_identities_service::UpdateDomain::new(self.inner.clone())
            .set_domain(domain.into())
    }

    /// Deletes a domain.
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
    pub fn delete_domain(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::DeleteDomain {
        super::builder::managed_identities_service::DeleteDomain::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Adds an AD trust to a domain.
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
    pub fn attach_trust(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::AttachTrust {
        super::builder::managed_identities_service::AttachTrust::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the DNS conditional forwarder.
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
    pub fn reconfigure_trust(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::ReconfigureTrust {
        super::builder::managed_identities_service::ReconfigureTrust::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Removes an AD trust.
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
    pub fn detach_trust(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::DetachTrust {
        super::builder::managed_identities_service::DetachTrust::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Validates a trust state, that the target domain is reachable, and that the
    /// target domain is able to accept incoming trust requests.
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
    pub fn validate_trust(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::ValidateTrust {
        super::builder::managed_identities_service::ValidateTrust::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::ListOperations {
        super::builder::managed_identities_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::GetOperation {
        super::builder::managed_identities_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::DeleteOperation {
        super::builder::managed_identities_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::managed_identities_service::CancelOperation {
        super::builder::managed_identities_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
