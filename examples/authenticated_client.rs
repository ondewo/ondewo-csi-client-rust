// Copyright 2021-2026 ONDEWO GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Call an ONDEWO CSI server with a Keycloak bearer token.
//!
//! This is the crate's usage snippet. It lives here rather than in a doc comment because doctests
//! are disabled crate-wide (see the `doctest = false` note in `Cargo.toml`); as an example it is
//! still compiled by `cargo test` and `cargo build --examples`, so it cannot rot.
//!
//! ```sh
//! ONDEWO_CSI_HOST=https://csi.example.com:443 \
//! ONDEWO_CSI_ACCESS_TOKEN=<keycloak access token> \
//! ONDEWO_CSI_CAI_TOKEN=<cai token> \
//!   cargo run --example authenticated_client
//! ```

use std::env;
use std::error::Error;

use ondewo_csi_client::api::ondewo::csi;
use ondewo_csi_client::api::ondewo::csi::conversations_client::ConversationsClient;
use ondewo_csi_client::auth::BearerTokenInterceptor;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let host = env::var("ONDEWO_CSI_HOST")?;
    let access_token = env::var("ONDEWO_CSI_ACCESS_TOKEN")?;

    let mut interceptor = BearerTokenInterceptor::new(&access_token)?;
    if let Ok(cai_token) = env::var("ONDEWO_CSI_CAI_TOKEN") {
        interceptor = interceptor.with_cai_token(&cai_token)?;
    }

    let channel = Endpoint::from_shared(host)?.connect().await?;
    let mut client = ConversationsClient::with_interceptor(channel, interceptor);

    let response = client
        .list_s2s_pipelines(csi::ListS2sPipelinesRequest {})
        .await?
        .into_inner();

    for pipeline in response.pipelines {
        println!(
            "{} (s2t {}, nlu {}, t2s {})",
            pipeline.id,
            pipeline.s2t_pipeline_id,
            pipeline.nlu_project_id,
            pipeline.t2s_pipeline_id
        );
    }
    Ok(())
}
