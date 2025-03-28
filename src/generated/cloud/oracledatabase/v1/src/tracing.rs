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

/// Implements a [OracleDatabase](super::stub::OracleDatabase) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct OracleDatabase<T>
where
    T: super::stub::OracleDatabase + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> OracleDatabase<T>
where
    T: super::stub::OracleDatabase + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::OracleDatabase for OracleDatabase<T>
where
    T: super::stub::OracleDatabase + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_cloud_exadata_infrastructures(
        &self,
        req: crate::model::ListCloudExadataInfrastructuresRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCloudExadataInfrastructuresResponse> {
        self.inner
            .list_cloud_exadata_infrastructures(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_cloud_exadata_infrastructure(
        &self,
        req: crate::model::GetCloudExadataInfrastructureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CloudExadataInfrastructure> {
        self.inner
            .get_cloud_exadata_infrastructure(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn create_cloud_exadata_infrastructure(
        &self,
        req: crate::model::CreateCloudExadataInfrastructureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .create_cloud_exadata_infrastructure(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_cloud_exadata_infrastructure(
        &self,
        req: crate::model::DeleteCloudExadataInfrastructureRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .delete_cloud_exadata_infrastructure(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_cloud_vm_clusters(
        &self,
        req: crate::model::ListCloudVmClustersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListCloudVmClustersResponse> {
        self.inner.list_cloud_vm_clusters(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_cloud_vm_cluster(
        &self,
        req: crate::model::GetCloudVmClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::CloudVmCluster> {
        self.inner.get_cloud_vm_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_cloud_vm_cluster(
        &self,
        req: crate::model::CreateCloudVmClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_cloud_vm_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_cloud_vm_cluster(
        &self,
        req: crate::model::DeleteCloudVmClusterRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_cloud_vm_cluster(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_entitlements(
        &self,
        req: crate::model::ListEntitlementsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListEntitlementsResponse> {
        self.inner.list_entitlements(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_db_servers(
        &self,
        req: crate::model::ListDbServersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDbServersResponse> {
        self.inner.list_db_servers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_db_nodes(
        &self,
        req: crate::model::ListDbNodesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDbNodesResponse> {
        self.inner.list_db_nodes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_gi_versions(
        &self,
        req: crate::model::ListGiVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListGiVersionsResponse> {
        self.inner.list_gi_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_db_system_shapes(
        &self,
        req: crate::model::ListDbSystemShapesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDbSystemShapesResponse> {
        self.inner.list_db_system_shapes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_autonomous_databases(
        &self,
        req: crate::model::ListAutonomousDatabasesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAutonomousDatabasesResponse> {
        self.inner.list_autonomous_databases(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_autonomous_database(
        &self,
        req: crate::model::GetAutonomousDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AutonomousDatabase> {
        self.inner.get_autonomous_database(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_autonomous_database(
        &self,
        req: crate::model::CreateAutonomousDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_autonomous_database(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_autonomous_database(
        &self,
        req: crate::model::DeleteAutonomousDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_autonomous_database(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn restore_autonomous_database(
        &self,
        req: crate::model::RestoreAutonomousDatabaseRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.restore_autonomous_database(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn generate_autonomous_database_wallet(
        &self,
        req: crate::model::GenerateAutonomousDatabaseWalletRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GenerateAutonomousDatabaseWalletResponse> {
        self.inner
            .generate_autonomous_database_wallet(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_autonomous_db_versions(
        &self,
        req: crate::model::ListAutonomousDbVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAutonomousDbVersionsResponse> {
        self.inner.list_autonomous_db_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_autonomous_database_character_sets(
        &self,
        req: crate::model::ListAutonomousDatabaseCharacterSetsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAutonomousDatabaseCharacterSetsResponse> {
        self.inner
            .list_autonomous_database_character_sets(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_autonomous_database_backups(
        &self,
        req: crate::model::ListAutonomousDatabaseBackupsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAutonomousDatabaseBackupsResponse> {
        self.inner
            .list_autonomous_database_backups(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
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
