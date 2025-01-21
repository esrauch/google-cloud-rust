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

/// Implements a [InstanceAdmin](crate::traits::) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct InstanceAdmin<T>
where
    T: crate::traits::InstanceAdmin + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> InstanceAdmin<T>
where
    T: crate::traits::InstanceAdmin + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::traits::InstanceAdmin for InstanceAdmin<T>
where
    T: crate::traits::InstanceAdmin + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_instance_configs(
        &self,
        req: crate::model::ListInstanceConfigsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstanceConfigsResponse> {
        self.inner.list_instance_configs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_instance_config(
        &self,
        req: crate::model::GetInstanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InstanceConfig> {
        self.inner.get_instance_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_instance_config(
        &self,
        req: crate::model::CreateInstanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_instance_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_instance_config(
        &self,
        req: crate::model::UpdateInstanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_instance_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_instance_config(
        &self,
        req: crate::model::DeleteInstanceConfigRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_instance_config(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_instance_config_operations(
        &self,
        req: crate::model::ListInstanceConfigOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstanceConfigOperationsResponse> {
        self.inner
            .list_instance_config_operations(req, options)
            .await
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
    async fn list_instance_partitions(
        &self,
        req: crate::model::ListInstancePartitionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstancePartitionsResponse> {
        self.inner.list_instance_partitions(req, options).await
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
    async fn create_instance(
        &self,
        req: crate::model::CreateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_instance(
        &self,
        req: crate::model::UpdateInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_instance(
        &self,
        req: crate::model::DeleteInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_instance(req, options).await
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
    async fn get_instance_partition(
        &self,
        req: crate::model::GetInstancePartitionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::InstancePartition> {
        self.inner.get_instance_partition(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_instance_partition(
        &self,
        req: crate::model::CreateInstancePartitionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_instance_partition(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_instance_partition(
        &self,
        req: crate::model::DeleteInstancePartitionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_instance_partition(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_instance_partition(
        &self,
        req: crate::model::UpdateInstancePartitionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.update_instance_partition(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_instance_partition_operations(
        &self,
        req: crate::model::ListInstancePartitionOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListInstancePartitionOperationsResponse> {
        self.inner
            .list_instance_partition_operations(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn move_instance(
        &self,
        req: crate::model::MoveInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.move_instance(req, options).await
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
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}
