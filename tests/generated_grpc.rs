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

//! End-to-end tests for the GENERATED tonic service stubs.
//!
//! `ondewo-csi-api` declares exactly one service, `Conversations`, so the fake below implements
//! all nine of its RPCs - seven unary, the bidirectional `S2sStream` and the server-streaming
//! `GetControlStream`. It is served over a loopback socket and driven by the generated
//! `ConversationsClient`, so a request really is encoded, routed by its
//! `/ondewo.csi.Conversations/<Method>` path, decoded, answered and decoded again. That is what
//! catches a service the generator wired to the wrong path, a codec mismatch, or a method that
//! silently went missing.
//!
//! No network beyond `127.0.0.1` and no ONDEWO server is involved.

use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use ondewo_csi_client::api::ondewo::csi::conversations_client::ConversationsClient;
use ondewo_csi_client::api::ondewo::csi::conversations_server::{
    Conversations, ConversationsServer,
};
use ondewo_csi_client::api::ondewo::{csi, t2s};
use ondewo_csi_client::auth::{
    BearerTokenInterceptor, AUTHORIZATION_METADATA_KEY, CAI_TOKEN_METADATA_KEY,
};
use tokio::net::TcpListener;
use tokio_stream::{wrappers::TcpListenerStream, Stream, StreamExt};
use tonic::transport::{Channel, Endpoint, Server};
use tonic::{Code, Request, Response, Status};

/// The pipeline id `get_s2s_pipeline` answers with `not_found` for, so the error path is
/// exercised too.
const MISSING_PIPELINE: &str = "does-not-exist";

/// Metadata the fake server captured from the last request it handled.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct SeenMetadata {
    authorization: Option<String>,
    cai_token: Option<String>,
}

/// A minimal in-process implementation of the generated `Conversations` service.
#[derive(Clone, Default)]
struct FakeConversations {
    seen: Arc<Mutex<SeenMetadata>>,
}

impl FakeConversations {
    fn record<T>(&self, request: &Request<T>) {
        let read = |key: &str| {
            request
                .metadata()
                .get(key)
                .map(|value| value.to_str().unwrap().to_string())
        };
        *self.seen.lock().unwrap() = SeenMetadata {
            authorization: read(AUTHORIZATION_METADATA_KEY),
            cai_token: read(CAI_TOKEN_METADATA_KEY),
        };
    }

    fn seen(&self) -> SeenMetadata {
        self.seen.lock().unwrap().clone()
    }
}

/// One synthesized chunk carrying an EXPLICITLY EMPTY `instruction`. `t2s::RequestConfig.instruction`
/// is a proto3 `optional` string, so `Some("")` and `None` are two different values; echoing the
/// first back is what proves the presence bit survives a real gRPC hop.
fn synthesized(utterance_id: &str, chunk_index: i32) -> csi::S2sStreamResponse {
    csi::S2sStreamResponse {
        utterance_id: utterance_id.to_string(),
        chunk_index,
        last_chunk: chunk_index == 1,
        turn_epoch: 7,
        response: Some(csi::s2s_stream_response::Response::SynthesizeResponse(
            t2s::SynthesizeResponse {
                audio_uuid: format!("audio-{chunk_index}"),
                text: "hello".to_string(),
                config: Some(t2s::RequestConfig {
                    t2s_pipeline_id: "t2s-1".to_string(),
                    instruction: Some(String::new()),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )),
    }
}

#[tonic::async_trait]
impl Conversations for FakeConversations {
    async fn create_s2s_pipeline(
        &self,
        request: Request<csi::S2sPipeline>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn get_s2s_pipeline(
        &self,
        request: Request<csi::S2sPipelineId>,
    ) -> Result<Response<csi::S2sPipeline>, Status> {
        self.record(&request);
        let id = request.into_inner().id;
        if id == MISSING_PIPELINE {
            return Err(Status::not_found(format!("no s2s pipeline named {id}")));
        }
        Ok(Response::new(csi::S2sPipeline {
            id,
            s2t_pipeline_id: "s2t-1".to_string(),
            nlu_project_id: "project-1".to_string(),
            nlu_language_code: "de".to_string(),
            t2s_pipeline_id: "t2s-1".to_string(),
        }))
    }

    async fn update_s2s_pipeline(
        &self,
        request: Request<csi::S2sPipeline>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn delete_s2s_pipeline(
        &self,
        request: Request<csi::S2sPipelineId>,
    ) -> Result<Response<()>, Status> {
        self.record(&request);
        Ok(Response::new(()))
    }

    async fn list_s2s_pipelines(
        &self,
        request: Request<csi::ListS2sPipelinesRequest>,
    ) -> Result<Response<csi::ListS2sPipelinesResponse>, Status> {
        self.record(&request);
        Ok(Response::new(csi::ListS2sPipelinesResponse {
            pipelines: vec![csi::S2sPipeline {
                id: "pipeline-1".to_string(),
                s2t_pipeline_id: "s2t-1".to_string(),
                nlu_project_id: "project-1".to_string(),
                nlu_language_code: "de".to_string(),
                t2s_pipeline_id: "t2s-1".to_string(),
            }],
        }))
    }

    type S2sStreamStream =
        Pin<Box<dyn Stream<Item = Result<csi::S2sStreamResponse, Status>> + Send>>;

    /// Drains the inbound half before answering, so the test really exercises both directions of
    /// the bidirectional stream rather than only the server-to-client one.
    async fn s2s_stream(
        &self,
        request: Request<tonic::Streaming<csi::S2sStreamRequest>>,
    ) -> Result<Response<Self::S2sStreamStream>, Status> {
        self.record(&request);
        let mut inbound = request.into_inner();
        let mut session_id = String::new();
        let mut chunks = 0;
        while let Some(message) = inbound.next().await {
            let message = message?;
            if !message.session_id.is_empty() {
                session_id = message.session_id;
            }
            chunks += 1;
        }
        if chunks == 0 {
            return Err(Status::invalid_argument("the request stream was empty"));
        }
        let utterance_id = format!("{session_id}/utterances/{chunks}");
        let responses = vec![
            Ok(synthesized(&utterance_id, 0)),
            Ok(synthesized(&utterance_id, 1)),
        ];
        Ok(Response::new(Box::pin(tokio_stream::iter(responses))))
    }

    async fn check_upstream_health(
        &self,
        request: Request<()>,
    ) -> Result<Response<csi::CheckUpstreamHealthResponse>, Status> {
        self.record(&request);
        Ok(Response::new(csi::CheckUpstreamHealthResponse {
            s2t_status: Some(ondewo_csi_client::api::google::rpc::Status {
                code: 0,
                message: "ok".to_string(),
                details: vec![],
            }),
            nlu_status: None,
            t2s_status: None,
        }))
    }

    type GetControlStreamStream =
        Pin<Box<dyn Stream<Item = Result<csi::ControlStreamResponse, Status>> + Send>>;

    async fn get_control_stream(
        &self,
        request: Request<csi::ControlStreamRequest>,
    ) -> Result<Response<Self::GetControlStreamStream>, Status> {
        self.record(&request);
        let responses = vec![
            Ok(csi::ControlStreamResponse {
                control_status: csi::ControlStatus::Ok as i32,
                epoch: 1,
            }),
            Ok(csi::ControlStreamResponse {
                control_status: csi::ControlStatus::BargeIn as i32,
                epoch: 2,
            }),
        ];
        Ok(Response::new(Box::pin(tokio_stream::iter(responses))))
    }

    async fn set_control_status(
        &self,
        request: Request<csi::SetControlStatusRequest>,
    ) -> Result<Response<csi::SetControlStatusResponse>, Status> {
        self.record(&request);
        Ok(Response::new(csi::SetControlStatusResponse {
            old_control_status: csi::ControlStatus::Ok as i32,
            new_control_status: request.into_inner().control_status,
        }))
    }
}

/// Start the generated server on an ephemeral loopback port and return it with its address.
///
/// The server task is detached; it ends when the test process does.
async fn start_server() -> (FakeConversations, SocketAddr) {
    let service = FakeConversations::default();
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local_addr");

    let served = service.clone();
    tokio::spawn(async move {
        Server::builder()
            .add_service(ConversationsServer::new(served))
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await
            .expect("the in-process gRPC server must not fail");
    });

    (service, addr)
}

async fn connect(addr: SocketAddr) -> Channel {
    Endpoint::from_shared(format!("http://{addr}"))
        .expect("endpoint")
        .connect_timeout(Duration::from_secs(10))
        .connect()
        .await
        .expect("the in-process gRPC server must accept a connection")
}

#[tokio::test]
async fn a_unary_call_round_trips_through_the_generated_client_and_server() {
    let (_service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    let response = client
        .list_s2s_pipelines(csi::ListS2sPipelinesRequest {})
        .await
        .expect("ListS2sPipelines must succeed")
        .into_inner();

    assert_eq!(response.pipelines.len(), 1);
    assert_eq!(response.pipelines[0].id, "pipeline-1");
    assert_eq!(response.pipelines[0].nlu_language_code, "de");
}

/// The bidirectional `S2sStream` is the RPC this API exists for: audio in, synthesized audio out.
/// Both directions have to carry real messages.
#[tokio::test]
async fn a_bidirectional_stream_round_trips_in_both_directions() {
    let (_service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    let requests = tokio_stream::iter(vec![
        csi::S2sStreamRequest {
            pipeline_id: "pipeline-1".to_string(),
            session_id: "projects/p/agent/sessions/s".to_string(),
            audio: vec![0x01, 0x02],
            ..Default::default()
        },
        csi::S2sStreamRequest {
            pipeline_id: "pipeline-1".to_string(),
            audio: vec![0x03, 0x04],
            end_of_stream: true,
            ..Default::default()
        },
    ]);

    let mut responses = client
        .s2s_stream(requests)
        .await
        .expect("S2sStream must succeed")
        .into_inner();

    let mut received = Vec::new();
    while let Some(message) = responses.next().await {
        received.push(message.expect("every streamed message must decode"));
    }

    assert_eq!(received.len(), 2, "the server sent two chunks");
    assert_eq!(
        received[0].utterance_id, "projects/p/agent/sessions/s/utterances/2",
        "the server has to have seen BOTH inbound messages"
    );
    assert_eq!(received[0].chunk_index, 0);
    assert!(received[1].last_chunk);
}

/// The explicit-presence guarantee of `tests/generated_messages.rs`, but over a real hop: the
/// server sends `Some("")` and the client must not receive `None`.
#[tokio::test]
async fn an_explicit_presence_field_survives_a_real_grpc_hop() {
    let (_service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    let mut responses = client
        .s2s_stream(tokio_stream::iter(vec![csi::S2sStreamRequest {
            session_id: "projects/p/agent/sessions/s".to_string(),
            end_of_stream: true,
            ..Default::default()
        }]))
        .await
        .expect("S2sStream must succeed")
        .into_inner();

    let first = responses
        .next()
        .await
        .expect("the server sends at least one chunk")
        .expect("the chunk must decode");

    let synthesize = match first.response {
        Some(csi::s2s_stream_response::Response::SynthesizeResponse(response)) => response,
        other => panic!("expected a SynthesizeResponse, got {other:?}"),
    };
    assert_eq!(
        synthesize.config.unwrap().instruction,
        Some(String::new()),
        "an explicitly empty presence string must not come back unset"
    );
}

/// A server-streaming RPC has to deliver every message the server sent, in order.
#[tokio::test]
async fn a_server_streaming_call_delivers_every_message() {
    let (_service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    let mut responses = client
        .get_control_stream(csi::ControlStreamRequest {})
        .await
        .expect("GetControlStream must succeed")
        .into_inner();

    let mut epochs = Vec::new();
    while let Some(message) = responses.next().await {
        epochs.push(message.expect("every streamed message must decode").epoch);
    }
    assert_eq!(epochs, vec![1, 2]);
}

/// A server-side `Status` has to reach the caller as that same status, not as a transport error.
#[tokio::test]
async fn a_server_error_reaches_the_client_as_its_status() {
    let (_service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    let error = client
        .get_s2s_pipeline(csi::S2sPipelineId {
            id: MISSING_PIPELINE.to_string(),
        })
        .await
        .expect_err("GetS2sPipeline must report the missing pipeline");

    assert_eq!(error.code(), Code::NotFound);
    assert_eq!(error.message(), "no s2s pipeline named does-not-exist");
}

/// Every RPC the `Conversations` proto declares must exist on the generated client and be
/// routable - a method the generator dropped, or wired to the wrong path, fails here with
/// `Unimplemented`.
#[tokio::test]
async fn every_declared_service_method_exists_and_is_routable() {
    let (_service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    let pipeline = csi::S2sPipeline {
        id: "pipeline-1".to_string(),
        ..Default::default()
    };
    client
        .create_s2s_pipeline(pipeline.clone())
        .await
        .expect("CreateS2sPipeline");
    client
        .get_s2s_pipeline(csi::S2sPipelineId {
            id: "pipeline-1".to_string(),
        })
        .await
        .expect("GetS2sPipeline");
    client
        .update_s2s_pipeline(pipeline)
        .await
        .expect("UpdateS2sPipeline");
    client
        .delete_s2s_pipeline(csi::S2sPipelineId {
            id: "pipeline-1".to_string(),
        })
        .await
        .expect("DeleteS2sPipeline");
    client
        .list_s2s_pipelines(csi::ListS2sPipelinesRequest {})
        .await
        .expect("ListS2sPipelines");
    client
        .s2s_stream(tokio_stream::iter(vec![csi::S2sStreamRequest {
            end_of_stream: true,
            ..Default::default()
        }]))
        .await
        .expect("S2sStream");
    client
        .check_upstream_health(())
        .await
        .expect("CheckUpstreamHealth");
    client
        .get_control_stream(csi::ControlStreamRequest {})
        .await
        .expect("GetControlStream");
    let status = client
        .set_control_status(csi::SetControlStatusRequest {
            control_status: csi::ControlStatus::EmergencyStop as i32,
        })
        .await
        .expect("SetControlStatus")
        .into_inner();
    assert_eq!(
        status.new_control_status,
        csi::ControlStatus::EmergencyStop as i32
    );
}

/// The hand-written [`BearerTokenInterceptor`] has to put its metadata on the wire, where the
/// server can actually read it - asserting on the `Request` it returns would not prove that.
#[tokio::test]
async fn the_bearer_interceptor_reaches_the_server() {
    let (service, addr) = start_server().await;
    let interceptor = BearerTokenInterceptor::new("access-token-abc")
        .expect("a plain ASCII token is valid")
        .with_cai_token("cai-token-xyz")
        .expect("a plain ASCII cai token is valid");
    let mut client = ConversationsClient::with_interceptor(connect(addr).await, interceptor);

    client
        .list_s2s_pipelines(csi::ListS2sPipelinesRequest {})
        .await
        .expect("ListS2sPipelines");

    assert_eq!(
        service.seen(),
        SeenMetadata {
            authorization: Some("Bearer access-token-abc".to_string()),
            cai_token: Some("cai-token-xyz".to_string()),
        }
    );
}

/// Without the interceptor the client must send no credentials at all - the unauthenticated path
/// (plaintext server, or an ingress that injects the bearer token) has to stay usable.
#[tokio::test]
async fn a_client_without_an_interceptor_sends_no_credentials() {
    let (service, addr) = start_server().await;
    let mut client = ConversationsClient::new(connect(addr).await);

    client
        .list_s2s_pipelines(csi::ListS2sPipelinesRequest {})
        .await
        .expect("ListS2sPipelines");

    assert_eq!(service.seen(), SeenMetadata::default());
}

/// A client built against an address nothing listens on must surface a transport error rather
/// than panic or hang - `connect_lazy` defers the connect to the first call.
#[tokio::test]
async fn a_call_to_an_unreachable_target_fails_as_a_status() {
    let channel = Endpoint::from_static("http://127.0.0.1:1")
        .connect_timeout(Duration::from_secs(2))
        .connect_lazy();
    let mut client = ConversationsClient::new(channel);

    let error = client
        .get_s2s_pipeline(csi::S2sPipelineId {
            id: "pipeline-1".to_string(),
        })
        .await
        .expect_err("nothing listens on port 1");

    assert_eq!(error.code(), Code::Unavailable);
}
