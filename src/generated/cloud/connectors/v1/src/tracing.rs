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

/// Implements a [Connectors](super::stub::Connectors) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Connectors<T>
where
    T: super::stub::Connectors + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Connectors<T>
where
    T: super::stub::Connectors + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::Connectors for Connectors<T>
where
    T: super::stub::Connectors + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_connections(
        &self,
        req: crate::model::ListConnectionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListConnectionsResponse> {
        self.inner.list_connections(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection(
        &self,
        req: crate::model::GetConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Connection> {
        self.inner.get_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_connection(
        &self,
        req: crate::model::CreateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_connection(
        &self,
        req: crate::model::UpdateConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_connection(
        &self,
        req: crate::model::DeleteConnectionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_connection(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_providers(
        &self,
        req: crate::model::ListProvidersRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListProvidersResponse> {
        self.inner.list_providers(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_provider(
        &self,
        req: crate::model::GetProviderRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Provider> {
        self.inner.get_provider(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_connectors(
        &self,
        req: crate::model::ListConnectorsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListConnectorsResponse> {
        self.inner.list_connectors(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connector(
        &self,
        req: crate::model::GetConnectorRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Connector> {
        self.inner.get_connector(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_connector_versions(
        &self,
        req: crate::model::ListConnectorVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListConnectorVersionsResponse> {
        self.inner.list_connector_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connector_version(
        &self,
        req: crate::model::GetConnectorVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ConnectorVersion> {
        self.inner.get_connector_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_connection_schema_metadata(
        &self,
        req: crate::model::GetConnectionSchemaMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ConnectionSchemaMetadata> {
        self.inner
            .get_connection_schema_metadata(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn refresh_connection_schema_metadata(
        &self,
        req: crate::model::RefreshConnectionSchemaMetadataRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner
            .refresh_connection_schema_metadata(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_runtime_entity_schemas(
        &self,
        req: crate::model::ListRuntimeEntitySchemasRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRuntimeEntitySchemasResponse> {
        self.inner.list_runtime_entity_schemas(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_runtime_action_schemas(
        &self,
        req: crate::model::ListRuntimeActionSchemasRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRuntimeActionSchemasResponse> {
        self.inner.list_runtime_action_schemas(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_runtime_config(
        &self,
        req: crate::model::GetRuntimeConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RuntimeConfig> {
        self.inner.get_runtime_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_global_settings(
        &self,
        req: crate::model::GetGlobalSettingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Settings> {
        self.inner.get_global_settings(req, options).await
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
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<iam_v1::model::Policy> {
        self.inner.set_iam_policy(req, options).await
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
