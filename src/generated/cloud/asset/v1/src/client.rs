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

/// Implements a client for the Cloud Asset API.
///
/// # Service Description
///
/// Asset service definition.
///
/// # Configuration
///
/// `AssetService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `AssetService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AssetService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AssetService {
    inner: Arc<dyn crate::stubs::dynamic::AssetService>,
}

impl AssetService {
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
        T: crate::stubs::AssetService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::AssetService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AssetService> {
        crate::transport::AssetService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AssetService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::AssetService::new)
    }

    /// Exports assets with time and resource types to a given Cloud Storage
    /// location/BigQuery table. For Cloud Storage location destinations, the
    /// output format is newline-delimited JSON. Each line represents a
    /// [google.cloud.asset.v1.Asset][google.cloud.asset.v1.Asset] in the JSON
    /// format; for BigQuery table destinations, the output table stores the fields
    /// in asset Protobuf as columns. This API implements the
    /// [google.longrunning.Operation][google.longrunning.Operation] API, which
    /// allows you to keep track of the export. We recommend intervals of at least
    /// 2 seconds with exponential retry to poll the export operation result. For
    /// regular-size resource parent, the export operation usually finishes within
    /// 5 minutes.
    ///
    /// [google.cloud.asset.v1.Asset]: crate::model::Asset
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    /// # use google_cloud_asset_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::ExportAssetsResponse, model::ExportAssetsRequest>
    /// ) -> Result<model::ExportAssetsResponse> {
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
    /// # use google_cloud_asset_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::ExportAssetsResponse, model::ExportAssetsRequest>
    /// ) -> Result<model::ExportAssetsResponse> {
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
    /// field contains the final response, this will be a [crate::model::ExportAssetsResponse] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::ExportAssetsRequest] value in the `metadata`
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
    /// [send()]: crate::builders::asset_service::ExportAssets::send
    /// [poller()]: crate::builders::asset_service::ExportAssets::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn export_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::ExportAssets {
        crate::builders::asset_service::ExportAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Lists assets with time and resource types and returns paged results in
    /// response.
    pub fn list_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::ListAssets {
        crate::builders::asset_service::ListAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Batch gets the update history of assets that overlap a time window.
    /// For IAM_POLICY content, this API outputs history when the asset and its
    /// attached IAM POLICY both exist. This can create gaps in the output history.
    /// Otherwise, this API outputs history with asset in both non-delete or
    /// deleted status.
    /// If a specified asset does not exist, this API returns an INVALID_ARGUMENT
    /// error.
    pub fn batch_get_assets_history(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::BatchGetAssetsHistory {
        crate::builders::asset_service::BatchGetAssetsHistory::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a feed in a parent project/folder/organization to listen to its
    /// asset updates.
    pub fn create_feed(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::CreateFeed {
        crate::builders::asset_service::CreateFeed::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details about an asset feed.
    pub fn get_feed(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::GetFeed {
        crate::builders::asset_service::GetFeed::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists all asset feeds in a parent project/folder/organization.
    pub fn list_feeds(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::ListFeeds {
        crate::builders::asset_service::ListFeeds::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates an asset feed configuration.
    pub fn update_feed(
        &self,
        feed: impl Into<crate::model::Feed>,
    ) -> crate::builders::asset_service::UpdateFeed {
        crate::builders::asset_service::UpdateFeed::new(self.inner.clone()).set_feed(feed.into())
    }

    /// Deletes an asset feed.
    pub fn delete_feed(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::DeleteFeed {
        crate::builders::asset_service::DeleteFeed::new(self.inner.clone()).set_name(name.into())
    }

    /// Searches all Google Cloud resources within the specified scope, such as a
    /// project, folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllResources` permission on the desired scope,
    /// otherwise the request will be rejected.
    pub fn search_all_resources(
        &self,
        scope: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::SearchAllResources {
        crate::builders::asset_service::SearchAllResources::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Searches all IAM policies within the specified scope, such as a project,
    /// folder, or organization. The caller must be granted the
    /// `cloudasset.assets.searchAllIamPolicies` permission on the desired scope,
    /// otherwise the request will be rejected.
    pub fn search_all_iam_policies(
        &self,
        scope: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::SearchAllIamPolicies {
        crate::builders::asset_service::SearchAllIamPolicies::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes IAM policies to answer which identities have what accesses on
    /// which resources.
    pub fn analyze_iam_policy(
        &self,
        analysis_query: impl Into<crate::model::IamPolicyAnalysisQuery>,
    ) -> crate::builders::asset_service::AnalyzeIamPolicy {
        crate::builders::asset_service::AnalyzeIamPolicy::new(self.inner.clone())
            .set_analysis_query(analysis_query.into())
    }

    /// Analyzes IAM policies asynchronously to answer which identities have what
    /// accesses on which resources, and writes the analysis results to a Google
    /// Cloud Storage or a BigQuery destination. For Cloud Storage destination, the
    /// output format is the JSON format that represents a
    /// [AnalyzeIamPolicyResponse][google.cloud.asset.v1.AnalyzeIamPolicyResponse].
    /// This method implements the
    /// [google.longrunning.Operation][google.longrunning.Operation], which allows
    /// you to track the operation status. We recommend intervals of at least 2
    /// seconds with exponential backoff retry to poll the operation result. The
    /// metadata contains the metadata for the long-running operation.
    ///
    /// [google.cloud.asset.v1.AnalyzeIamPolicyResponse]: crate::model::AnalyzeIamPolicyResponse
    /// [google.longrunning.Operation]: longrunning::model::Operation
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
    /// # use google_cloud_asset_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AnalyzeIamPolicyLongrunningResponse, model::AnalyzeIamPolicyLongrunningMetadata>
    /// ) -> Result<model::AnalyzeIamPolicyLongrunningResponse> {
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
    /// # use google_cloud_asset_v1::model;
    /// async fn wait(
    ///     mut poller: impl lro::Poller<model::AnalyzeIamPolicyLongrunningResponse, model::AnalyzeIamPolicyLongrunningMetadata>
    /// ) -> Result<model::AnalyzeIamPolicyLongrunningResponse> {
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
    /// field contains the final response, this will be a [crate::model::AnalyzeIamPolicyLongrunningResponse] (as
    /// an [Any]), or the error (as a `Status`).
    ///
    /// If the `done` field is `false`, the operation has not completed.  The
    /// operation may also include a [crate::model::AnalyzeIamPolicyLongrunningMetadata] value in the `metadata`
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
    /// [send()]: crate::builders::asset_service::AnalyzeIamPolicyLongrunning::send
    /// [poller()]: crate::builders::asset_service::AnalyzeIamPolicyLongrunning::poller
    /// [until_done()]: lro::Poller::until_done
    /// [PollingPolicy]: gax::polling_policy::PollingPolicy
    /// [PollingBackoffPolicy]: gax::polling_backoff_policy::PollingBackoffPolicy
    /// [get_operation]: Self::get_operation
    /// [metadata]: longrunning::model::Operation::result
    /// [name]: longrunning::model::Operation::name
    /// [Operation]: longrunning::model::Operation
    /// [result]: longrunning::model::Operation::result
    /// [Any]: wkt::Any
    pub fn analyze_iam_policy_longrunning(
        &self,
        analysis_query: impl Into<crate::model::IamPolicyAnalysisQuery>,
    ) -> crate::builders::asset_service::AnalyzeIamPolicyLongrunning {
        crate::builders::asset_service::AnalyzeIamPolicyLongrunning::new(self.inner.clone())
            .set_analysis_query(analysis_query.into())
    }

    /// Analyze moving a resource to a specified destination without kicking off
    /// the actual move. The analysis is best effort depending on the user's
    /// permissions of viewing different hierarchical policies and configurations.
    /// The policies and configuration are subject to change before the actual
    /// resource migration takes place.
    pub fn analyze_move(
        &self,
        resource: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::AnalyzeMove {
        crate::builders::asset_service::AnalyzeMove::new(self.inner.clone())
            .set_resource(resource.into())
    }

    /// Issue a job that queries assets using a SQL statement compatible with
    /// [BigQuery SQL](https://cloud.google.com/bigquery/docs/introduction-sql).
    ///
    /// If the query execution finishes within timeout and there's no pagination,
    /// the full query results will be returned in the `QueryAssetsResponse`.
    ///
    /// Otherwise, full query results can be obtained by issuing extra requests
    /// with the `job_reference` from the a previous `QueryAssets` call.
    ///
    /// Note, the query result has approximately 10 GB limitation enforced by
    /// [BigQuery](https://cloud.google.com/bigquery/docs/best-practices-performance-output).
    /// Queries return larger results will result in errors.
    pub fn query_assets(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::QueryAssets {
        crate::builders::asset_service::QueryAssets::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Creates a saved query in a parent project/folder/organization.
    pub fn create_saved_query(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::CreateSavedQuery {
        crate::builders::asset_service::CreateSavedQuery::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets details about a saved query.
    pub fn get_saved_query(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::GetSavedQuery {
        crate::builders::asset_service::GetSavedQuery::new(self.inner.clone()).set_name(name.into())
    }

    /// Lists all saved queries in a parent project/folder/organization.
    pub fn list_saved_queries(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::ListSavedQueries {
        crate::builders::asset_service::ListSavedQueries::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a saved query.
    pub fn update_saved_query(
        &self,
        saved_query: impl Into<crate::model::SavedQuery>,
    ) -> crate::builders::asset_service::UpdateSavedQuery {
        crate::builders::asset_service::UpdateSavedQuery::new(self.inner.clone())
            .set_saved_query(saved_query.into())
    }

    /// Deletes a saved query.
    pub fn delete_saved_query(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::DeleteSavedQuery {
        crate::builders::asset_service::DeleteSavedQuery::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets effective IAM policies for a batch of resources.
    pub fn batch_get_effective_iam_policies(
        &self,
        scope: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::BatchGetEffectiveIamPolicies {
        crate::builders::asset_service::BatchGetEffectiveIamPolicies::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes organization policies under a scope.
    pub fn analyze_org_policies(
        &self,
        scope: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::AnalyzeOrgPolicies {
        crate::builders::asset_service::AnalyzeOrgPolicies::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes organization policies governed containers (projects, folders or
    /// organization) under a scope.
    pub fn analyze_org_policy_governed_containers(
        &self,
        scope: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::AnalyzeOrgPolicyGovernedContainers {
        crate::builders::asset_service::AnalyzeOrgPolicyGovernedContainers::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Analyzes organization policies governed assets (Google Cloud resources or
    /// policies) under a scope. This RPC supports custom constraints and the
    /// following canned constraints:
    ///
    /// * constraints/ainotebooks.accessMode
    /// * constraints/ainotebooks.disableFileDownloads
    /// * constraints/ainotebooks.disableRootAccess
    /// * constraints/ainotebooks.disableTerminal
    /// * constraints/ainotebooks.environmentOptions
    /// * constraints/ainotebooks.requireAutoUpgradeSchedule
    /// * constraints/ainotebooks.restrictVpcNetworks
    /// * constraints/compute.disableGuestAttributesAccess
    /// * constraints/compute.disableInstanceDataAccessApis
    /// * constraints/compute.disableNestedVirtualization
    /// * constraints/compute.disableSerialPortAccess
    /// * constraints/compute.disableSerialPortLogging
    /// * constraints/compute.disableVpcExternalIpv6
    /// * constraints/compute.requireOsLogin
    /// * constraints/compute.requireShieldedVm
    /// * constraints/compute.restrictLoadBalancerCreationForTypes
    /// * constraints/compute.restrictProtocolForwardingCreationForTypes
    /// * constraints/compute.restrictXpnProjectLienRemoval
    /// * constraints/compute.setNewProjectDefaultToZonalDNSOnly
    /// * constraints/compute.skipDefaultNetworkCreation
    /// * constraints/compute.trustedImageProjects
    /// * constraints/compute.vmCanIpForward
    /// * constraints/compute.vmExternalIpAccess
    /// * constraints/gcp.detailedAuditLoggingMode
    /// * constraints/gcp.resourceLocations
    /// * constraints/iam.allowedPolicyMemberDomains
    /// * constraints/iam.automaticIamGrantsForDefaultServiceAccounts
    /// * constraints/iam.disableServiceAccountCreation
    /// * constraints/iam.disableServiceAccountKeyCreation
    /// * constraints/iam.disableServiceAccountKeyUpload
    /// * constraints/iam.restrictCrossProjectServiceAccountLienRemoval
    /// * constraints/iam.serviceAccountKeyExpiryHours
    /// * constraints/resourcemanager.accessBoundaries
    /// * constraints/resourcemanager.allowedExportDestinations
    /// * constraints/sql.restrictAuthorizedNetworks
    /// * constraints/sql.restrictNoncompliantDiagnosticDataAccess
    /// * constraints/sql.restrictNoncompliantResourceCreation
    /// * constraints/sql.restrictPublicIp
    /// * constraints/storage.publicAccessPrevention
    /// * constraints/storage.restrictAuthTypes
    /// * constraints/storage.uniformBucketLevelAccess
    ///
    /// This RPC only returns either resources of types [supported by search
    /// APIs](https://cloud.google.com/asset-inventory/docs/supported-asset-types)
    /// or IAM policies.
    pub fn analyze_org_policy_governed_assets(
        &self,
        scope: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::AnalyzeOrgPolicyGovernedAssets {
        crate::builders::asset_service::AnalyzeOrgPolicyGovernedAssets::new(self.inner.clone())
            .set_scope(scope.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::asset_service::GetOperation {
        crate::builders::asset_service::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
