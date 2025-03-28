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

/// Implements a client for the BeyondCorp API.
///
/// # Service Description
///
/// API Overview:
///
/// The `beyondcorp.googleapis.com` service implements the Google Cloud
/// BeyondCorp API.
///
/// Data Model:
///
/// The AppConnectorsService exposes the following resource:
///
/// * AppConnectors, named as follows:
///   `projects/{project_id}/locations/{location_id}/appConnectors/{app_connector_id}`.
///
/// The AppConnectorsService provides methods to manage
/// (create/read/update/delete) BeyondCorp AppConnectors.
///
/// # Configuration
///
/// `AppConnectorsService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `AppConnectorsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AppConnectorsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AppConnectorsService {
    inner: Arc<dyn super::stub::dynamic::AppConnectorsService>,
}

impl AppConnectorsService {
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
        T: super::stub::AppConnectorsService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::AppConnectorsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::AppConnectorsService> {
        super::transport::AppConnectorsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::AppConnectorsService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::AppConnectorsService::new)
    }

    /// Lists AppConnectors in a given project and location.
    pub fn list_app_connectors(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::ListAppConnectors {
        super::builder::app_connectors_service::ListAppConnectors::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details of a single AppConnector.
    pub fn get_app_connector(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::GetAppConnector {
        super::builder::app_connectors_service::GetAppConnector::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a new AppConnector in a given project and location.
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
    pub fn create_app_connector(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::CreateAppConnector {
        super::builder::app_connectors_service::CreateAppConnector::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the parameters of a single AppConnector.
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
    pub fn update_app_connector(
        &self,
        app_connector: impl Into<crate::model::AppConnector>,
    ) -> super::builder::app_connectors_service::UpdateAppConnector {
        super::builder::app_connectors_service::UpdateAppConnector::new(self.inner.clone())
            .set_app_connector(app_connector.into())
    }

    /// Deletes a single AppConnector.
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
    pub fn delete_app_connector(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::DeleteAppConnector {
        super::builder::app_connectors_service::DeleteAppConnector::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Report status for a given connector.
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
    pub fn report_status(
        &self,
        app_connector: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::ReportStatus {
        super::builder::app_connectors_service::ReportStatus::new(self.inner.clone())
            .set_app_connector(app_connector.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::ListLocations {
        super::builder::app_connectors_service::ListLocations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::GetLocation {
        super::builder::app_connectors_service::GetLocation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    pub fn set_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::SetIamPolicy {
        super::builder::app_connectors_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::GetIamPolicy {
        super::builder::app_connectors_service::GetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    pub fn test_iam_permissions(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::TestIamPermissions {
        super::builder::app_connectors_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::ListOperations {
        super::builder::app_connectors_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::GetOperation {
        super::builder::app_connectors_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::DeleteOperation {
        super::builder::app_connectors_service::DeleteOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::app_connectors_service::CancelOperation {
        super::builder::app_connectors_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
