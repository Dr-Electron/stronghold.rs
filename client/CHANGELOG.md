# Changelog

## \[0.5.0]

- - replace actor system riker with actix

- introduced registry actor for clients as service

- introduced snapshot actor as service

- merge `Internal` and `Client`-Actors into `SecureClient`

- api change in interface for test reading secrets out of a vault. minimal impact.

- [e8b10eac](https://www.github.com/iotaledger/stronghold.rs/commit/e8b10eac4a914e5d78aae40ab4f1da15ac136ac7) feat: Migrating stronghold from riker actor system implementation to actix. client + internal actor have been merged. Message types are transformed into structs. on 2021-08-23

- \[[PR 270](https://github.com/iotaledger/stronghold.rs/pull/270)]

- Move management of network-Actor and client-target into Registry

- Make client-target optional, in case that it is killed before switching to another target

- Make registry a normal actor instead of a system-service

- [a4cb0152](https://www.github.com/iotaledger/stronghold.rs/commit/a4cb0152f79c643afbd4eea72318c3ce300a0c27) doc(client): Add changelog on 2021-10-26

- - \[[PR 297](https://github.com/iotaledger/stronghold.rs/pull/297)] Add `CopyRecord` procedure.
  - [ae6f566c](https://www.github.com/iotaledger/stronghold.rs/commit/ae6f566c97ca1bc2de4b710ac8aa86768159502b) feat(client): add procedure for copying a record on 2021-11-29

- - make stronghold interface clonable
  - [681a024e](https://www.github.com/iotaledger/stronghold.rs/commit/681a024e7fd5d6095bbf571d5a3d22fb449b54da) Clonable Stronghold Instance ([#257](https://www.github.com/iotaledger/stronghold.rs/pull/257)) on 2021-09-13

- Update inline Docs and README files to reflect the current state of the project.
  - [fc95c271](https://www.github.com/iotaledger/stronghold.rs/commit/fc95c27128dedf8aa2d366776c22cb9c8e3f158a) add changes. on 2021-07-01
  - [eafca12a](https://www.github.com/iotaledger/stronghold.rs/commit/eafca12ad915166d8039df6ad050bb1c65cbe248) fix changes format. on 2021-07-01

- - Add `actors::secure::StoreError::NotExisting` as proper error type for correct error handling in client.
  - [ad57181e](https://www.github.com/iotaledger/stronghold.rs/commit/ad57181e7c5baa4b6ccb66fb464667c97967db08) fix: inconsistent error message. ([#251](https://www.github.com/iotaledger/stronghold.rs/pull/251)) on 2021-08-26

- \[[PR 254](https://github.com/iotaledger/stronghold.rs/pull/254)]\
  Change key handling in the `SecureClient` to avoid unnecessary cloning of keys.
  Remove obsolete VaultId-HashSet from the `SecureClient`.
  - [9b8d0da1](https://www.github.com/iotaledger/stronghold.rs/commit/9b8d0da150afd7446198672c8f7675547031c060) Fix(client): Avoid Key cloning, remove redundant code ([#254](https://www.github.com/iotaledger/stronghold.rs/pull/254)) on 2021-09-09

- \[[PR 276](https://github.com/iotaledger/stronghold.rs/pull/276)]

- Remove `relay` and `mdns` features.

- In the `StrongholdP2p` Interface enable / disable mdns and relay functionality on init via config flags in the `StrongholdP2pBuilder`.
  Per default, both are enabled.

- In the `Stronghold` client interface enable / disable mdns and relay in the `NetworkConfig` when spawning a new p2p-network actor.
  Per default, both are disabled.

- [8cbb8944](https://www.github.com/iotaledger/stronghold.rs/commit/8cbb8944bd4ef94ec331b97a8a9cbc7122172f8e) Add changelog on 2021-10-29

- [679cf029](https://www.github.com/iotaledger/stronghold.rs/commit/679cf02943460edf4560445f0b563f9cd0f0c9e8) feat(client):  mdns/relay config in Stronghold client on 2021-11-01

- Patch crypto.rs version v0.7 -> v0.8.
  - [47b6364b](https://www.github.com/iotaledger/stronghold.rs/commit/47b6364bbd256f71cc7eb7cf4a731db19d39dab6) chore(\*): patch iota-crypto v0.7.0 -> v0.8.0 on 2021-11-12

- \[[PR 290](https://github.com/iotaledger/stronghold.rs/pull/290)]

- Persist the state of stronghold-p2p in the `SecureClient` by serializing the `NetworkConfig` and writing it to the store.

- Allow loading stored states into the `NetworkActor` on init.

- Allow reuse of same `Keypair` that is stored in the vault.

- [83903c7e](https://www.github.com/iotaledger/stronghold.rs/commit/83903c7e69803a7dea54f2140d58a271796e6cc9) Add changelog for p2p/persist-config on 2021-11-16

- \[[PR 285](https://github.com/iotaledger/stronghold.rs/pull/285)]
  Implement messages to write the keypair used for `StrongholdP2p` in the vault and derive the
  `PeerId` and a new noise `AuthenticKeypair` from it.
  - [70b29c10](https://www.github.com/iotaledger/stronghold.rs/commit/70b29c1086db0491f4c8b14d8db49eadb6d6cfa8) doc(client): document p2p_messages, add changelog on 2021-11-12

- \[[PR 258](https://github.com/iotaledger/stronghold.rs/pull/258)]
  Implement API for the Stronghold Procedures, see PR 258 for details.
  - [47b6364b](https://www.github.com/iotaledger/stronghold.rs/commit/47b6364bbd256f71cc7eb7cf4a731db19d39dab6) chore(\*): patch iota-crypto v0.7.0 -> v0.8.0 on 2021-11-12

- \[[PR 269](https://github.com/iotaledger/stronghold.rs/pull/269)]
  Refactor Error types in engine and client:

- Add differentiated error types for the different methods

- Avoid unwraps in the engine

- Remove the single, crate-wide used error types of engine and client

- Remove `anyhow` Error types in client and bubble up the actual error instead

- Add nested Result type alias `StrongholdResult<T> = Result<T, ActorError>` for interface errors

- [d6b814dd](https://www.github.com/iotaledger/stronghold.rs/commit/d6b814dd7729dbbf39b73e050767992aadc19377) Add Changelog for PR 269 on 2021-11-03

- - corrects wrong control flow. `write_to_vault` always returned an error even if the operation was successful.
  - [aea8a9dc](https://www.github.com/iotaledger/stronghold.rs/commit/aea8a9dc8c3fa12e5444c5b4bb3303876e4c1a2f) Fix/wrong cf on write to vault ([#253](https://www.github.com/iotaledger/stronghold.rs/pull/253)) on 2021-08-30

## \[0.4.1]

- Merged Store, Vault and Snapshot into a single crate called Stronghold-Engine.
  Merged utils-derive and communication-macros into a new crate called stronghold-derive
  Export Stronghold-derive through Stronghold-utils.
  - Bumped due to a bump in stronghold-engine.
  - [36c8983](https://www.github.com/iotaledger/stronghold.rs/commit/36c8983eefd594c702a9e8b32bad25354ad127c0) merge derive/macro crates. on 2021-04-21
  - [b7d44f5](https://www.github.com/iotaledger/stronghold.rs/commit/b7d44f530e08be27128f25f46b4bb05cf3da99bd) update config. on 2021-04-21

## \[0.4.0]

- Updated cargo.toml files with the updated crypto.rs revisions and authors.
  Fixed logic in snapshot and providers to use the `try_*` encryption and decryption functions.
  Fixed commandline and stopped it from overwriting snapshots.
  - [64e08fe](https://www.github.com/iotaledger/stronghold.rs/commit/64e08fe39454d2191561783d009b155c91db37c1) add .changes. on 2021-03-19
  - [0758b67](https://www.github.com/iotaledger/stronghold.rs/commit/0758b6734a1e22d491345a6b894acea12ab5b1b7) add .changes. on 2021-03-19
- Remove old logic from client and vault.
  Added clear cache logic and message to stop the client actor.
  Removed Client Derive Data hashmap.
  - [81892aa](https://www.github.com/iotaledger/stronghold.rs/commit/81892aa704b920c50de2517e8073943d8bf0c2b9) add md file. on 2021-04-20
- Add libp2p-relay protocol to stronghold-communication.
  Add methods for using a relay peer to the iota-stronghold interface.
  - [a414582](https://www.github.com/iotaledger/stronghold.rs/commit/a414582024f45e854a75ab82e4196777ab4a42b8) Feat/comms relay ([#183](https://www.github.com/iotaledger/stronghold.rs/pull/183)) on 2021-04-13
- move stronghold-utils and add utils-derive for proc macros.
  rebuild vault and remove versioning.
  update client to use new vault.
  - [5490f0a](https://www.github.com/iotaledger/stronghold.rs/commit/5490f0aaaf58e5322a5569c02669514ec067b02f) refactor vault ([#181](https://www.github.com/iotaledger/stronghold.rs/pull/181)) on 2021-04-15

## \[0.3.0]

- Rename the previously incorrectly named combined SLIP10+Ed25519 procedures (now
  named with a `SLIP10DeriveAnd` prefix) and add back the Ed25519 ("only")
  procedures.
  - [e221dcb](https://www.github.com/iotaledger/stronghold.rs/commit/e221dcb31519960e60982012da3c2ac154d989e1) Add back the atomic Ed25519 procedures ([#122](https://www.github.com/iotaledger/stronghold.rs/pull/122)) on 2021-01-08
  - [8e255bf](https://www.github.com/iotaledger/stronghold.rs/commit/8e255bf4aad8caf69dcddfac24d4cdb07f716177) fix(covector): wrong version bump type ([#128](https://www.github.com/iotaledger/stronghold.rs/pull/128)) on 2021-01-14
- Add documentation and cleanup the code for these crates and modules in preparation for beta.
  - [dae0457](https://www.github.com/iotaledger/stronghold.rs/commit/dae04579cb20ad69a7aecdf102fb66ecac4aaf46) Beta Cleanup ([#166](https://www.github.com/iotaledger/stronghold.rs/pull/166)) on 2021-03-19
- Change the communication firewall configuration, add new methods for it to the client interface.
  Cleanup the stronghold-communication code, add documentation and examples.
  - [b9d006c](https://www.github.com/iotaledger/stronghold.rs/commit/b9d006cef88f6ae45f47a8644702a800d13e39c5) Feat/communication cleanup ([#167](https://www.github.com/iotaledger/stronghold.rs/pull/167)) on 2021-03-18
- Implement a configurable firewall in the communication actor, add a macro to derive permissions for requests.
  - [025685f](https://www.github.com/iotaledger/stronghold.rs/commit/025685fb181ba0600f31680a3f4c115c0e2097f7) Feat/communication firewall ([#158](https://www.github.com/iotaledger/stronghold.rs/pull/158)) on 2021-03-11
- Refactor the communication actor, enable using a relay peer, and integrate communication as feature into the stronghold interface.
  Remove unecessary Option/ Result wraps in `random` and `iota-stronghold`.
  Rename stronghold-test-utils to stronghold-utils and added riker ask pattern to it.
  - [9c7cba6](https://www.github.com/iotaledger/stronghold.rs/commit/9c7cba624e2a99f04a2d033b8673f8a4b8735f0b) Feat/integrate comms ([#130](https://www.github.com/iotaledger/stronghold.rs/pull/130)) on 2021-02-26
  - [fcb62bb](https://www.github.com/iotaledger/stronghold.rs/commit/fcb62bbf966bfcd543b13a79d73839a3fee0219e) fix/covector-2 ([#163](https://www.github.com/iotaledger/stronghold.rs/pull/163)) on 2021-03-12
- Remove Crypto, Random and Primitives libraries in favor of Crypto.rs
  Moved Runtime into the engine.
  Add new guarded types for Runtime and remove old logic.

## Features:

- Causes segfault upon access without borrow
- Protects using mprotect
- Adds guard pages proceeding and following the allocated memory.
- Adds a canary pointer to detect underflows.
- Locks memory with mlock.
- Frees memory using munlock
- Memory is zeroed when no longer in use through sodium_free
- Can be compared in constant time
- Can not be printed using debug
- Can not be cloned using the Clone trait.

Implement guarded types in Vault to protect the data and the keys.
Clean up logic inside of the Client library.

- [dd65b67](https://www.github.com/iotaledger/stronghold.rs/commit/dd65b67f42718150c7c7dbab9606ee2167cf11ce) add changes. on 2021-03-11
- [829ecac](https://www.github.com/iotaledger/stronghold.rs/commit/829ecac2e8090d478706c673cd45f1b91a60b2de) fix(covector) ([#164](https://www.github.com/iotaledger/stronghold.rs/pull/164)) on 2021-03-12
- Add `Clone`/`Debug` implementations for `StrongholdFlags` and `VaultFlags`.
  - [e9fda48](https://www.github.com/iotaledger/stronghold.rs/commit/e9fda4859d0367f3a69265dcb5d4d276bfb07066) Add Clone/Debug implementations for StrongholdFlags/VaultFlags ([#157](https://www.github.com/iotaledger/stronghold.rs/pull/157)) on 2021-03-12

## \[0.2.0]

- Added the initial client logic and integrated it with the Riker actor model. Change includes a Client/Cache actor, a Bucket actor, a Snapshot actor, and a keystore actor.  All of the Stronghold APIs are available.
  - [7c7320a](https://www.github.com/iotaledger/stronghold.rs/commit/7c7320ab0bc71749510a590f418c9bd70329dc02) add client changelog. on 2020-11-30
  - [4986685](https://www.github.com/iotaledger/stronghold.rs/commit/49866854f32dde8589f37c6d9ea0c2e7ddb3c461) remove todos and update readme. on 2020-11-30
  - [7f1e9ed](https://www.github.com/iotaledger/stronghold.rs/commit/7f1e9edf5f5c5e148376575057a55d1d1398708a) Chore/covector fix ([#61](https://www.github.com/iotaledger/stronghold.rs/pull/61)) on 2020-12-01
  - [f882754](https://www.github.com/iotaledger/stronghold.rs/commit/f88275451e7d3c140bbfd1c90a9267aa222fb6d0) fix(client): readme and changelog ([#64](https://www.github.com/iotaledger/stronghold.rs/pull/64)) on 2020-12-01
- Create SignUnlockBlock procedure.
  - [f9d180a](https://www.github.com/iotaledger/stronghold.rs/commit/f9d180a85fe57c2942d6ebabfcfdb3c445b0ba5b) feat(client): introduce SignUnlockBlock proc ([#92](https://www.github.com/iotaledger/stronghold.rs/pull/92)) on 2020-12-21
- Alpha release of Stronghold: "Saint-Malo"
  - [4b6f4af](https://www.github.com/iotaledger/stronghold.rs/commit/4b6f4af29f6c21044f5063ec4a8d8aff643f81a7) chore(release) ([#105](https://www.github.com/iotaledger/stronghold.rs/pull/105)) on 2020-12-24
  - [06c6d51](https://www.github.com/iotaledger/stronghold.rs/commit/06c6d513dfcd1ba8ed6379177790ec6db28a6fea) fix(changelog): Alpha Release ([#106](https://www.github.com/iotaledger/stronghold.rs/pull/106)) on 2020-12-24
- Introduce release manager for rust crates including tangle registry.
  - [c10811e](https://www.github.com/iotaledger/stronghold.rs/commit/c10811effbff396370762e76a2f2d44221dc7327) feat(covector): rigging ([#57](https://www.github.com/iotaledger/stronghold.rs/pull/57)) on 2020-11-29
- Add a hierarchical wallet implementation following SLIP10 for the Ed25519 curve.
  - [dd12c16](https://www.github.com/iotaledger/stronghold.rs/commit/dd12c16d628ec996728d356cfb815f185cc5cc37) Add changelog message on 2020-12-02
  - [d3c63be](https://www.github.com/iotaledger/stronghold.rs/commit/d3c63bec8052c0cd6a636fef3463b90893b55d4b) fix(covector) ([#82](https://www.github.com/iotaledger/stronghold.rs/pull/82)) on 2020-12-17
