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

/// Implements a client for the Access Approval API.
///
/// # Service Description
///
/// This API allows a customer to manage accesses to cloud resources by
/// Google personnel. It defines the following resource model:
///
/// - The API has a collection of
///   [ApprovalRequest][google.cloud.accessapproval.v1.ApprovalRequest]
///   resources, named `approvalRequests/{approval_request}`
/// - The API has top-level settings per Project/Folder/Organization, named
///   `accessApprovalSettings`
///
/// The service also periodically emails a list of recipients, defined at the
/// Project/Folder/Organization level in the accessApprovalSettings, when there
/// is a pending ApprovalRequest for them to act on. The ApprovalRequests can
/// also optionally be published to a Pub/Sub topic owned by the customer
/// (contact support if you would like to enable Pub/Sub notifications).
///
/// ApprovalRequests can be approved or dismissed. Google personnel can only
/// access the indicated resource or resources if the request is approved
/// (subject to some exclusions:
/// <https://cloud.google.com/access-approval/docs/overview#exclusions>).
///
/// Note: Using Access Approval functionality will mean that Google may not be
/// able to meet the SLAs for your chosen products, as any support response times
/// may be dramatically increased. As such the SLAs do not apply to any service
/// disruption to the extent impacted by Customer's use of Access Approval. Do
/// not enable Access Approval for projects where you may require high service
/// availability and rapid response by Google Cloud Support.
///
/// After a request is approved or dismissed, no further action may be taken on
/// it. Requests with the requested_expiration in the past or with no activity
/// for 14 days are considered dismissed. When an approval expires, the request
/// is considered dismissed.
///
/// If a request is not approved or dismissed, we call it pending.
///
/// [google.cloud.accessapproval.v1.ApprovalRequest]: crate::model::ApprovalRequest
///
/// # Configuration
///
/// `AccessApproval` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `AccessApproval` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AccessApproval` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AccessApproval {
    inner: Arc<dyn super::stub::dynamic::AccessApproval>,
}

impl AccessApproval {
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
        T: super::stub::AccessApproval + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stub::dynamic::AccessApproval>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::AccessApproval> {
        super::transport::AccessApproval::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stub::AccessApproval> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::AccessApproval::new)
    }

    /// Lists approval requests associated with a project, folder, or organization.
    /// Approval requests can be filtered by state (pending, active, dismissed).
    /// The order is reverse chronological.
    pub fn list_approval_requests(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builder::access_approval::ListApprovalRequests {
        super::builder::access_approval::ListApprovalRequests::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets an approval request. Returns NOT_FOUND if the request does not exist.
    pub fn get_approval_request(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::GetApprovalRequest {
        super::builder::access_approval::GetApprovalRequest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Approves a request and returns the updated ApprovalRequest.
    ///
    /// Returns NOT_FOUND if the request does not exist. Returns
    /// FAILED_PRECONDITION if the request exists but is not in a pending state.
    pub fn approve_approval_request(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::ApproveApprovalRequest {
        super::builder::access_approval::ApproveApprovalRequest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Dismisses a request. Returns the updated ApprovalRequest.
    ///
    /// NOTE: This does not deny access to the resource if another request has been
    /// made and approved. It is equivalent in effect to ignoring the request
    /// altogether.
    ///
    /// Returns NOT_FOUND if the request does not exist.
    ///
    /// Returns FAILED_PRECONDITION if the request exists but is not in a pending
    /// state.
    pub fn dismiss_approval_request(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::DismissApprovalRequest {
        super::builder::access_approval::DismissApprovalRequest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Invalidates an existing ApprovalRequest. Returns the updated
    /// ApprovalRequest.
    ///
    /// NOTE: This does not deny access to the resource if another request has been
    /// made and approved. It only invalidates a single approval.
    ///
    /// Returns FAILED_PRECONDITION if the request exists but is not in an approved
    /// state.
    pub fn invalidate_approval_request(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::InvalidateApprovalRequest {
        super::builder::access_approval::InvalidateApprovalRequest::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets the settings associated with a project, folder, or organization.
    pub fn get_access_approval_settings(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::GetAccessApprovalSettings {
        super::builder::access_approval::GetAccessApprovalSettings::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Updates the settings associated with a project, folder, or organization.
    /// Settings to update are determined by the value of field_mask.
    pub fn update_access_approval_settings(
        &self,
        settings: impl Into<crate::model::AccessApprovalSettings>,
    ) -> super::builder::access_approval::UpdateAccessApprovalSettings {
        super::builder::access_approval::UpdateAccessApprovalSettings::new(self.inner.clone())
            .set_settings(settings.into())
    }

    /// Deletes the settings associated with a project, folder, or organization.
    /// This will have the effect of disabling Access Approval for the project,
    /// folder, or organization, but only if all ancestors also have Access
    /// Approval disabled. If Access Approval is enabled at a higher level of the
    /// hierarchy, then Access Approval will still be enabled at this level as
    /// the settings are inherited.
    pub fn delete_access_approval_settings(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::DeleteAccessApprovalSettings {
        super::builder::access_approval::DeleteAccessApprovalSettings::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Retrieves the service account that is used by Access Approval to access KMS
    /// keys for signing approved approval requests.
    pub fn get_access_approval_service_account(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builder::access_approval::GetAccessApprovalServiceAccount {
        super::builder::access_approval::GetAccessApprovalServiceAccount::new(self.inner.clone())
            .set_name(name.into())
    }
}
