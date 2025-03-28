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

/// Implements a client for the Identity and Access Management (IAM) API.
///
/// # Service Description
///
/// An interface for managing Identity and Access Management (IAM) policy
/// bindings.
///
/// # Configuration
///
/// `PolicyBindings` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `PolicyBindings` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `PolicyBindings` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct PolicyBindings {
    inner: Arc<dyn super::stub::dynamic::PolicyBindings>,
}

impl PolicyBindings {
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
        T: super::stub::PolicyBindings + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::PolicyBindings>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::PolicyBindings> {
        super::transport::PolicyBindings::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::PolicyBindings> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::PolicyBindings::new)
    }

    /// Creates a policy binding and returns a long-running operation.
    /// Callers will need the IAM permissions on both the policy and target.
    /// Once the binding is created, the policy is applied to the target.
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
    pub fn create_policy_binding(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::policy_bindings::CreatePolicyBinding {
        super::builder::policy_bindings::CreatePolicyBinding::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a policy binding.
    pub fn get_policy_binding(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::policy_bindings::GetPolicyBinding {
        super::builder::policy_bindings::GetPolicyBinding::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates a policy binding and returns a long-running operation.
    /// Callers will need the IAM permissions on the policy and target in the
    /// binding to update, and the IAM permission to remove the existing policy
    /// from the binding. Target is immutable and cannot be updated. Once the
    /// binding is updated, the new policy is applied to the target.
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
    pub fn update_policy_binding(
        &self,
        policy_binding: impl Into<crate::model::PolicyBinding>,
    ) -> super::builder::policy_bindings::UpdatePolicyBinding {
        super::builder::policy_bindings::UpdatePolicyBinding::new(self.inner.clone())
            .set_policy_binding(policy_binding.into())
    }

    /// Deletes a policy binding and returns a long-running operation.
    /// Callers will need the IAM permissions on both the policy and target.
    /// Once the binding is deleted, the policy no longer applies to the target.
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
    pub fn delete_policy_binding(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::policy_bindings::DeletePolicyBinding {
        super::builder::policy_bindings::DeletePolicyBinding::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists policy bindings.
    pub fn list_policy_bindings(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::policy_bindings::ListPolicyBindings {
        super::builder::policy_bindings::ListPolicyBindings::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Search policy bindings by target. Returns all policy binding objects bound
    /// directly to target.
    pub fn search_target_policy_bindings(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::policy_bindings::SearchTargetPolicyBindings {
        super::builder::policy_bindings::SearchTargetPolicyBindings::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::policy_bindings::GetOperation {
        super::builder::policy_bindings::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}

/// Implements a client for the Identity and Access Management (IAM) API.
///
/// # Service Description
///
/// Manages Identity and Access Management (IAM) principal access boundary
/// policies.
///
/// # Configuration
///
/// `PrincipalAccessBoundaryPolicies` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `PrincipalAccessBoundaryPolicies` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `PrincipalAccessBoundaryPolicies` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct PrincipalAccessBoundaryPolicies {
    inner: Arc<dyn super::stub::dynamic::PrincipalAccessBoundaryPolicies>,
}

impl PrincipalAccessBoundaryPolicies {
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
        T: super::stub::PrincipalAccessBoundaryPolicies + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::PrincipalAccessBoundaryPolicies>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::PrincipalAccessBoundaryPolicies> {
        super::transport::PrincipalAccessBoundaryPolicies::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::PrincipalAccessBoundaryPolicies> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::PrincipalAccessBoundaryPolicies::new)
    }

    /// Creates a principal access boundary policy, and returns a long running
    /// operation.
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
    pub fn create_principal_access_boundary_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::principal_access_boundary_policies::CreatePrincipalAccessBoundaryPolicy
    {
        super::builder::principal_access_boundary_policies::CreatePrincipalAccessBoundaryPolicy::new(self.inner.clone())
            .set_parent ( parent.into() )
    }

    /// Gets a principal access boundary policy.
    pub fn get_principal_access_boundary_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::principal_access_boundary_policies::GetPrincipalAccessBoundaryPolicy {
        super::builder::principal_access_boundary_policies::GetPrincipalAccessBoundaryPolicy::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Updates a principal access boundary policy.
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
    pub fn update_principal_access_boundary_policy(
        &self,
        principal_access_boundary_policy: impl Into<crate::model::PrincipalAccessBoundaryPolicy>,
    ) -> super::builder::principal_access_boundary_policies::UpdatePrincipalAccessBoundaryPolicy
    {
        super::builder::principal_access_boundary_policies::UpdatePrincipalAccessBoundaryPolicy::new(self.inner.clone())
            .set_principal_access_boundary_policy ( principal_access_boundary_policy.into() )
    }

    /// Deletes a principal access boundary policy.
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
    pub fn delete_principal_access_boundary_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::principal_access_boundary_policies::DeletePrincipalAccessBoundaryPolicy
    {
        super::builder::principal_access_boundary_policies::DeletePrincipalAccessBoundaryPolicy::new(self.inner.clone())
            .set_name ( name.into() )
    }

    /// Lists principal access boundary policies.
    pub fn list_principal_access_boundary_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::principal_access_boundary_policies::ListPrincipalAccessBoundaryPolicies
    {
        super::builder::principal_access_boundary_policies::ListPrincipalAccessBoundaryPolicies::new(self.inner.clone())
            .set_parent ( parent.into() )
    }

    /// Returns all policy bindings that bind a specific policy if a user has
    /// searchPolicyBindings permission on that policy.
    pub fn search_principal_access_boundary_policy_bindings(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::principal_access_boundary_policies::SearchPrincipalAccessBoundaryPolicyBindings
    {
        super::builder::principal_access_boundary_policies::SearchPrincipalAccessBoundaryPolicyBindings::new(self.inner.clone())
            .set_name ( name.into() )
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::principal_access_boundary_policies::GetOperation {
        super::builder::principal_access_boundary_policies::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
