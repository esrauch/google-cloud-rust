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

/// Implements a client for the Cloud Functions API.
///
/// # Service Description
///
/// Google Cloud Functions is used to deploy functions that are executed by
/// Google in response to various events. Data connected with that event is
/// passed to a function as the input data.
///
/// A **function** is a resource which describes a function that should be
/// executed and how it is triggered.
///
/// # Configuration
///
/// `FunctionService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `FunctionService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `FunctionService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct FunctionService {
    inner: Arc<dyn super::stub::dynamic::FunctionService>,
}

impl FunctionService {
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
        T: super::stub::FunctionService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::FunctionService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::FunctionService> {
        super::transport::FunctionService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::FunctionService> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::FunctionService::new)
    }

    /// Returns a function with the given name from the requested project.
    pub fn get_function(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::function_service::GetFunction {
        super::builder::function_service::GetFunction::new(self.inner.clone()).set_name(name.into())
    }

    /// Returns a list of functions that belong to the requested project.
    pub fn list_functions(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::function_service::ListFunctions {
        super::builder::function_service::ListFunctions::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a new function. If a function with the given name already exists in
    /// the specified project, the long running operation will return
    /// `ALREADY_EXISTS` error.
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
    pub fn create_function(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::function_service::CreateFunction {
        super::builder::function_service::CreateFunction::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates existing function.
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
    pub fn update_function(
        &self,
        function: impl Into<crate::model::Function>,
    ) -> super::builder::function_service::UpdateFunction {
        super::builder::function_service::UpdateFunction::new(self.inner.clone())
            .set_function(function.into())
    }

    /// Deletes a function with the given name from the specified project. If the
    /// given function is used by some trigger, the trigger will be updated to
    /// remove this function.
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
    pub fn delete_function(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::function_service::DeleteFunction {
        super::builder::function_service::DeleteFunction::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns a signed URL for uploading a function source code.
    /// For more information about the signed URL usage see:
    /// <https://cloud.google.com/storage/docs/access-control/signed-urls>.
    /// Once the function source code upload is complete, the used signed
    /// URL should be provided in CreateFunction or UpdateFunction request
    /// as a reference to the function source code.
    ///
    /// When uploading source code to the generated signed URL, please follow
    /// these restrictions:
    ///
    /// * Source file type should be a zip file.
    /// * No credentials should be attached - the signed URLs provide access to the
    ///   target bucket using internal service identity; if credentials were
    ///   attached, the identity from the credentials would be used, but that
    ///   identity does not have permissions to upload files to the URL.
    ///
    /// When making a HTTP PUT request, specify this header:
    ///
    /// * `content-type: application/zip`
    ///
    /// Do not specify this header:
    ///
    /// * `Authorization: Bearer YOUR_TOKEN`
    pub fn generate_upload_url(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::function_service::GenerateUploadUrl {
        super::builder::function_service::GenerateUploadUrl::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Returns a signed URL for downloading deployed function source code.
    /// The URL is only valid for a limited period and should be used within
    /// 30 minutes of generation.
    /// For more information about the signed URL usage see:
    /// <https://cloud.google.com/storage/docs/access-control/signed-urls>
    pub fn generate_download_url(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::function_service::GenerateDownloadUrl {
        super::builder::function_service::GenerateDownloadUrl::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Returns a list of runtimes that are supported for the requested project.
    pub fn list_runtimes(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::function_service::ListRuntimes {
        super::builder::function_service::ListRuntimes::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::function_service::ListLocations {
        super::builder::function_service::ListLocations::new(self.inner.clone())
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
    ) -> super::builder::function_service::SetIamPolicy {
        super::builder::function_service::SetIamPolicy::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    pub fn get_iam_policy(
        &self,
        resource: impl Into<std::string::String>,
    ) -> super::builder::function_service::GetIamPolicy {
        super::builder::function_service::GetIamPolicy::new(self.inner.clone())
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
    ) -> super::builder::function_service::TestIamPermissions {
        super::builder::function_service::TestIamPermissions::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::function_service::ListOperations {
        super::builder::function_service::ListOperations::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::function_service::GetOperation {
        super::builder::function_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
