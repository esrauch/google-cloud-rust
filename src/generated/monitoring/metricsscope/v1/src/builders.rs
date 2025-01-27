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

pub mod metrics_scopes {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::MetricsScopes] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a MetricsScopes::get_metrics_scope call.
    #[derive(Clone, Debug)]
    pub struct GetMetricsScope(RequestBuilder<crate::model::GetMetricsScopeRequest>);

    impl GetMetricsScope {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetMetricsScopeRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::MetricsScope> {
            (*self.0.stub)
                .get_metrics_scope(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetMetricsScopeRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetMetricsScope {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MetricsScopes::list_metrics_scopes_by_monitored_project call.
    #[derive(Clone, Debug)]
    pub struct ListMetricsScopesByMonitoredProject(
        RequestBuilder<crate::model::ListMetricsScopesByMonitoredProjectRequest>,
    );

    impl ListMetricsScopesByMonitoredProject {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListMetricsScopesByMonitoredProjectRequest>>(
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
        pub async fn send(
            self,
        ) -> Result<crate::model::ListMetricsScopesByMonitoredProjectResponse> {
            (*self.0.stub)
                .list_metrics_scopes_by_monitored_project(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [monitored_resource_container][crate::model::ListMetricsScopesByMonitoredProjectRequest::monitored_resource_container].
        pub fn set_monitored_resource_container<T: Into<std::string::String>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.monitored_resource_container = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListMetricsScopesByMonitoredProject {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MetricsScopes::create_monitored_project call.
    #[derive(Clone, Debug)]
    pub struct CreateMonitoredProject(RequestBuilder<crate::model::CreateMonitoredProjectRequest>);

    impl CreateMonitoredProject {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CreateMonitoredProjectRequest>>(
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
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [create_monitored_project][crate::client::MetricsScopes::create_monitored_project].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .create_monitored_project(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `create_monitored_project`.
        pub fn poller(
            self,
        ) -> impl lro::Poller<crate::model::MonitoredProject, crate::model::OperationMetadata>
        {
            type Operation =
                lro::Operation<crate::model::MonitoredProject, crate::model::OperationMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [parent][crate::model::CreateMonitoredProjectRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [monitored_project][crate::model::CreateMonitoredProjectRequest::monitored_project].
        pub fn set_monitored_project<
            T: Into<std::option::Option<crate::model::MonitoredProject>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.monitored_project = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for CreateMonitoredProject {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MetricsScopes::delete_monitored_project call.
    #[derive(Clone, Debug)]
    pub struct DeleteMonitoredProject(RequestBuilder<crate::model::DeleteMonitoredProjectRequest>);

    impl DeleteMonitoredProject {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::DeleteMonitoredProjectRequest>>(
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
        ///
        /// # Long running operations
        ///
        /// This starts, but does not poll, a longrunning operation. More information
        /// on [delete_monitored_project][crate::client::MetricsScopes::delete_monitored_project].
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .delete_monitored_project(self.0.request, self.0.options)
                .await
        }

        /// Creates a [Poller][lro::Poller] to work with `delete_monitored_project`.
        pub fn poller(self) -> impl lro::Poller<wkt::Empty, crate::model::OperationMetadata> {
            type Operation = lro::Operation<wkt::Empty, crate::model::OperationMetadata>;
            let polling_policy = self.0.stub.get_polling_policy(&self.0.options);
            let polling_backoff_policy = self.0.stub.get_polling_backoff_policy(&self.0.options);

            let stub = self.0.stub.clone();
            let mut options = self.0.options.clone();
            options.set_retry_policy(gax::retry_policy::NeverRetry);
            let query = move |name| {
                let stub = stub.clone();
                let options = options.clone();
                async {
                    let op = GetOperation::new(stub)
                        .set_name(name)
                        .with_options(options)
                        .send()
                        .await?;
                    Ok(Operation::new(op))
                }
            };

            let start = move || async {
                let op = self.send().await?;
                Ok(Operation::new(op))
            };

            lro::new_poller(polling_policy, polling_backoff_policy, start, query)
        }

        /// Sets the value of [name][crate::model::DeleteMonitoredProjectRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for DeleteMonitoredProject {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a MetricsScopes::get_operation call.
    #[derive(Clone, Debug)]
    pub struct GetOperation(RequestBuilder<longrunning::model::GetOperationRequest>);

    impl GetOperation {
        pub(crate) fn new(stub: Arc<dyn crate::traits::dyntraits::MetricsScopes>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<longrunning::model::GetOperationRequest>>(
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
        pub async fn send(self) -> Result<longrunning::model::Operation> {
            (*self.0.stub)
                .get_operation(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][longrunning::model::GetOperationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOperation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
