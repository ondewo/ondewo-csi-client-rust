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

//! Wire-level tests for the GENERATED prost messages under `src/api`.
//!
//! These are the cases that catch a broken generator: a dropped field, a shifted tag number, a
//! presence field silently coerced to its zero value, an enum whose discriminants moved. They are
//! pure encode/decode - no runtime, no socket. The gRPC plumbing is covered by
//! `tests/generated_grpc.rs`.
//!
//! CSI is a composition API: `ondewo.csi` itself is small, and most of the surface a caller
//! touches comes from the `ondewo.nlu`, `ondewo.s2t` and `ondewo.t2s` packages it vendors. The
//! cases below therefore reach across those package boundaries on purpose.

use std::collections::HashMap;

use ondewo_csi_client::api::google;
use ondewo_csi_client::api::ondewo::{csi, nlu, s2t, t2s};
use prost::Message;
use prost_types::Timestamp;

/// A fully populated [`csi::ControlMessageServiceParameters`] - scalar, repeated bytes, a nested
/// message from another package (which itself carries a map and a presence field) and a oneof at
/// once.
fn sample_parameters() -> csi::ControlMessageServiceParameters {
    let mut parameters = HashMap::new();
    parameters.insert(
        "city".to_string(),
        nlu::context::Parameter {
            name: "city".to_string(),
            display_name: "City".to_string(),
            value: "Vienna".to_string(),
            value_original: "vienna".to_string(),
            created_at: Some(Timestamp {
                seconds: 1_700_000_000,
                nanos: 0,
            }),
            ..Default::default()
        },
    );

    csi::ControlMessageServiceParameters {
        transfer_id: "transfer-1".to_string(),
        wav_files: vec![vec![0x52, 0x49, 0x46, 0x46], vec![0x57, 0x41, 0x56, 0x45]],
        text: "please hold".to_string(),
        context: Some(nlu::Context {
            name: "greeting-context".to_string(),
            lifespan_count: 5,
            parameters,
            lifespan_time: Some(42.5),
            ..Default::default()
        }),
        session_id: "projects/p/agent/sessions/s".to_string(),
        context_name: "greeting-context".to_string(),
        condition_start: Some(csi::Condition {
            r#type: csi::ConditionType::Immediate as i32,
            value: "now".to_string(),
        }),
        condition_end: Some(csi::Condition {
            r#type: csi::ConditionType::Duration as i32,
            value: "30s".to_string(),
        }),
        config: Some(csi::control_message_service_parameters::Config::T2sConfig(
            t2s::RequestConfig {
                t2s_pipeline_id: "t2s-pipeline-1".to_string(),
                ..Default::default()
            },
        )),
    }
}

#[test]
fn the_control_message_parameters_survive_a_serialize_parse_round_trip() {
    let original = sample_parameters();

    let bytes = original.encode_to_vec();
    assert!(
        !bytes.is_empty(),
        "a populated ControlMessageServiceParameters must not encode to zero bytes"
    );
    assert_eq!(
        bytes.len(),
        original.encoded_len(),
        "encoded_len must agree with the bytes actually written"
    );

    let parsed = csi::ControlMessageServiceParameters::decode(bytes.as_slice())
        .expect("re-parsing our own bytes must work");
    assert_eq!(parsed, original);

    // Spot-check the individual fields too: a PartialEq on two identically broken values would
    // still pass above.
    assert_eq!(parsed.transfer_id, "transfer-1");
    assert_eq!(parsed.wav_files.len(), 2);
    assert_eq!(parsed.wav_files[1], vec![0x57, 0x41, 0x56, 0x45]);
    let context = parsed.context.unwrap();
    assert_eq!(context.lifespan_count, 5);
    assert_eq!(context.lifespan_time, Some(42.5));
    assert_eq!(context.parameters["city"].value, "Vienna");
    assert_eq!(
        parsed.condition_end.unwrap().r#type,
        csi::ConditionType::Duration as i32
    );
}

#[test]
fn a_default_message_round_trips_to_zero_bytes() {
    let empty = csi::ControlMessageServiceParameters::default();

    assert_eq!(empty.transfer_id, "");
    assert!(empty.wav_files.is_empty());
    assert_eq!(empty.context, None);
    assert_eq!(empty.config, None);

    let bytes = empty.encode_to_vec();
    assert!(
        bytes.is_empty(),
        "proto3 must not put unset fields on the wire, got {bytes:?}"
    );
    assert_eq!(
        csi::ControlMessageServiceParameters::decode(bytes.as_slice()).unwrap(),
        empty
    );
}

/// `ondewo.csi` declares no proto3 `optional` scalar of its own, but the packages it vendors do -
/// `ondewo.nlu` has 75 of them and `ondewo.t2s` one - and they are part of THIS crate's surface,
/// reachable through `ControlMessageServiceParameters`. An unset field and a field explicitly set
/// to its zero value are two DIFFERENT values and must stay distinguishable across the wire; a
/// generator that collapses them makes `0.0` unsendable.
#[test]
fn an_explicit_presence_field_distinguishes_unset_from_zero() {
    let unset = nlu::Context {
        name: "ctx".to_string(),
        lifespan_time: None,
        ..Default::default()
    };
    let explicit_zero = nlu::Context {
        name: "ctx".to_string(),
        lifespan_time: Some(0.0),
        ..Default::default()
    };

    let unset_bytes = unset.encode_to_vec();
    let zero_bytes = explicit_zero.encode_to_vec();
    assert_ne!(
        unset_bytes, zero_bytes,
        "an explicitly set 0.0 must occupy the wire, an unset field must not"
    );

    assert_eq!(
        nlu::Context::decode(unset_bytes.as_slice())
            .unwrap()
            .lifespan_time,
        None
    );
    assert_eq!(
        nlu::Context::decode(zero_bytes.as_slice())
            .unwrap()
            .lifespan_time,
        Some(0.0)
    );

    // The same for a presence STRING, where the zero value is the empty string - `t2s::RequestConfig`
    // is what a csi pipeline hands to the text-to-speech service.
    let unset = t2s::RequestConfig {
        t2s_pipeline_id: "t2s-pipeline-1".to_string(),
        instruction: None,
        ..Default::default()
    };
    let explicit_empty = t2s::RequestConfig {
        instruction: Some(String::new()),
        ..unset.clone()
    };
    assert_ne!(unset.encode_to_vec(), explicit_empty.encode_to_vec());
    assert_eq!(
        t2s::RequestConfig::decode(explicit_empty.encode_to_vec().as_slice())
            .unwrap()
            .instruction,
        Some(String::new()),
        "an explicitly empty string must not come back unset"
    );
}

/// Decoding tolerates fields it does not know: an unknown tag is skipped, not an error.
#[test]
fn decoding_skips_an_unknown_field() {
    let mut bytes = csi::S2sPipelineId {
        id: "pipeline-1".to_string(),
    }
    .encode_to_vec();
    // tag 999, wire type 0 (varint), value 1
    bytes.extend_from_slice(&[0xB8, 0x3E, 0x01]);

    let parsed = csi::S2sPipelineId::decode(bytes.as_slice())
        .expect("an unknown field must be skipped, not rejected");
    assert_eq!(parsed.id, "pipeline-1");
}

#[test]
fn decoding_rejects_a_truncated_message() {
    let bytes = sample_parameters().encode_to_vec();
    let truncated = &bytes[..bytes.len() - 1];

    assert!(
        csi::ControlMessageServiceParameters::decode(truncated).is_err(),
        "a truncated message must not decode silently"
    );
}

/// The zero value of an enum is the one a default-constructed message carries, so it must be the
/// variant the proto declares as `= 0`.
#[test]
fn the_enum_zero_value_is_the_unspecified_variant() {
    assert_eq!(csi::sip_trigger::SipTriggerType::Unspecified as i32, 0);
    assert_eq!(
        csi::sip_trigger::SipTriggerType::try_from(0),
        Ok(csi::sip_trigger::SipTriggerType::Unspecified)
    );
    assert_eq!(
        csi::SipTrigger::default().r#type,
        csi::sip_trigger::SipTriggerType::Unspecified as i32,
        "a default message must carry the enum's zero value"
    );

    assert_eq!(
        csi::sip_trigger::SipTriggerType::Unspecified.as_str_name(),
        "UNSPECIFIED"
    );
    assert_eq!(
        csi::sip_trigger::SipTriggerType::from_str_name("UNSPECIFIED"),
        Some(csi::sip_trigger::SipTriggerType::Unspecified)
    );
    assert_eq!(
        csi::sip_trigger::SipTriggerType::from_str_name("NOT_A_VARIANT"),
        None
    );
    assert!(
        csi::sip_trigger::SipTriggerType::try_from(9_999).is_err(),
        "an out-of-range discriminant must not map to a variant"
    );

    // `ControlStatus` is the other shape this API uses: its zero variant is OK, not an
    // UNSPECIFIED, and a default message has to carry exactly that.
    assert_eq!(csi::ControlStatus::Ok as i32, 0);
    assert_eq!(csi::ControlStreamResponse::default().control_status, 0);
}

/// A non-zero enum value has to travel as its discriminant, not as the zero value.
#[test]
fn a_non_zero_enum_value_round_trips() {
    let message = csi::ControlMessage {
        service: csi::ControlMessageServiceName::OndewoT2s as i32,
        method: csi::ControlMessageServiceMethod::PlayText as i32,
        parameters: Some(sample_parameters()),
    };

    let parsed = csi::ControlMessage::decode(message.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, message);
    assert_eq!(
        csi::ControlMessageServiceName::try_from(parsed.service),
        Ok(csi::ControlMessageServiceName::OndewoT2s)
    );
    assert_eq!(
        csi::ControlMessageServiceMethod::try_from(parsed.method),
        Ok(csi::ControlMessageServiceMethod::PlayText)
    );
}

/// Repeated and nested message fields have to nest, not flatten.
#[test]
fn a_nested_and_repeated_message_round_trips() {
    let response = csi::ListS2sPipelinesResponse {
        pipelines: vec![
            csi::S2sPipeline {
                id: "pipeline-1".to_string(),
                s2t_pipeline_id: "s2t-1".to_string(),
                nlu_project_id: "project-1".to_string(),
                nlu_language_code: "de".to_string(),
                t2s_pipeline_id: "t2s-1".to_string(),
            },
            csi::S2sPipeline {
                id: "second".to_string(),
                ..Default::default()
            },
        ],
    };

    let parsed =
        csi::ListS2sPipelinesResponse::decode(response.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, response);
    assert_eq!(parsed.pipelines.len(), 2);
    assert_eq!(parsed.pipelines[1].id, "second");
}

/// The generator emits one module per proto PACKAGE. CSI vendors six packages besides its own -
/// `ondewo.{nlu,s2t,t2s}` and `google.{api,rpc,type}` - and messages of every one of them have to
/// be reachable and usable, including the csi message that stitches three of them together.
#[test]
fn messages_of_every_generated_package_are_reachable() {
    // ondewo.csi + ondewo.nlu + ondewo.t2s in one message: the S2S stream response is a oneof over
    // the responses of the services a pipeline chains.
    let response = csi::S2sStreamResponse {
        utterance_id: "utterance-1".to_string(),
        chunk_index: 2,
        last_chunk: true,
        turn_epoch: 7,
        response: Some(csi::s2s_stream_response::Response::SynthesizeResponse(
            t2s::SynthesizeResponse {
                audio_uuid: "audio-1".to_string(),
                text: "hello".to_string(),
                sample_rate: 16_000.0,
                ..Default::default()
            },
        )),
    };
    let parsed = csi::S2sStreamResponse::decode(response.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, response);

    let detected = csi::S2sStreamResponse {
        response: Some(csi::s2s_stream_response::Response::DetectIntentResponse(
            nlu::DetectIntentResponse {
                response_id: "response-1".to_string(),
                ..Default::default()
            },
        )),
        ..Default::default()
    };
    assert_eq!(
        csi::S2sStreamResponse::decode(detected.encode_to_vec().as_slice()).unwrap(),
        detected
    );

    // ondewo.s2t
    let s2t_config = s2t::TranscribeRequestConfig {
        s2t_pipeline_id: "s2t-pipeline-1".to_string(),
        language: Some("de".to_string()),
        ..Default::default()
    };
    assert_eq!(
        s2t::TranscribeRequestConfig::decode(s2t_config.encode_to_vec().as_slice()).unwrap(),
        s2t_config
    );

    // google.rpc - `CheckUpstreamHealthResponse` reports one `Status` per upstream service.
    let health = csi::CheckUpstreamHealthResponse {
        s2t_status: Some(google::rpc::Status {
            code: 0,
            message: "ok".to_string(),
            details: vec![],
        }),
        nlu_status: Some(google::rpc::Status {
            code: 5,
            message: "not found".to_string(),
            details: vec![],
        }),
        t2s_status: None,
    };
    let parsed =
        csi::CheckUpstreamHealthResponse::decode(health.encode_to_vec().as_slice()).unwrap();
    assert_eq!(parsed, health);
    assert_eq!(parsed.nlu_status.unwrap().message, "not found");

    // google.type
    let coordinates = google::r#type::LatLng {
        latitude: 48.208_2,
        longitude: 16.373_8,
    };
    assert_eq!(
        google::r#type::LatLng::decode(coordinates.encode_to_vec().as_slice()).unwrap(),
        coordinates
    );

    // google.api
    let pattern = google::api::CustomHttpPattern {
        kind: "GET".to_string(),
        path: "/v1/s2s_pipelines".to_string(),
    };
    assert_eq!(
        google::api::CustomHttpPattern::decode(pattern.encode_to_vec().as_slice()).unwrap(),
        pattern
    );
}
