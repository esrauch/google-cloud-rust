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

/// Implements a [AnalyticsHubService](super::stub::AnalyticsHubService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct AnalyticsHubService<T>
where
    T: super::stub::AnalyticsHubService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> AnalyticsHubService<T>
where
    T: super::stub::AnalyticsHubService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> super::stub::AnalyticsHubService for AnalyticsHubService<T>
where
    T: super::stub::AnalyticsHubService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn list_data_exchanges(
        &self,
        req: crate::model::ListDataExchangesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDataExchangesResponse> {
        self.inner.list_data_exchanges(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_org_data_exchanges(
        &self,
        req: crate::model::ListOrgDataExchangesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListOrgDataExchangesResponse> {
        self.inner.list_org_data_exchanges(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_data_exchange(
        &self,
        req: crate::model::GetDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DataExchange> {
        self.inner.get_data_exchange(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_data_exchange(
        &self,
        req: crate::model::CreateDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DataExchange> {
        self.inner.create_data_exchange(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_data_exchange(
        &self,
        req: crate::model::UpdateDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::DataExchange> {
        self.inner.update_data_exchange(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_data_exchange(
        &self,
        req: crate::model::DeleteDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_data_exchange(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_listings(
        &self,
        req: crate::model::ListListingsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListListingsResponse> {
        self.inner.list_listings(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_listing(
        &self,
        req: crate::model::GetListingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Listing> {
        self.inner.get_listing(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_listing(
        &self,
        req: crate::model::CreateListingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Listing> {
        self.inner.create_listing(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_listing(
        &self,
        req: crate::model::UpdateListingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Listing> {
        self.inner.update_listing(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_listing(
        &self,
        req: crate::model::DeleteListingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<()> {
        self.inner.delete_listing(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn subscribe_listing(
        &self,
        req: crate::model::SubscribeListingRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SubscribeListingResponse> {
        self.inner.subscribe_listing(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn subscribe_data_exchange(
        &self,
        req: crate::model::SubscribeDataExchangeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.subscribe_data_exchange(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn refresh_subscription(
        &self,
        req: crate::model::RefreshSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.refresh_subscription(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_subscription(
        &self,
        req: crate::model::GetSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Subscription> {
        self.inner.get_subscription(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_subscriptions(
        &self,
        req: crate::model::ListSubscriptionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSubscriptionsResponse> {
        self.inner.list_subscriptions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_shared_resource_subscriptions(
        &self,
        req: crate::model::ListSharedResourceSubscriptionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSharedResourceSubscriptionsResponse> {
        self.inner
            .list_shared_resource_subscriptions(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn revoke_subscription(
        &self,
        req: crate::model::RevokeSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RevokeSubscriptionResponse> {
        self.inner.revoke_subscription(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_subscription(
        &self,
        req: crate::model::DeleteSubscriptionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.delete_subscription(req, options).await
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
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
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
