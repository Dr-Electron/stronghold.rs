// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Main Stronghold Interface
//!
//! All functionality can be accessed from the interface. Functions
//! are provided in an asynchronous way, and should be run by the
//! actor's system [`SystemRunner`].
use crate::{
    actors::{
        secure_messages::{
            CheckRecord, CheckVault, ClearCache, CreateVault, DeleteFromStore, GarbageCollect, GetData, ListIds,
            ReadFromStore, ReloadData, RevokeData, WriteToStore, WriteToVault,
        },
        secure_procedures::{CallProcedure, ProcResult, Procedure},
        snapshot_messages::{FillSnapshot, ReadFromSnapshot, WriteSnapshot},
        GetAllClients, GetClient, GetSnapshot, GetTarget, Registry, RemoveClient, SecureClient, SpawnClient,
        SwitchTarget, VaultDoesNotExist,
    },
    state::snapshot::{ReadSnapshotError, WriteSnapshotError},
    utils::{LoadFromPath, StrongholdFlags, VaultFlags},
    Location, VaultError,
};
use actix::prelude::*;
use engine::vault::{ClientId, RecordHint, RecordId};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    path::PathBuf,
    time::Duration,
};
use thiserror::Error as DeriveError;
use zeroize::Zeroize;

#[cfg(feature = "p2p")]
use crate::actors::{
    p2p::{
        messages as network_msg,
        messages::{RemoteVaultError, ShRequest, SwarmInfo},
        NetworkActor, NetworkConfig,
    },
    GetNetwork, InsertNetwork, StopNetwork,
};
#[cfg(feature = "p2p")]
use p2p::{
    firewall::{Rule, RuleDirection},
    DialErr, ListenErr, ListenRelayErr, Multiaddr, OutboundFailure, PeerId,
};
#[cfg(feature = "p2p")]
use std::io;

#[cfg(test)]
use crate::actors::ReadFromVault;

#[derive(DeriveError, Debug, Clone)]
pub enum Error<E: Debug + Display = Infallible> {
    #[error("Error sending message to Actor: `{0}`")]
    ActorMailbox(#[from] MailboxError),

    #[error("Target Actor has not been spawned or was killed.")]
    ActorNotSpawned,

    #[error("`{0}`")]
    Inner(E),
}

#[cfg(feature = "p2p")]
/// Error on performing an operation on a remote Stronghold.
pub type RemoteError<E = Infallible> = Error<SendRequestError<E>>;

#[cfg(feature = "p2p")]
impl<E: Debug + Display> RemoteError<E> {
    fn inner(e: E) -> Self {
        RemoteError::Inner(SendRequestError::Inner(e))
    }
}

#[cfg(feature = "p2p")]
impl<E: Debug + Display> From<OutboundFailure> for RemoteError<E> {
    fn from(f: OutboundFailure) -> Self {
        RemoteError::Inner(SendRequestError::OutboundFailure(f))
    }
}

#[cfg(feature = "p2p")]
#[derive(DeriveError, Debug, Clone)]
pub enum SendRequestError<E: Debug + Display> {
    #[error("Outbound Failure `{0}`")]
    OutboundFailure(OutboundFailure),

    #[error("`{0}`")]
    Inner(E),
}

#[derive(Clone)]
/// The main type for the Stronghold System.  Used as the entry point for the actor model.  Contains various pieces of
/// metadata to interpret the data in the vault and store.
pub struct Stronghold {
    registry: Addr<Registry>,
}

impl Stronghold {
    /// Initializes a new instance of the system asynchronously.  Sets up the first client actor. Accepts
    /// the first client_path: `Vec<u8>` and any `StrongholdFlags` which pertain to the first actor.
    /// The [`actix::SystemRunner`] is not being used directly by stronghold, and must be initialized externally.
    pub async fn init_stronghold_system(
        client_path: Vec<u8>,
        _options: Vec<StrongholdFlags>,
    ) -> Result<Self, MailboxError> {
        // Init actor registry.
        let registry = Registry::default().start();

        let mut stronghold = Self { registry };

        match stronghold.spawn_stronghold_actor(client_path, _options).await {
            Ok(_) => Ok(stronghold),
            Err(e) => Err(e),
        }
    }

    /// Spawns a new set of actors for the Stronghold system. Accepts the client_path: [`Vec<u8>`] and the options:
    /// `StrongholdFlags`
    pub async fn spawn_stronghold_actor(
        &mut self,
        client_path: Vec<u8>,
        _options: Vec<StrongholdFlags>,
    ) -> Result<(), MailboxError> {
        let client_id = ClientId::load_from_path(&client_path, &client_path.clone());
        self.registry.send(SpawnClient { id: client_id }).await?;

        match self.switch_client(client_id).await {
            Ok(_) => Ok(()),
            Err(Error::ActorMailbox(e)) => Err(e),
            _ => unreachable!(),
        }
    }

    /// Switches the actor target to another actor in the system specified by the client_path: [`Vec<u8>`].
    pub async fn switch_actor_target(&mut self, client_path: Vec<u8>) -> Result<(), Error> {
        let client_id = ClientId::load_from_path(&client_path, &client_path);
        self.switch_client(client_id).await.map(|_| ())
    }

    /// Writes data into the Stronghold. Uses the current target actor as the client and writes to the specified
    /// location of [`Location`] type. The payload must be specified as a [`Vec<u8>`] and a [`RecordHint`] can be
    /// provided. Also accepts [`VaultFlags`] for when a new Vault is created.
    pub async fn write_to_vault(
        &self,
        location: Location,
        payload: Vec<u8>,
        hint: RecordHint,
        _options: Vec<VaultFlags>,
    ) -> Result<(), Error<VaultError>> {
        let target = self.target().await?;

        let vault_path = location.vault_path().to_vec();

        let vault_exists = target.send(CheckVault { vault_path }).await?;
        if !vault_exists {
            // does not exist
            target
                .send(CreateVault {
                    location: location.clone(),
                })
                .await?;
        }
        // write to vault
        target
            .send(WriteToVault {
                location,
                payload,
                hint,
            })
            .await?
            .map_err(Error::Inner)
    }

    /// Writes data into an insecure cache.  This method, accepts a [`Vec<u8>`] as key, a [`Vec<u8>`] payload, and an
    /// optional [`Duration`]. The lifetime allows the data to be deleted after the specified duration has passed.
    /// If no lifetime is specified, the data will persist until it is manually deleted or over-written.
    /// Returns [`None`] if the key didn't exist yet. If the key is already present, the value is updated, and the old
    /// value is returned.
    ///
    /// Note: One store is mapped to one client. The same key can be specified across multiple clients.
    pub async fn write_to_store(
        &self,
        key: Vec<u8>,
        payload: Vec<u8>,
        lifetime: Option<Duration>,
    ) -> Result<Option<Vec<u8>>, Error> {
        let target = self.target().await?;
        let existing = target.send(WriteToStore { key, payload, lifetime }).await?;
        Ok(existing)
    }

    /// A method that reads from an insecure cache. This method, accepts a [`Vec<u8>`] as key and returns the payload
    /// in the form of a ([`Vec<u8>`].  If the key does not exist, `None` is returned.
    ///
    /// Note: One store is mapped to one client. The same key can be specified across multiple clients.
    pub async fn read_from_store(&self, key: Vec<u8>) -> Result<Option<Vec<u8>>, Error> {
        let target = self.target().await?;
        let data = target.send(ReadFromStore { key }).await?;
        Ok(data)
    }

    /// A method to delete data from an insecure cache. This method, accepts a [`Vec<u8>`] as key.
    ///
    /// Note: One store is mapped to one client. The same key can be specified across multiple clients.
    pub async fn delete_from_store(&self, key: Vec<u8>) -> Result<(), Error> {
        let target = self.target().await?;
        target.send(DeleteFromStore { key }).await?;
        Ok(())
    }

    /// Revokes the data from the specified location of type [`Location`]. Revoked data is not readable and can be
    /// removed from a vault with a call to `garbage_collect`.  if the `should_gc` flag is set to `true`, this call
    /// with automatically cleanup the revoke. Otherwise, the data is just marked as revoked.
    pub async fn delete_data(&self, location: Location, should_gc: bool) -> Result<(), Error<VaultError>> {
        let target = self.target().await?;
        target
            .send(RevokeData {
                location: location.clone(),
            })
            .await?
            .map_err(Error::Inner)?;
        if should_gc {
            target
                .send(GarbageCollect { location })
                .await?
                .map_err(|e| Error::Inner(e.into()))?;
        }
        Ok(())
    }

    /// Garbage collects any revokes in a Vault based on the given `vault_path` and the current target actor.
    pub async fn garbage_collect<V: Into<Vec<u8>>>(&self, vault_path: V) -> Result<(), Error<VaultDoesNotExist>> {
        let target = self.target().await?;
        target
            .send(GarbageCollect {
                location: Location::Generic {
                    vault_path: vault_path.into(),
                    record_path: Vec::new(),
                },
            })
            .await?
            .map_err(Error::Inner)
    }

    /// Returns a list of the available [`RecordId`] and [`RecordHint`] values in a vault by the given `vault_path`.
    pub async fn list_hints_and_ids<V: Into<Vec<u8>>>(
        &self,
        vault_path: V,
    ) -> Result<Vec<(RecordId, RecordHint)>, Error<VaultDoesNotExist>> {
        let target = self.target().await?;
        target
            .send(ListIds {
                vault_path: vault_path.into(),
            })
            .await?
            .map_err(Error::Inner)
    }

    /// Executes a runtime command given a [`Procedure`].  Returns a [`ProcResult`] based off of the control_request
    /// specified.
    pub async fn runtime_exec(&self, control_request: Procedure) -> Result<ProcResult, Error> {
        let target = self.target().await?;
        let result = target
            .send(CallProcedure { proc: control_request })
            .await?
            .unwrap_or_else(ProcResult::Error);
        Ok(result)
    }

    /// Checks whether a record exists in the client based off of the given [`Location`].
    pub async fn record_exists(&self, location: Location) -> Result<bool, Error> {
        let target = self.target().await?;
        let exists = target.send(CheckRecord { location }).await?;
        Ok(exists)
    }

    /// checks whether a vault exists in the client.
    pub async fn vault_exists<V: Into<Vec<u8>>>(&self, vault_path: V) -> Result<bool, Error> {
        let target = self.target().await?;
        let exists = target
            .send(CheckVault {
                vault_path: vault_path.into(),
            })
            .await?;
        Ok(exists)
    }

    /// Reads data from a given snapshot file.  Can only read the data for a single `client_path` at a time. If the new
    /// actor uses a new `client_path` the former client path may be passed into the function call to read the data into
    /// that actor. Also requires keydata to unlock the snapshot. A filename and filepath can be specified. The Keydata
    /// should implement and use Zeroize.
    pub async fn read_snapshot<T: Zeroize + AsRef<Vec<u8>>>(
        &mut self,
        client_path: Vec<u8>,
        former_client_path: Option<Vec<u8>>,
        keydata: &T,
        filename: Option<String>,
        path: Option<PathBuf>,
    ) -> Result<(), Error<ReadSnapshotError>> {
        let client_id = ClientId::load_from_path(&client_path, &client_path);
        let former_client_id = former_client_path.map(|cp| ClientId::load_from_path(&cp, &cp));

        // this feature resembles the functionality given by the former riker
        // system dependence. if there is a former client id path present,
        // the new actor is being changed into the former one ( see old ReloadData impl.)
        let target;
        if let Some(id) = former_client_id {
            target = self.switch_client(id).await.map_err(|err| match err {
                Error::ActorMailbox(e) => Error::ActorMailbox(e),
                Error::ActorNotSpawned => Error::ActorNotSpawned,
                Error::Inner(_) => unreachable!(),
            })?;
        } else {
            target = self.target::<ReadSnapshotError>().await?;
        }

        let mut key: [u8; 32] = [0u8; 32];
        let keydata = keydata.as_ref();

        key.copy_from_slice(keydata);

        // get address of snapshot actor
        let snapshot_actor = self.registry.send(GetSnapshot {}).await?;

        // read the snapshots contents
        let result = snapshot_actor
            .send(ReadFromSnapshot {
                key,
                filename,
                path,
                id: client_id,
                fid: former_client_id,
            })
            .await?
            .map_err(Error::Inner)?;

        // send data to secure actor and reload
        target
            .send(ReloadData {
                data: result.data,
                id: result.id,
            })
            .await?;
        Ok(())
    }

    /// Writes the entire state of the [`Stronghold`] into a snapshot.  All Actors and their associated data will be
    /// written into the specified snapshot. Requires keydata to encrypt the snapshot and a filename and path can be
    /// specified. The Keydata should implement and use Zeroize.
    pub async fn write_all_to_snapshot<T: Zeroize + AsRef<Vec<u8>>>(
        &mut self,
        keydata: &T,
        filename: Option<String>,
        path: Option<PathBuf>,
    ) -> Result<(), Error<WriteSnapshotError>> {
        // this should be delegated to the secure client actor
        // wrapping the interior functionality inside it.
        let clients: Vec<(ClientId, Addr<SecureClient>)> = self.registry.send(GetAllClients).await?;

        let mut key: [u8; 32] = [0u8; 32];
        let keydata = keydata.as_ref();
        key.copy_from_slice(keydata);

        // get snapshot actor
        let snapshot = self.registry.send(GetSnapshot {}).await?;

        for (id, client) in clients {
            // get data from secure actor
            let data = client.send(GetData {}).await?;

            // fill into snapshot
            snapshot.send(FillSnapshot { data, id }).await?;
        } // end loop

        // write snapshot
        snapshot
            .send(WriteSnapshot { key, filename, path })
            .await?
            .map_err(Error::Inner)?;
        Ok(())
    }

    /// Used to kill a stronghold actor or clear the cache of the given actor system based on the client_path. If
    /// `kill_actor` is `true`, the actor will be removed from the system.  Otherwise, the cache of the
    /// current target actor will be cleared.
    ///
    /// **Note**: If `kill_actor` is set to `true` and the target is the currently active client, a new client has to be
    /// set via [`Stronghold::switch_actor_target`], before any following operations can be performed.
    pub async fn kill_stronghold(&mut self, client_path: Vec<u8>, kill_actor: bool) -> Result<(), Error> {
        let client_id = ClientId::load_from_path(&client_path.clone(), &client_path);
        let client;
        if kill_actor {
            client = self
                .registry
                .send(RemoveClient { id: client_id })
                .await?
                .ok_or(Error::ActorNotSpawned)?;
        } else {
            client = self
                .registry
                .send(GetClient { id: client_id })
                .await?
                .ok_or(Error::ActorNotSpawned)?;
        }
        client.send(ClearCache).await?;
        Ok(())
    }

    /// Unimplemented until Policies are implemented.
    #[allow(dead_code)]
    fn check_config_flags() {
        unimplemented!()
    }

    /// A test function for reading data from a vault.
    // API CHANGE!
    #[cfg(test)]
    pub async fn read_secret(&self, _client_path: Vec<u8>, location: Location) -> Result<Option<Vec<u8>>, Error> {
        let target = self.target().await?;
        let secret = target.send(ReadFromVault { location }).await?;
        Ok(secret)
    }

    async fn switch_client(&mut self, client_id: ClientId) -> Result<Addr<SecureClient>, Error> {
        self.registry
            .send(SwitchTarget { id: client_id })
            .await?
            .ok_or(Error::ActorNotSpawned)
    }

    async fn target<E: Debug + Display>(&self) -> Result<Addr<SecureClient>, Error<E>> {
        self.registry.send(GetTarget).await?.ok_or(Error::ActorNotSpawned)
    }
}

#[cfg(feature = "p2p")]
impl Stronghold {
    /// Spawn the p2p-network actor and swarm.
    ///
    /// Return `Ok(false)` if there is an existing network actor and no new one was spawned.
    pub async fn spawn_p2p(
        &mut self,
        firewall_rule: Rule<ShRequest>,
        network_config: NetworkConfig,
    ) -> Result<bool, Error<io::Error>> {
        if self.registry.send(GetNetwork).await?.is_some() {
            return Ok(false);
        }
        let addr = NetworkActor::new(self.registry.clone(), firewall_rule, network_config)
            .await
            .map_err(Error::Inner)?
            .start();
        self.registry.send(InsertNetwork { addr }).await?;
        Ok(true)
    }

    /// Gracefully stop the network actor and swarm.
    /// Return `false` if there is no active network actor.
    pub async fn stop_p2p(&mut self) -> Result<bool, MailboxError> {
        self.registry.send(StopNetwork).await
    }

    /// Start listening on the swarm to the given address. If not address is provided, it will be assigned by the OS.
    pub async fn start_listening(&self, address: Option<Multiaddr>) -> Result<Multiaddr, Error<ListenErr>> {
        let actor = self.network_actor().await?;
        actor
            .send(network_msg::StartListening { address })
            .await?
            .map_err(Error::Inner)
    }

    /// Stop listening on the swarm.
    pub async fn stop_listening(&self) -> Result<(), Error<ListenErr>> {
        let actor = self.network_actor().await?;
        actor.send(network_msg::StopListening).await?;
        Ok(())
    }

    ///  Get the peer id, listening addresses and connection info of the local peer
    pub async fn get_swarm_info(&self) -> Result<SwarmInfo, Error> {
        let actor = self.network_actor().await?;
        let info = actor.send(network_msg::GetSwarmInfo).await?;
        Ok(info)
    }

    /// Add dial information for a remote peers.
    /// This will attempt to connect the peer directly either by the address if one is provided, or by peer id
    /// if the peer is already known e.g. from multicast DNS.
    /// If the peer is not a relay and can not be reached directly, it will be attempted to reach it via the relays,
    /// if there are any.
    pub async fn add_peer(&self, peer: PeerId, address: Option<Multiaddr>) -> Result<Multiaddr, Error<DialErr>> {
        let actor = self.network_actor().await?;
        if let Some(address) = address {
            actor.send(network_msg::AddPeerAddr { peer, address }).await?;
        }
        actor
            .send(network_msg::ConnectPeer { peer })
            .await?
            .map_err(Error::Inner)
    }

    /// Add a relay to the list of relays that may be tried to use if a remote peer can not be reached directly.
    pub async fn add_dialing_relay(
        &self,
        relay: PeerId,
        relay_addr: Option<Multiaddr>,
    ) -> Result<Option<Multiaddr>, Error> {
        let actor = self.network_actor().await?;
        let addr = actor.send(network_msg::AddDialingRelay { relay, relay_addr }).await?;
        Ok(addr)
    }

    /// Start listening via a relay peer on an address following the scheme
    /// `<relay-addr>/<relay-id>/p2p-circuit/<local-id>`. This will establish a keep-alive connection to the relay,
    /// the relay will forward all requests to the local peer.
    pub async fn start_relayed_listening(
        &self,
        relay: PeerId,
        relay_addr: Option<Multiaddr>,
    ) -> Result<Multiaddr, Error<ListenRelayErr>> {
        let actor = self.network_actor().await?;
        actor
            .send(network_msg::StartListeningRelay { relay, relay_addr })
            .await?
            .map_err(Error::Inner)
    }

    /// Stop listening with the relay.
    pub async fn remove_listening_relay(&self, relay: PeerId) -> Result<(), Error> {
        let actor = self.network_actor().await?;
        actor.send(network_msg::StopListeningRelay { relay }).await?;
        Ok(())
    }

    /// Remove a peer from the list of peers used for dialing.
    pub async fn remove_dialing_relay(&self, relay: PeerId) -> Result<(), Error> {
        let actor = self.network_actor().await?;
        actor.send(network_msg::RemoveDialingRelay { relay }).await?;
        Ok(())
    }

    /// Change the firewall rule for specific peers, optionally also set it as the default rule, which applies if there
    /// are no specific rules for a peer. All inbound requests from the peers that this rule applies to, will be
    /// approved/ rejected based on this rule.
    pub async fn set_firewall_rule(
        &self,
        rule: Rule<ShRequest>,
        peers: Vec<PeerId>,
        set_default: bool,
    ) -> Result<(), Error> {
        let actor = self.network_actor().await?;

        if set_default {
            actor
                .send(network_msg::SetFirewallDefault {
                    direction: RuleDirection::Inbound,
                    rule: rule.clone(),
                })
                .await?;
        }

        for peer in peers {
            actor
                .send(network_msg::SetFirewallRule {
                    peer,
                    direction: RuleDirection::Inbound,
                    rule: rule.clone(),
                })
                .await?;
        }
        Ok(())
    }

    /// Remove peer specific rules from the firewall configuration.
    pub async fn remove_firewall_rules(&self, peers: Vec<PeerId>) -> Result<(), Error> {
        let actor = self.network_actor().await?;
        for peer in peers {
            actor
                .send(network_msg::RemoveFirewallRule {
                    peer,
                    direction: RuleDirection::Inbound,
                })
                .await?;
        }
        Ok(())
    }

    /// Write to the vault of a remote Stronghold.
    pub async fn write_remote_vault(
        &self,
        peer: PeerId,
        location: Location,
        payload: Vec<u8>,
        hint: RecordHint,
        _options: Vec<VaultFlags>,
    ) -> Result<(), RemoteError<RemoteVaultError>> {
        let actor = self.network_actor().await?;

        let vault_path = location.vault_path().to_vec();

        // check if vault exists
        let send_request = network_msg::SendRequest {
            peer,
            request: CheckVault { vault_path },
        };
        let vault_exists = actor.send(send_request).await??;

        // no vault so create new one before writing.
        if !vault_exists {
            let send_request = network_msg::SendRequest {
                peer,
                request: CreateVault {
                    location: location.clone(),
                },
            };
            actor.send(send_request).await??;
        }

        // write data
        let send_request = network_msg::SendRequest {
            peer,
            request: network_msg::WriteToRemoteVault {
                location: location.clone(),
                payload: payload.clone(),
                hint,
            },
        };
        actor.send(send_request).await??.map_err(RemoteError::inner)
    }

    /// Write to the store of a remote Stronghold.
    ///
    /// Returns [`None`] if the key didn't exist yet. If the key is already present, the value is updated, and the old
    /// value is returned.
    pub async fn write_to_remote_store(
        &self,
        peer: PeerId,
        key: Vec<u8>,
        payload: Vec<u8>,
        lifetime: Option<Duration>,
    ) -> Result<Option<Vec<u8>>, RemoteError> {
        let actor = self.network_actor().await?;
        let send_request = network_msg::SendRequest {
            peer,
            request: WriteToStore { key, payload, lifetime },
        };
        let existing = actor.send(send_request).await??;
        Ok(existing)
    }

    /// Read from the store of a remote Stronghold.
    pub async fn read_from_remote_store(&self, peer: PeerId, key: Vec<u8>) -> Result<Option<Vec<u8>>, RemoteError> {
        let actor = self.network_actor().await?;
        let send_request = network_msg::SendRequest {
            peer,
            request: ReadFromStore { key },
        };
        let data = actor.send(send_request).await??;
        Ok(data)
    }

    /// Returns a list of the available records and their `RecordHint` values of a remote vault.
    pub async fn list_remote_hints_and_ids<V: Into<Vec<u8>>>(
        &self,
        peer: PeerId,
        vault_path: V,
    ) -> Result<Vec<(RecordId, RecordHint)>, RemoteError<VaultDoesNotExist>> {
        let actor = self.network_actor().await?;
        let send_request = network_msg::SendRequest {
            peer,
            request: ListIds {
                vault_path: vault_path.into(),
            },
        };
        actor.send(send_request).await??.map_err(RemoteError::inner)
    }

    /// Executes a runtime command at a remote Stronghold.
    /// It is required that the peer has successfully been added with the `add_peer` method.
    pub async fn remote_runtime_exec(
        &self,
        peer: PeerId,
        control_request: Procedure,
    ) -> Result<ProcResult, RemoteError> {
        let actor = self.network_actor().await?;
        let send_request = network_msg::SendRequest {
            peer,
            request: CallProcedure { proc: control_request },
        };
        let result = actor.send(send_request).await??.unwrap_or_else(ProcResult::Error);
        Ok(result)
    }

    async fn network_actor<E: Debug + Display>(&self) -> Result<Addr<NetworkActor>, Error<E>> {
        self.registry.send(GetNetwork).await?.ok_or(Error::ActorNotSpawned)
    }
}
