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

/// Implements a client for the Organization Policy API.
///
/// # Service Description
///
/// An interface for managing organization policies.
///
/// The Organization Policy Service provides a simple mechanism for
/// organizations to restrict the allowed configurations across their entire
/// resource hierarchy.
///
/// You can use a policy to configure restrictions on resources. For
/// example, you can enforce a policy that restricts which Google
/// Cloud APIs can be activated in a certain part of your resource
/// hierarchy, or prevents serial port access to VM instances in a
/// particular folder.
///
/// Policies are inherited down through the resource hierarchy. A policy
/// applied to a parent resource automatically applies to all its child resources
/// unless overridden with a policy lower in the hierarchy.
///
/// A constraint defines an aspect of a resource's configuration that can be
/// controlled by an organization's policy administrator. Policies are a
/// collection of constraints that defines their allowable configuration on a
/// particular resource and its child resources.
///
/// # Configuration
///
/// `OrgPolicy` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `OrgPolicy` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `OrgPolicy` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct OrgPolicy {
    inner: Arc<dyn super::stub::dynamic::OrgPolicy>,
}

impl OrgPolicy {
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
        T: super::stub::OrgPolicy + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::OrgPolicy>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::OrgPolicy> {
        super::transport::OrgPolicy::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::OrgPolicy> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::OrgPolicy::new)
    }

    /// Lists constraints that could be applied on the specified resource.
    pub fn list_constraints(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::org_policy::ListConstraints {
        super::builder::org_policy::ListConstraints::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Retrieves all of the policies that exist on a particular resource.
    pub fn list_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::org_policy::ListPolicies {
        super::builder::org_policy::ListPolicies::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets a policy on a resource.
    ///
    /// If no policy is set on the resource, `NOT_FOUND` is returned. The
    /// `etag` value can be used with `UpdatePolicy()` to update a
    /// policy during read-modify-write.
    pub fn get_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::org_policy::GetPolicy {
        super::builder::org_policy::GetPolicy::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the effective policy on a resource. This is the result of merging
    /// policies in the resource hierarchy and evaluating conditions. The
    /// returned policy will not have an `etag` or `condition` set because it is
    /// an evaluated policy across multiple resources.
    /// Subtrees of Resource Manager resource hierarchy with 'under:' prefix will
    /// not be expanded.
    pub fn get_effective_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::org_policy::GetEffectivePolicy {
        super::builder::org_policy::GetEffectivePolicy::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a policy.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// constraint does not exist.
    /// Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the
    /// policy already exists on the given Google Cloud resource.
    pub fn create_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::org_policy::CreatePolicy {
        super::builder::org_policy::CreatePolicy::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates a policy.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// constraint or the policy do not exist.
    /// Returns a `google.rpc.Status` with `google.rpc.Code.ABORTED` if the etag
    /// supplied in the request does not match the persisted etag of the policy
    ///
    /// Note: the supplied policy will perform a full overwrite of all
    /// fields.
    pub fn update_policy(
        &self,
        policy: impl Into<crate::model::Policy>,
    ) -> super::builder::org_policy::UpdatePolicy {
        super::builder::org_policy::UpdatePolicy::new(self.inner.clone()).set_policy(policy.into())
    }

    /// Deletes a policy.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// constraint or organization policy does not exist.
    pub fn delete_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::org_policy::DeletePolicy {
        super::builder::org_policy::DeletePolicy::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a custom constraint.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// organization does not exist.
    /// Returns a `google.rpc.Status` with `google.rpc.Code.ALREADY_EXISTS` if the
    /// constraint already exists on the given organization.
    pub fn create_custom_constraint(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::org_policy::CreateCustomConstraint {
        super::builder::org_policy::CreateCustomConstraint::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a custom constraint.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// constraint does not exist.
    ///
    /// Note: the supplied policy will perform a full overwrite of all
    /// fields.
    pub fn update_custom_constraint(
        &self,
        custom_constraint: impl Into<crate::model::CustomConstraint>,
    ) -> super::builder::org_policy::UpdateCustomConstraint {
        super::builder::org_policy::UpdateCustomConstraint::new(self.inner.clone())
            .set_custom_constraint(custom_constraint.into())
    }

    /// Gets a custom constraint.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// custom constraint does not exist.
    pub fn get_custom_constraint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::org_policy::GetCustomConstraint {
        super::builder::org_policy::GetCustomConstraint::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Retrieves all of the custom constraints that exist on a particular
    /// organization resource.
    pub fn list_custom_constraints(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::org_policy::ListCustomConstraints {
        super::builder::org_policy::ListCustomConstraints::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a custom constraint.
    ///
    /// Returns a `google.rpc.Status` with `google.rpc.Code.NOT_FOUND` if the
    /// constraint does not exist.
    pub fn delete_custom_constraint(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::org_policy::DeleteCustomConstraint {
        super::builder::org_policy::DeleteCustomConstraint::new(self.inner.clone())
            .set_name(name.into())
    }
}
