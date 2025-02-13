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
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the OS Config API.
///
/// # Service Description
///
/// OS Config API
///
/// The OS Config service is a server-side component that you can use to
/// manage package installations and patch jobs for virtual machine instances.
///
/// # Configuration
///
/// `OsConfigService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `OsConfigService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `OsConfigService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct OsConfigService {
    inner: Arc<dyn crate::stubs::dynamic::OsConfigService>,
}

impl OsConfigService {
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
        T: crate::stubs::OsConfigService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::OsConfigService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::OsConfigService> {
        crate::transport::OsConfigService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::OsConfigService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::OsConfigService::new)
    }

    /// Patch VM instances by creating and running a patch job.
    pub fn execute_patch_job(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::ExecutePatchJob {
        crate::builders::os_config_service::ExecutePatchJob::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get the patch job. This can be used to track the progress of an
    /// ongoing patch job or review the details of completed jobs.
    pub fn get_patch_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::GetPatchJob {
        crate::builders::os_config_service::GetPatchJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Cancel a patch job. The patch job must be active. Canceled patch jobs
    /// cannot be restarted.
    pub fn cancel_patch_job(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::CancelPatchJob {
        crate::builders::os_config_service::CancelPatchJob::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Get a list of patch jobs.
    pub fn list_patch_jobs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::ListPatchJobs {
        crate::builders::os_config_service::ListPatchJobs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get a list of instance details for a given patch job.
    pub fn list_patch_job_instance_details(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::ListPatchJobInstanceDetails {
        crate::builders::os_config_service::ListPatchJobInstanceDetails::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Create an OS Config patch deployment.
    pub fn create_patch_deployment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::CreatePatchDeployment {
        crate::builders::os_config_service::CreatePatchDeployment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Get an OS Config patch deployment.
    pub fn get_patch_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::GetPatchDeployment {
        crate::builders::os_config_service::GetPatchDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Get a page of OS Config patch deployments.
    pub fn list_patch_deployments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::ListPatchDeployments {
        crate::builders::os_config_service::ListPatchDeployments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Delete an OS Config patch deployment.
    pub fn delete_patch_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::DeletePatchDeployment {
        crate::builders::os_config_service::DeletePatchDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update an OS Config patch deployment.
    pub fn update_patch_deployment(
        &self,
        patch_deployment: impl Into<crate::model::PatchDeployment>,
    ) -> crate::builders::os_config_service::UpdatePatchDeployment {
        crate::builders::os_config_service::UpdatePatchDeployment::new(self.inner.clone())
            .set_patch_deployment(patch_deployment.into())
    }

    /// Change state of patch deployment to "PAUSED".
    /// Patch deployment in paused state doesn't generate patch jobs.
    pub fn pause_patch_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::PausePatchDeployment {
        crate::builders::os_config_service::PausePatchDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Change state of patch deployment back to "ACTIVE".
    /// Patch deployment in active state continues to generate patch jobs.
    pub fn resume_patch_deployment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::ResumePatchDeployment {
        crate::builders::os_config_service::ResumePatchDeployment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::GetOperation {
        crate::builders::os_config_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_service::CancelOperation {
        crate::builders::os_config_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}

/// Implements a client for the OS Config API.
///
/// # Service Description
///
/// Zonal OS Config API
///
/// The OS Config service is the server-side component that allows users to
/// manage package installations and patch jobs for Compute Engine VM instances.
///
/// # Configuration
///
/// `OsConfigZonalService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `OsConfigZonalService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `OsConfigZonalService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct OsConfigZonalService {
    inner: Arc<dyn crate::stubs::dynamic::OsConfigZonalService>,
}

impl OsConfigZonalService {
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
        T: crate::stubs::OsConfigZonalService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::OsConfigZonalService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::OsConfigZonalService> {
        crate::transport::OsConfigZonalService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::OsConfigZonalService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::OsConfigZonalService::new)
    }

    /// Create an OS policy assignment.
    ///
    /// This method also creates the first revision of the OS policy assignment.
    ///
    /// This method returns a long running operation (LRO) that contains the
    /// rollout details. The rollout can be cancelled by cancelling the LRO.
    ///
    /// For more information, see [Method:
    /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_osconfig_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::OSPolicyAssignment, model::OSPolicyAssignmentOperationMetadata>
    /// ) -> Result<model::OSPolicyAssignment> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_osconfig_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::OSPolicyAssignment, model::OSPolicyAssignmentOperationMetadata>
    /// ) -> Result<model::OSPolicyAssignment> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::OSPolicyAssignment] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OSPolicyAssignmentOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::os_config_zonal_service::CreateOSPolicyAssignment::send
    /// [poller()]: crate::builders::os_config_zonal_service::CreateOSPolicyAssignment::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn create_os_policy_assignment(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::CreateOSPolicyAssignment {
        crate::builders::os_config_zonal_service::CreateOSPolicyAssignment::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Update an existing OS policy assignment.
    ///
    /// This method creates a new revision of the OS policy assignment.
    ///
    /// This method returns a long running operation (LRO) that contains the
    /// rollout details. The rollout can be cancelled by cancelling the LRO.
    ///
    /// For more information, see [Method:
    /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_osconfig_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::OSPolicyAssignment, model::OSPolicyAssignmentOperationMetadata>
    /// ) -> Result<model::OSPolicyAssignment> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_osconfig_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::OSPolicyAssignment, model::OSPolicyAssignmentOperationMetadata>
    /// ) -> Result<model::OSPolicyAssignment> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [crate::model::OSPolicyAssignment] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OSPolicyAssignmentOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::os_config_zonal_service::UpdateOSPolicyAssignment::send
    /// [poller()]: crate::builders::os_config_zonal_service::UpdateOSPolicyAssignment::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn update_os_policy_assignment(
        &self,
        os_policy_assignment: impl Into<crate::model::OSPolicyAssignment>,
    ) -> crate::builders::os_config_zonal_service::UpdateOSPolicyAssignment {
        crate::builders::os_config_zonal_service::UpdateOSPolicyAssignment::new(self.inner.clone())
            .set_os_policy_assignment(os_policy_assignment.into())
    }

    /// Retrieve an existing OS policy assignment.
    ///
    /// This method always returns the latest revision. In order to retrieve a
    /// previous revision of the assignment, also provide the revision ID in the
    /// `name` parameter.
    pub fn get_os_policy_assignment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::GetOSPolicyAssignment {
        crate::builders::os_config_zonal_service::GetOSPolicyAssignment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List the OS policy assignments under the parent resource.
    ///
    /// For each OS policy assignment, the latest revision is returned.
    pub fn list_os_policy_assignments(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::ListOSPolicyAssignments {
        crate::builders::os_config_zonal_service::ListOSPolicyAssignments::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// List the OS policy assignment revisions for a given OS policy assignment.
    pub fn list_os_policy_assignment_revisions(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::ListOSPolicyAssignmentRevisions {
        crate::builders::os_config_zonal_service::ListOSPolicyAssignmentRevisions::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// Delete the OS policy assignment.
    ///
    /// This method creates a new revision of the OS policy assignment.
    ///
    /// This method returns a long running operation (LRO) that contains the
    /// rollout details. The rollout can be cancelled by cancelling the LRO.
    ///
    /// If the LRO completes and is not cancelled, all revisions associated with
    /// the OS policy assignment are deleted.
    ///
    /// For more information, see [Method:
    /// projects.locations.osPolicyAssignments.operations.cancel](https://cloud.google.com/compute/docs/osconfig/rest/v1/projects.locations.osPolicyAssignments.operations/cancel).
    ///
    /// # Long running operations
    ///
    /// Calling [poller()] on the resulting builder returns an implementation of
    /// the [lro::Poller] trait. You need to call `Poller::poll` on this
    /// `Poller` at least once to start the LRO. You may periodically poll this
    /// object to find the status of the operation. The poller automatically
    /// extract the final response value and any intermediate metadata values.
    ///
    /// Calling [send()] on the resulting builder starts a LRO (long-Running
    /// Operation). LROs run in the background, and the application may poll
    /// them periodically to find out if they have succeeded, or failed. See
    /// below for instructions on how to manually use the resulting [Operation].
    /// We recommend `poller()` in favor of `send()`.
    ///
    /// ## Polling until completion
    ///
    /// Applications that do not care about intermediate results in a
    /// long-running operation may use the [until_done()] function:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_osconfig_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::OSPolicyAssignmentOperationMetadata>
    /// ) -> Result<wkt::Empty> {
    ///     poller.until_done().await
    /// }
    /// ```
    ///
    /// This will wait until the LRO completes (successfully or with an error).
    /// Applications can set the [PollingPolicy] and [PollingBackoffPolicy] to
    /// control for how long the function runs.
    ///
    /// ## Polling with detailed metadata updates
    ///
    /// Using the result of [poller()] follows a common pattern:
    ///
    /// ```
    /// # use gax::Result;
    /// # use google_cloud_osconfig_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<wkt::Empty, model::OSPolicyAssignmentOperationMetadata>
    /// ) -> Result<wkt::Empty> {
    ///     while let Some(p) = poller.poll().await {
    ///         match p {
    ///             lro::PollingResult::Completed(r) => { return r; },
    ///             lro::PollingResult::InProgress(m) => { println!("in progress {m:?}"); },
    ///             lro::PollingResult::PollingError(_) => { /* ignored */ },
    ///         }
    ///         tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    ///     }
    ///     Err(gax::error::Error::other("LRO never completed"))
    /// }
    /// ```
    ///
    /// ## Manually polling long-running operations
    ///
    /// If you call [send()], you need to examine the contents of the resulting
    /// [Operation][longrunning::model::Operation] to determine the result of
    /// the operation.
    ///
    /// If the `done` field is `true`, the operation has completed. The `result`
    /// field contains the final response, this will be a [wkt::Empty] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::OSPolicyAssignmentOperationMetadata] value in the `metadata`
    /// field. This value would also be encoded as an [Any]. The metadata may
    /// include information about how much progress the LRO has made.
    ///
    /// To find out if the operation has completed, use the [get_operation]
    /// method and repeat the steps outlined above.
    ///
    /// Note that most errors on [get_operation] do not indicate that the
    /// long-running operation failed. Long-running operation failures return
    /// the error status in the [result] field.
    ///
    /// [send()]: crate::builders::os_config_zonal_service::DeleteOSPolicyAssignment::send
    /// [poller()]: crate::builders::os_config_zonal_service::DeleteOSPolicyAssignment::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn delete_os_policy_assignment(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::DeleteOSPolicyAssignment {
        crate::builders::os_config_zonal_service::DeleteOSPolicyAssignment::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Get the OS policy asssignment report for the specified Compute Engine VM
    /// instance.
    pub fn get_os_policy_assignment_report(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::GetOSPolicyAssignmentReport {
        crate::builders::os_config_zonal_service::GetOSPolicyAssignmentReport::new(
            self.inner.clone(),
        )
        .set_name(name.into())
    }

    /// List OS policy asssignment reports for all Compute Engine VM instances in
    /// the specified zone.
    pub fn list_os_policy_assignment_reports(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::ListOSPolicyAssignmentReports {
        crate::builders::os_config_zonal_service::ListOSPolicyAssignmentReports::new(
            self.inner.clone(),
        )
        .set_parent(parent.into())
    }

    /// Get inventory data for the specified VM instance. If the VM has no
    /// associated inventory, the message `NOT_FOUND` is returned.
    pub fn get_inventory(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::GetInventory {
        crate::builders::os_config_zonal_service::GetInventory::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List inventory data for all VM instances in the specified zone.
    pub fn list_inventories(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::ListInventories {
        crate::builders::os_config_zonal_service::ListInventories::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the vulnerability report for the specified VM instance. Only VMs with
    /// inventory data have vulnerability reports associated with them.
    pub fn get_vulnerability_report(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::GetVulnerabilityReport {
        crate::builders::os_config_zonal_service::GetVulnerabilityReport::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List vulnerability reports for all VM instances in the specified zone.
    pub fn list_vulnerability_reports(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::ListVulnerabilityReports {
        crate::builders::os_config_zonal_service::ListVulnerabilityReports::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::GetOperation {
        crate::builders::os_config_zonal_service::GetOperation::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::os_config_zonal_service::CancelOperation {
        crate::builders::os_config_zonal_service::CancelOperation::new(self.inner.clone())
            .set_name(name.into())
    }
}
