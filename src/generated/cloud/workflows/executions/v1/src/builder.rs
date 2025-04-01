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

pub mod executions {
    use crate::Result;
    use std::sync::Arc;

    /// A builder for [Executions][super::super::client::Executions].
    ///
    /// ```
    /// # tokio_test::block_on(async {
    /// # use google_cloud_workflows_executions_v1::*;
    /// # use builder::executions::ClientBuilder;
    /// # use client::Executions;
    /// let builder : ClientBuilder = Executions::builder();
    /// let client = builder
    ///     .with_endpoint("https://workflowexecutions.googleapis.com")
    ///     .build().await?;
    /// # gax::Result::<()>::Ok(()) });
    /// ```
    pub type ClientBuilder =
        gax::client_builder::ClientBuilder<client::Factory, gaxi::options::Credentials>;

    pub(crate) mod client {
        use super::super::super::client::Executions;
        pub struct Factory;
        impl gax::client_builder::internal::ClientFactory for Factory {
            type Client = Executions;
            type Credentials = gaxi::options::Credentials;
            async fn build(self, config: gaxi::options::ClientConfig) -> gax::Result<Self::Client> {
                Self::Client::new(config).await
            }
        }
    }

    /// Common implementation for [super::super::client::Executions] request builders.
    #[derive(Clone, Debug)]
    pub(crate) struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn super::super::stub::dynamic::Executions>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::Executions>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for [Executions::list_executions][super::super::client::Executions::list_executions] calls.
    #[derive(Clone, Debug)]
    pub struct ListExecutions(RequestBuilder<crate::model::ListExecutionsRequest>);

    impl ListExecutions {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::Executions>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListExecutionsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListExecutionsResponse> {
            (*self.0.stub)
                .list_executions(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        pub async fn paginator(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListExecutionsResponse, gax::error::Error>
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

        /// Sets the value of [parent][crate::model::ListExecutionsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListExecutionsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListExecutionsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [view][crate::model::ListExecutionsRequest::view].
        pub fn set_view<T: Into<crate::model::ExecutionView>>(mut self, v: T) -> Self {
            self.0.request.view = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListExecutionsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [order_by][crate::model::ListExecutionsRequest::order_by].
        pub fn set_order_by<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.order_by = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for ListExecutions {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [Executions::create_execution][super::super::client::Executions::create_execution] calls.
    #[derive(Clone, Debug)]
    pub struct CreateExecution(RequestBuilder<crate::model::CreateExecutionRequest>);

    impl CreateExecution {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::Executions>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateExecutionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Execution> {
            (*self.0.stub)
                .create_execution(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [parent][crate::model::CreateExecutionRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [execution][crate::model::CreateExecutionRequest::execution].
        pub fn set_execution<T: Into<std::option::Option<crate::model::Execution>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.execution = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CreateExecution {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [Executions::get_execution][super::super::client::Executions::get_execution] calls.
    #[derive(Clone, Debug)]
    pub struct GetExecution(RequestBuilder<crate::model::GetExecutionRequest>);

    impl GetExecution {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::Executions>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetExecutionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Execution> {
            (*self.0.stub)
                .get_execution(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetExecutionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [view][crate::model::GetExecutionRequest::view].
        pub fn set_view<T: Into<crate::model::ExecutionView>>(mut self, v: T) -> Self {
            self.0.request.view = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for GetExecution {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for [Executions::cancel_execution][super::super::client::Executions::cancel_execution] calls.
    #[derive(Clone, Debug)]
    pub struct CancelExecution(RequestBuilder<crate::model::CancelExecutionRequest>);

    impl CancelExecution {
        pub(crate) fn new(stub: Arc<dyn super::super::stub::dynamic::Executions>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CancelExecutionRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Execution> {
            (*self.0.stub)
                .cancel_execution(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::CancelExecutionRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    #[doc(hidden)]
    impl gax::options::internal::RequestBuilder for CancelExecution {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
