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

/// Implements a [SecurityCenter](super::stub::SecurityCenter) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct SecurityCenter<T>
where
    T: super::stub::SecurityCenter + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> SecurityCenter<T>
where
    T: super::stub::SecurityCenter + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::SecurityCenter for SecurityCenter<T>
where
    T: super::stub::SecurityCenter + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn batch_create_resource_value_configs(
        &self,
        req: crate::model::BatchCreateResourceValueConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BatchCreateResourceValueConfigsResponse> {
        self.inner
            .batch_create_resource_value_configs(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn bulk_mute_findings(
        &self,
        req: crate::model::BulkMuteFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.bulk_mute_findings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_big_query_export(
        &self,
        req: crate::model::CreateBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BigQueryExport> {
        self.inner.create_big_query_export(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_finding(
        &self,
        req: crate::model::CreateFindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Finding> {
        self.inner.create_finding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_mute_config(
        &self,
        req: crate::model::CreateMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MuteConfig> {
        self.inner.create_mute_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_notification_config(
        &self,
        req: crate::model::CreateNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NotificationConfig> {
        self.inner.create_notification_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_source(
        &self,
        req: crate::model::CreateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Source> {
        self.inner.create_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_big_query_export(
        &self,
        req: crate::model::DeleteBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_big_query_export(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_mute_config(
        &self,
        req: crate::model::DeleteMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_mute_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_notification_config(
        &self,
        req: crate::model::DeleteNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_notification_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_resource_value_config(
        &self,
        req: crate::model::DeleteResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_resource_value_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_big_query_export(
        &self,
        req: crate::model::GetBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BigQueryExport> {
        self.inner.get_big_query_export(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_simulation(
        &self,
        req: crate::model::GetSimulationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Simulation> {
        self.inner.get_simulation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_valued_resource(
        &self,
        req: crate::model::GetValuedResourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ValuedResource> {
        self.inner.get_valued_resource(req, options).await
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
    async fn get_mute_config(
        &self,
        req: crate::model::GetMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MuteConfig> {
        self.inner.get_mute_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_notification_config(
        &self,
        req: crate::model::GetNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NotificationConfig> {
        self.inner.get_notification_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_resource_value_config(
        &self,
        req: crate::model::GetResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ResourceValueConfig> {
        self.inner.get_resource_value_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_source(
        &self,
        req: crate::model::GetSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Source> {
        self.inner.get_source(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn group_findings(
        &self,
        req: crate::model::GroupFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::GroupFindingsResponse> {
        self.inner.group_findings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_attack_paths(
        &self,
        req: crate::model::ListAttackPathsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAttackPathsResponse> {
        self.inner.list_attack_paths(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_big_query_exports(
        &self,
        req: crate::model::ListBigQueryExportsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListBigQueryExportsResponse> {
        self.inner.list_big_query_exports(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_findings(
        &self,
        req: crate::model::ListFindingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListFindingsResponse> {
        self.inner.list_findings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_mute_configs(
        &self,
        req: crate::model::ListMuteConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListMuteConfigsResponse> {
        self.inner.list_mute_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_notification_configs(
        &self,
        req: crate::model::ListNotificationConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListNotificationConfigsResponse> {
        self.inner.list_notification_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_resource_value_configs(
        &self,
        req: crate::model::ListResourceValueConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListResourceValueConfigsResponse> {
        self.inner.list_resource_value_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_sources(
        &self,
        req: crate::model::ListSourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSourcesResponse> {
        self.inner.list_sources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_valued_resources(
        &self,
        req: crate::model::ListValuedResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListValuedResourcesResponse> {
        self.inner.list_valued_resources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn set_finding_state(
        &self,
        req: crate::model::SetFindingStateRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Finding> {
        self.inner.set_finding_state(req, options).await
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
    async fn set_mute(
        &self,
        req: crate::model::SetMuteRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Finding> {
        self.inner.set_mute(req, options).await
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
    async fn update_big_query_export(
        &self,
        req: crate::model::UpdateBigQueryExportRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::BigQueryExport> {
        self.inner.update_big_query_export(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_external_system(
        &self,
        req: crate::model::UpdateExternalSystemRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalSystem> {
        self.inner.update_external_system(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_finding(
        &self,
        req: crate::model::UpdateFindingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Finding> {
        self.inner.update_finding(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_mute_config(
        &self,
        req: crate::model::UpdateMuteConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::MuteConfig> {
        self.inner.update_mute_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_notification_config(
        &self,
        req: crate::model::UpdateNotificationConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::NotificationConfig> {
        self.inner.update_notification_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_resource_value_config(
        &self,
        req: crate::model::UpdateResourceValueConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ResourceValueConfig> {
        self.inner.update_resource_value_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_security_marks(
        &self,
        req: crate::model::UpdateSecurityMarksRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecurityMarks> {
        self.inner.update_security_marks(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_source(
        &self,
        req: crate::model::UpdateSourceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Source> {
        self.inner.update_source(req, options).await
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
