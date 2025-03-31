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

#[cfg(all(test, feature = "run-integration-tests"))]
mod driver {
    use rand::{Rng, distr::Alphanumeric};

    const SECRET_ID_LENGTH: usize = 32;

    #[tokio::test(flavor = "multi_thread")]
    async fn lro_start() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::lro::start(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn lro_automatic() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::lro::automatic(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn lro_polling() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::lro::polling(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn polling_policies_client_backoff() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::polling_policies::client_backoff(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn polling_policies_rpc_backoff() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::polling_policies::rpc_backoff(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn polling_policies_client_errors() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::polling_policies::client_errors(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn polling_policies_rpc_errors() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::polling_policies::rpc_errors(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn retry_policies_client() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::retry_policies::client_retry(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn retry_policies_client_full() -> user_guide_samples::Result<()> {
        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        user_guide_samples::retry_policies::client_retry_full(&project_id).await
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn retry_policies_request() -> user_guide_samples::Result<()> {
        use google_cloud_gax::options::RequestOptionsBuilder;
        use google_cloud_gax::retry_policy::AlwaysRetry;
        use google_cloud_gax::retry_policy::RetryPolicyExt;
        use google_cloud_secretmanager_v1 as sm;
        use std::time::Duration;

        let project_id = std::env::var("GOOGLE_CLOUD_PROJECT").unwrap();
        let secret_id: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(SECRET_ID_LENGTH)
            .map(char::from)
            .collect();

        let client = sm::client::SecretManagerService::builder().build().await?;
        // The sample will delete this secret. If that fails, the cleanup step
        // for the integration tests will garbage collect it in a couple of
        // days.
        let _ = client
            .create_secret(format!("projects/{project_id}"))
            .with_retry_policy(
                AlwaysRetry
                    .with_attempt_limit(5)
                    .with_time_limit(Duration::from_secs(15)),
            )
            .set_secret_id(&secret_id)
            .set_secret(
                sm::model::Secret::new()
                    .set_replication(sm::model::Replication::new().set_replication(
                        sm::model::replication::Replication::Automatic(
                            sm::model::replication::Automatic::new().into(),
                        ),
                    ))
                    .set_labels([("integration-test", "true")]),
            )
            .send()
            .await?;

        user_guide_samples::retry_policies::request_retry(&client, &project_id, &secret_id).await
    }
}
