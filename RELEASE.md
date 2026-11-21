# Release History

*****************

## Release ONDEWO CSI Rust Client 5.9.2

### New Features

* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) Keycloak authentication with offline-token start-up, the same behaviour as ondewo-csi-client-python 5.9.2: `refresh_token` in `KeycloakConfig` (`refresh_token` appended last, redacted `Debug`, all-or-nothing validation) hands the client a Keycloak *offline* refresh token instead of a password. The client then starts with one `grant_type=refresh_token` exchange and never sends a password grant, not even as a fallback; a refused token is never answered with a password login. Keycloak's brute-force detection counts failed LOGINS, so N processes booting at once with one technical user are a burst the realm penalises, while N refresh grants of one offline token are not.
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) A Keycloak configuration needs a password OR a refresh token, and the Keycloak URL, realm and client id are all-or-nothing; when both credentials are set the token wins. The password and the token are redacted from every rendering of the configuration and take part (hashed) in the shared-provider registry key, so two token-only configurations of different tenants never share a provider. The `token_expiration_in_s` window is armed on both start-up paths.
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) `KeycloakTokenProvider`, a weak SHA-256-keyed shared registry, and `KeycloakInterceptor`, which attaches `authorization: Bearer <token>` to every call; the default transport is one blocking HTTP/1.1 POST over std sockets and rustls (built without its default features).

### Bug Fixes

* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) A failed token refresh never ends the background refresh: any failure of the token endpoint (a 502 from a proxy, a DNS failure, a restarting Keycloak, a dead offline session) is retried with one full-jitter backoff (5 s doubling up to 300 s, measured from the moment the failure is recorded) that the lazy path shares, and a success resets it. Concurrent callers during an outage share one in-flight refresh, so an outage costs one token request, not one per call.
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) A token that is still valid keeps being served when its refresh fails inside the 30 s expiry leeway; only an EXPIRED token raises `KeycloakAuthenticationError` (with its `source`), whatever failed underneath: a transport error, a non-2xx status, a body that is not JSON or not a JSON object, a missing `access_token` or a non-numeric `expires_in`, each with the original error as its cause. A malformed response changes no provider state, and a password login whose response carries no `refresh_token` keeps the previous one, as the python SDK does.

### Improvements

* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) One Keycloak contract across every ONDEWO client SDK, with the python SDK as the reference: the same constants, log lines and error texts. The token request is bounded by 10 s, quoted Keycloak error bodies are cut at 500 characters, a token response is reported by its field names only (never its values), and each failure logs one WARNING with the attempt number and the retry delay, the first success afterwards one INFO; no credential appears in either line.
* [[OND236-120]](https://ondewo.atlassian.net/browse/OND236-120) Tests implement the cross-SDK test catalogue T01-T62 plus edge cases, on a fake token endpoint and clock, with 100 % coverage of the hand-written authentication code.
* Tracking API Version [5.9.0](https://github.com/ondewo/ondewo-csi-api/releases/tag/5.9.0) ( [Documentation](https://ondewo.github.io/ondewo-csi-api/) ); the generated code is unchanged against 5.9.0.

*****************

## Release ONDEWO CSI Rust Client 5.9.0

### Improvements

* Tracking API Version [5.9.0](https://github.com/ondewo/ondewo-csi-api/releases/tag/5.9.0) ( [Documentation](https://ondewo.github.io/ondewo-csi-api/) )
* Generated with ondewo-proto-compiler 5.17.0 (protoc 35.1); new runtime floors: prost / prost-types 0.14.6, tonic / tonic-prost 0.14.9 (protoc 35.1).

*****************

## Release ONDEWO CSI Rust Client 5.7.0

### Improvements

* Tracking API Version [5.7.0](https://github.com/ondewo/ondewo-csi-api/releases/tag/5.7.0) ( [Documentation](https://ondewo.github.io/ondewo-csi-api/) )

*****************

## Release ONDEWO CSI Rust Client 5.6.2

### Improvements

* Tracking API Version [5.6.2](https://github.com/ondewo/ondewo-csi-api/releases/tag/5.6.2) ( [Documentation](https://ondewo.github.io/ondewo-csi-api/) )

*****************

## Release ONDEWO CSI Rust Client 5.6.0

### New Features

* Tracking API Version [5.6.0](https://github.com/ondewo/ondewo-csi-api/releases/tag/5.6.0)
  (5.5.0 before). The regenerated stubs under `src/api` expose the new API surface:
  * `Conversations.SetCallMediaControl`: per-call operator media control pushed by ondewo-sip.
    The request `CallMediaControlLevel` carries the full effective level (`bot_muted`,
    `listening_paused`), a monotonic `generation` and a `reason` token; the response
    `SetCallMediaControlResponse` reports the `applied` level, `changed`, `stale`,
    `bot_playback_in_flight` and `refusal_reason`.
  * `ControlStreamResponse.media_control`: set only on media-control messages of
    `GetControlStream`. A client must handle such a message as media control and must not apply
    its echoed `control_status`, which would un-latch a pending `BARGE_IN`.
  * `SipTrigger.INVITE` is documented as not implemented; use the ondewo-vtsi
    `Calls.InviteToCall` RPC instead.
* The change is additive: no field, enum value or RPC was renumbered or removed. Rust code that
  builds `ControlStreamResponse` with a struct literal needs `media_control` (or
  `..Default::default()`).

### Tests

* `tests/generated_grpc.rs` serves and calls `SetCallMediaControl` (applied and stale
  generations) and checks that the level reaches the control stream as `media_control` on the
  media-control message only.
* `tests/generated_messages.rs` round-trips a `CallMediaControlLevel` inside a
  `ControlStreamResponse`, incl. a `u64::MAX` generation.

### Build

* `ondewo-proto-compiler` is pinned to 5.15.5 (5.15.4 before). The vendored nlu, s2t and t2s
  APIs of ondewo-csi-api are unchanged (7.1.0, 7.5.0, 6.6.0).

*****************

## Release ONDEWO CSI Rust Client 5.5.1

### New Features

* TLS, mutual TLS and plaintext channels: the new hand-written module `channel` builds the tonic
  `Endpoint` / `Channel` from a `ClientConfig` (`endpoint()`, `connect()`, `connect_lazy()`).
  `grpc_cert`, `grpc_client_cert` and `grpc_client_key` take PEM **content**, not file paths.
  Without `grpc_cert` the platform trust store is used (tonic `tls-native-roots`).
* Refused with a `ChannelError` before tonic sees the config: half a client identity
  (`IncompleteClientIdentity`), a client identity on a plaintext channel
  (`ClientIdentityWithoutTls`) and a certificate field that holds no PEM certificate
  (`NotAPemCertificate`).
* A plaintext channel logs a `tracing` warning naming `host:port`. `Debug` redacts the client key
  and renders no PEM, and error messages name only the field and `host:port`. A bare IPv6 host is
  bracketed, and `tls_domain_name` overrides the name the server certificate is checked against.
* README section "TLS, mutual TLS and certificates": the modes, a test PKI built with openssl, TLS
  security notes and the channel defaults. Of the python SDK channel options, the message size
  limit is applied per client (`MAX_MESSAGE_LENGTH`); keepalive pings are not set (TCP keepalive
  detects a dropped idle connection instead), and tonic has no reconnect backoff or retry policy.

### Tests

* `tests/tls_channel.rs` runs real handshakes against an in-process tonic server with an
  openssl-generated test PKI: plain TLS, mutual TLS, missing and foreign client identities, wrong
  CA, platform roots, CRLF PEMs, SAN override and IPv6 loopback.
* `tests/release_notes.rs` pins the RELEASE.md slice the GitHub release body is cut from.

### Build

* `ondewo-proto-compiler` is pinned to 5.15.4 (5.15.2 before this release), which pre-warms tonic
  `tls-native-roots` for the offline stub generation that `src/channel.rs` needs.
  `make generate_ondewo_protos` now also stages `README.md`, which the image's `cargo package`
  requires. The regenerated stubs under `src/api` are byte-identical.
* Tracking API Version [5.5.0](https://github.com/ondewo/ondewo-csi-api/releases/tag/5.5.0) (unchanged)

*****************

## Release ONDEWO CSI Rust Client 5.5.0

### New Features

* Initial release of the ONDEWO CSI (Conversational Speech Interface) gRPC client for rust. The crate
  ships the complete client surface of the [ondewo-csi-api](https://github.com/ondewo/ondewo-csi-api)
  protocol buffer definitions: [prost](https://crates.io/crates/prost) message types and
  [tonic](https://crates.io/crates/tonic) service clients for every RPC, generated by the
  `ondewo-rust-proto-compiler` image of
  [ondewo-proto-compiler](https://github.com/ondewo/ondewo-proto-compiler) 5.15.1.
* The generated module tree is confined to `src/api` and is re-exported from the crate root, so a
  message declared in `package ondewo.csi;` is reachable as
  `ondewo_csi_client::ondewo::csi::<Message>` and each service as
  `ondewo::csi::<service>_client::<Service>Client`. Hand-written modules live beside it
  under `src/` and are declared in the hand-written crate barrel `src/lib.rs`, which the
  generator leaves untouched.
* The generated stubs under `src/api` are COMMITTED, so the crate builds straight from a checkout -
  no docker, no compiler image and no submodule needed. CI compiles and tests exactly those files.
* A test suite under `tests/` exercises the generated stubs rather than only the hand-written code:
  prost messages are serialized and re-parsed field by field, maps and enum discriminants are
  pinned, and the generated `ConversationsServer` - all nine RPCs of `ondewo.csi.Conversations`,
  including the bidirectional `S2sStream` and the server-streaming `GetControlStream` - is served
  over a loopback socket and driven by the generated `ConversationsClient`, so every declared RPC
  really is encoded, routed by its `/ondewo.csi.Conversations/<Method>` path, answered and decoded
  again. `ondewo.csi` declares no proto3 `optional` scalar of its own, but the `ondewo.nlu` and
  `ondewo.t2s` packages it vendors do and they are part of this crate's surface, so explicit
  presence is asserted against those - both on the wire and across a real gRPC hop.
* A `BearerTokenInterceptor` (`src/auth.rs`) attaches the Keycloak `authorization` and `cai-token`
  metadata to every request; its `Debug` output redacts both credentials. `make coverage` gates the
  hand-written sources at 100% line coverage and the same gate runs in CI.
* `make build` runs the whole pipeline - submodule checkout, compiler image build, stub
  generation and `cargo build` - and `make check_build` asserts that a generated stub exists for
  every proto package before a release is cut.

*****************
