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
use crate::Result;

/// Implements a [BigtableInstanceAdmin](super::stub::BigtableInstanceAdmin) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct BigtableInstanceAdmin<T>
where
    T: super::stub::BigtableInstanceAdmin + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> BigtableInstanceAdmin<T>
where
    T: super::stub::BigtableInstanceAdmin + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::BigtableInstanceAdmin for BigtableInstanceAdmin<T>
where
    T: super::stub::BigtableInstanceAdmin + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_instance(
        &self,
        req: crate::model::GetInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Instance> {
        self.inner.get_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_instances(
        &self,
        req: crate::model::ListInstancesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstancesResponse> {
        self.inner.list_instances(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_instance(
        &self,
        req: crate::model::Instance,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Instance> {
        self.inner.update_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn partial_update_instance(
        &self,
        req: crate::model::PartialUpdateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.partial_update_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_cluster(
        &self,
        req: crate::model::CreateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cluster(
        &self,
        req: crate::model::GetClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Cluster> {
        self.inner.get_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_clusters(
        &self,
        req: crate::model::ListClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListClustersResponse> {
        self.inner.list_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_cluster(
        &self,
        req: crate::model::Cluster,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn partial_update_cluster(
        &self,
        req: crate::model::PartialUpdateClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.partial_update_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_cluster(
        &self,
        req: crate::model::DeleteClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_app_profile(
        &self,
        req: crate::model::CreateAppProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AppProfile> {
        self.inner.create_app_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_app_profile(
        &self,
        req: crate::model::GetAppProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AppProfile> {
        self.inner.get_app_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_app_profiles(
        &self,
        req: crate::model::ListAppProfilesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAppProfilesResponse> {
        self.inner.list_app_profiles(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_app_profile(
        &self,
        req: crate::model::UpdateAppProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_app_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_app_profile(
        &self,
        req: crate::model::DeleteAppProfileRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_app_profile(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_hot_tablets(
        &self,
        req: crate::model::ListHotTabletsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHotTabletsResponse> {
        self.inner.list_hot_tablets(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_logical_view(
        &self,
        req: crate::model::CreateLogicalViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_logical_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_logical_view(
        &self,
        req: crate::model::GetLogicalViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LogicalView> {
        self.inner.get_logical_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_logical_views(
        &self,
        req: crate::model::ListLogicalViewsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLogicalViewsResponse> {
        self.inner.list_logical_views(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_logical_view(
        &self,
        req: crate::model::UpdateLogicalViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_logical_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_logical_view(
        &self,
        req: crate::model::DeleteLogicalViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_logical_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_materialized_view(
        &self,
        req: crate::model::CreateMaterializedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_materialized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_materialized_view(
        &self,
        req: crate::model::GetMaterializedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MaterializedView> {
        self.inner.get_materialized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_materialized_views(
        &self,
        req: crate::model::ListMaterializedViewsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMaterializedViewsResponse> {
        self.inner.list_materialized_views(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_materialized_view(
        &self,
        req: crate::model::UpdateMaterializedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_materialized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_materialized_view(
        &self,
        req: crate::model::DeleteMaterializedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_materialized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements a [BigtableTableAdmin](super::stub::BigtableTableAdmin) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct BigtableTableAdmin<T>
where
    T: super::stub::BigtableTableAdmin + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> BigtableTableAdmin<T>
where
    T: super::stub::BigtableTableAdmin + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::BigtableTableAdmin for BigtableTableAdmin<T>
where
    T: super::stub::BigtableTableAdmin + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_table(
        &self,
        req: crate::model::CreateTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        self.inner.create_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_table_from_snapshot(
        &self,
        req: crate::model::CreateTableFromSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_table_from_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_tables(
        &self,
        req: crate::model::ListTablesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListTablesResponse> {
        self.inner.list_tables(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_table(
        &self,
        req: crate::model::GetTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        self.inner.get_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_table(
        &self,
        req: crate::model::UpdateTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_table(
        &self,
        req: crate::model::DeleteTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn undelete_table(
        &self,
        req: crate::model::UndeleteTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.undelete_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_authorized_view(
        &self,
        req: crate::model::CreateAuthorizedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_authorized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_authorized_views(
        &self,
        req: crate::model::ListAuthorizedViewsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAuthorizedViewsResponse> {
        self.inner.list_authorized_views(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_authorized_view(
        &self,
        req: crate::model::GetAuthorizedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AuthorizedView> {
        self.inner.get_authorized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_authorized_view(
        &self,
        req: crate::model::UpdateAuthorizedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_authorized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_authorized_view(
        &self,
        req: crate::model::DeleteAuthorizedViewRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_authorized_view(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn modify_column_families(
        &self,
        req: crate::model::ModifyColumnFamiliesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Table> {
        self.inner.modify_column_families(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn drop_row_range(
        &self,
        req: crate::model::DropRowRangeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.drop_row_range(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_consistency_token(
        &self,
        req: crate::model::GenerateConsistencyTokenRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateConsistencyTokenResponse> {
        self.inner.generate_consistency_token(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn check_consistency(
        &self,
        req: crate::model::CheckConsistencyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CheckConsistencyResponse> {
        self.inner.check_consistency(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn snapshot_table(
        &self,
        req: crate::model::SnapshotTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.snapshot_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_snapshot(
        &self,
        req: crate::model::GetSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Snapshot> {
        self.inner.get_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_snapshots(
        &self,
        req: crate::model::ListSnapshotsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSnapshotsResponse> {
        self.inner.list_snapshots(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_snapshot(
        &self,
        req: crate::model::DeleteSnapshotRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_snapshot(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_backup(
        &self,
        req: crate::model::CreateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_backup(
        &self,
        req: crate::model::GetBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Backup> {
        self.inner.get_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_backup(
        &self,
        req: crate::model::UpdateBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Backup> {
        self.inner.update_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_backup(
        &self,
        req: crate::model::DeleteBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_backups(
        &self,
        req: crate::model::ListBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBackupsResponse> {
        self.inner.list_backups(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn restore_table(
        &self,
        req: crate::model::RestoreTableRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.restore_table(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn copy_backup(
        &self,
        req: crate::model::CopyBackupRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.copy_backup(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.get_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_error_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        self.inner.get_polling_error_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
