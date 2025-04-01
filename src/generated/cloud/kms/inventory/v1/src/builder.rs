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

pub mod key_dashboard_service {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [KeyDashboardService][super::super::client::KeyDashboardService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_kms_inventory_v1::*;
    /// # use builder::key_dashboard_service::ClientBuilder;
    /// # use client::KeyDashboardService;
    /// let builder : ClientBuilder = KeyDashboardService::builder();
    /// let client = builder
    ///     .with_endpoint("https://kmsinventory.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::KeyDashboardService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = KeyDashboardService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::KeyDashboardService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::KeyDashboardService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::KeyDashboardService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [KeyDashboardService::list_crypto_keys][super::super::client::KeyDashboardService::list_crypto_keys] calls.
    #[derive(Clone, Debug)]
    pub struct ListCryptoKeys(RequestBuilder<crate::model::ListCryptoKeysRequest>);

    impl ListCryptoKeys {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::KeyDashboardService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListCryptoKeysRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListCryptoKeysResponse> {
            (*self.0.stub)
                .list_crypto_keys(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListCryptoKeysResponse, gax::error::Error>
        {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListCryptoKeysRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListCryptoKeysRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListCryptoKeysRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListCryptoKeys {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod key_tracking_service {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [KeyTrackingService][super::super::client::KeyTrackingService].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_kms_inventory_v1::*;
    /// # use builder::key_tracking_service::ClientBuilder;
    /// # use client::KeyTrackingService;
    /// let builder : ClientBuilder = KeyTrackingService::builder();
    /// let client = builder
    ///     .with_endpoint("https://kmsinventory.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::KeyTrackingService;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = KeyTrackingService;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::KeyTrackingService] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::KeyTrackingService>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::KeyTrackingService>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [KeyTrackingService::get_protected_resources_summary][super::super::client::KeyTrackingService::get_protected_resources_summary] calls.
    #[derive(Clone, Debug)]
    pub struct GetProtectedResourcesSummary(
        RequestBuilder<crate::model::GetProtectedResourcesSummaryRequest>,
    );

    impl GetProtectedResourcesSummary {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::KeyTrackingService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetProtectedResourcesSummaryRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ProtectedResourcesSummary> {
            (*self.0.stub)
                .get_protected_resources_summary(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetProtectedResourcesSummaryRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetProtectedResourcesSummary {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [KeyTrackingService::search_protected_resources][super::super::client::KeyTrackingService::search_protected_resources] calls.
    #[derive(Clone, Debug)]
    pub struct SearchProtectedResources(
        RequestBuilder<crate::model::SearchProtectedResourcesRequest>,
    );

    impl SearchProtectedResources {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::KeyTrackingService>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::SearchProtectedResourcesRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::SearchProtectedResourcesResponse> {
            (*self.0.stub)
                .search_protected_resources(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<
            crate::model::SearchProtectedResourcesResponse,
            gax::error::Error,
        > {
            use std::clone::Clone;
            let token = self.0.request.page_token.clone();
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [scope][crate::model::SearchProtectedResourcesRequest::scope].
        pub fn set_scope<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.scope = v.into();
            self
        }

        /// Sets the value of [crypto_key][crate::model::SearchProtectedResourcesRequest::crypto_key].
        pub fn set_crypto_key<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.crypto_key = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::SearchProtectedResourcesRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::SearchProtectedResourcesRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [resource_types][crate::model::SearchProtectedResourcesRequest::resource_types].
        pub fn set_resource_types<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<std::string::String>,
        {
            use std::iter::Iterator;
            self.0.request.resource_types = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for SearchProtectedResources {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
