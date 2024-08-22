// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>.
//!
//! The Tanssi AuRa consensus algorithm for orchestrator chain and container chain collators.    
//! This file contains those functions that are used by consensus_orchestrator.rs structs and
//! and traits
//! slot_author returns the author based on the slot number and authorities provided (aura-like)
//! authorities retrieves the current set of authorities based on the first eligible key found in the keystore

pub mod collators;
mod consensus_orchestrator;
mod manual_seal;

#[cfg(test)]
mod tests;

use cumulus_relay_chain_interface::RelayChainInterface;
use futures::executor::block_on;
use polkadot_primitives::SessionIndex;
pub use {
    crate::consensus_orchestrator::OrchestratorAuraWorkerAuxData,
    cumulus_primitives_core::ParaId,
    dp_consensus::TanssiAuthorityAssignmentApi,
    manual_seal::{
        get_aura_id_from_seed, ContainerManualSealAuraConsensusDataProvider,
        OrchestratorManualSealAuraConsensusDataProvider,
    },
    pallet_registrar_runtime_api::OnDemandBlockProductionApi,
    parity_scale_codec::{Decode, Encode},
    sc_consensus_aura::{
        find_pre_digest, slot_duration, AuraVerifier, BuildAuraWorkerParams, CompatibilityMode,
        SlotProportion,
    },
    sc_consensus_slots::InherentDataProviderExt,
    sp_api::{Core, ProvideRuntimeApi},
    sp_application_crypto::AppPublic,
    sp_consensus::Error as ConsensusError,
    sp_core::crypto::{ByteArray, Public},
    sp_core::H256,
    sp_keystore::{Keystore, KeystorePtr},
    sp_runtime::traits::{Block as BlockT, Header as HeaderT, Member, NumberFor},
    std::hash::Hash,
};

use {sp_consensus_slots::Slot, sp_core::crypto::Pair};

const LOG_TARGET: &str = "aura::tanssi";

type AuthorityId<P> = <P as Pair>::Public;

/// Get slot author for given block along with authorities.
pub(crate) fn slot_author<P: Pair>(
    slot: Slot,
    authorities: &[AuthorityId<P>],
) -> Option<&AuthorityId<P>> {
    if authorities.is_empty() {
        return None;
    }

    let idx = *slot % (authorities.len() as u64);
    assert!(
        idx <= usize::MAX as u64,
        "It is impossible to have a vector with length beyond the address space; qed",
    );

    let current_author = authorities.get(idx as usize).expect(
        "authorities not empty; index constrained to list length;this is a valid index; qed",
    );

    Some(current_author)
}

/// Return the set of authorities assigned to the paraId where
/// the first eligible key from the keystore is collating
pub fn authorities<B, C, P>(
    client: &C,
    parent_hash: &B::Hash,
    para_id: ParaId,
) -> Option<Vec<AuthorityId<P>>>
where
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    B: BlockT,
    C: ProvideRuntimeApi<B>,
    C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    let runtime_api = client.runtime_api();

    let authorities = runtime_api
        .para_id_authorities(*parent_hash, para_id)
        .ok()?;
    log::info!(
        "Authorities found for para {:?} are {:?}",
        para_id,
        authorities
    );
    authorities
}

/// Return the set of authorities assigned to the paraId where
/// the first eligible key from the keystore is collating
pub fn min_slot_freq<B, C, P>(
    client: &C,
    parent_hash: &B::Hash,
    para_id: ParaId,
) -> Option<SlotFrequency>
where
    P: Pair + Send + Sync + 'static,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    B: BlockT,
    C: ProvideRuntimeApi<B>,
    C::Api: OnDemandBlockProductionApi<B, ParaId, Slot>,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    let runtime_api = client.runtime_api();

    let slot_frequency = runtime_api
        .parathread_slot_frequency(*parent_hash, para_id)
        .ok()?;
    log::debug!("slot_freq for para {:?} is {:?}", para_id, slot_frequency);
    slot_frequency
}

use {
    nimbus_primitives::{NimbusId, NimbusPair, NIMBUS_KEY_ID},
    tp_traits::SlotFrequency,
};

/// Grab the first eligible nimbus key from the keystore
/// If multiple keys are eligible this function still only returns one
/// and makes no guarantees which one as that depends on the keystore's iterator behavior.
/// This is the standard way of determining which key to author with.
/// It also returns its ParaId assignment
pub fn first_eligible_key<B: BlockT, C, P>(
    client: &C,
    parent_hash: &B::Hash,
    keystore: KeystorePtr,
) -> Option<(AuthorityId<P>, ParaId)>
where
    C: ProvideRuntimeApi<B> + ?Sized,
    C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    // Get all the available keys
    let available_keys = Keystore::keys(&*keystore, NIMBUS_KEY_ID).ok()?;

    // Print a more helpful message than "not eligible" when there are no keys at all.
    if available_keys.is_empty() {
        log::warn!(
            target: LOG_TARGET,
            "🔏 No Nimbus keys available. We will not be able to author."
        );
        return None;
    }

    let runtime_api = client.runtime_api();

    // Iterate keys until we find an eligible one, or run out of candidates.
    // If we are skipping prediction, then we author with the first key we find.
    // prediction skipping only really makes sense when there is a single key in the keystore.
    available_keys.into_iter().find_map(|type_public_pair| {
        if let Ok(nimbus_id) = NimbusId::from_slice(&type_public_pair) {
            // If we dont find any parachain that we are assigned to, return none

            if let Ok(Some(para_id)) =
                runtime_api.check_para_id_assignment(*parent_hash, nimbus_id.clone().into())
            {
                log::debug!("Para id found for assignment {:?}", para_id);

                Some((nimbus_id.into(), para_id))
            } else {
                log::debug!("No Para id found for assignment {:?}", nimbus_id);

                None
            }
        } else {
            None
        }
    })
}

/// Grab the first eligible nimbus key from the keystore
/// If multiple keys are eligible this function still only returns one
/// and makes no guarantees which one as that depends on the keystore's iterator behavior.
/// This is the standard way of determining which key to author with.
/// It also returns its ParaId assignment
pub fn first_eligible_key_next_session<B: BlockT, C, P>(
    client: &C,
    parent_hash: &B::Hash,
    keystore: KeystorePtr,
) -> Option<(AuthorityId<P>, ParaId)>
where
    C: ProvideRuntimeApi<B> + ?Sized,
    C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    // Get all the available keys
    let available_keys = Keystore::keys(&*keystore, NIMBUS_KEY_ID).ok()?;

    // Print a more helpful message than "not eligible" when there are no keys at all.
    if available_keys.is_empty() {
        log::warn!(
            target: LOG_TARGET,
            "🔏 No Nimbus keys available. We will not be able to author."
        );
        return None;
    }

    let runtime_api = client.runtime_api();

    // Iterate keys until we find an eligible one, or run out of candidates.
    // If we are skipping prediction, then we author with the first key we find.
    // prediction skipping only really makes sense when there is a single key in the keystore.
    available_keys.into_iter().find_map(|type_public_pair| {
        if let Ok(nimbus_id) = NimbusId::from_slice(&type_public_pair) {
            // If we dont find any parachain that we are assigned to, return none

            if let Ok(Some(para_id)) = runtime_api
                .check_para_id_assignment_next_session(*parent_hash, nimbus_id.clone().into())
            {
                log::debug!("Para id found for assignment {:?}", para_id);

                Some((nimbus_id.into(), para_id))
            } else {
                log::debug!("No Para id found for assignment {:?}", nimbus_id);

                None
            }
        } else {
            None
        }
    })
}

/// Grab the first eligible nimbus key from the keystore
/// If multiple keys are eligible this function still only returns one
/// and makes no guarantees which one as that depends on the keystore's iterator behavior.
/// This is the standard way of determining which key to author with.
/// It also returns its ParaId assignment
pub fn first_eligible_key_solochain<B: BlockT, C, P>(
    client: &C,
    parent_hash: &H256,
    keystore: KeystorePtr,
) -> Option<(AuthorityId<P>, ParaId)>
where
    C: RelayChainInterface + ?Sized,
    //C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    // Get all the available keys
    let available_keys = Keystore::keys(&*keystore, NIMBUS_KEY_ID).ok()?;

    // Print a more helpful message than "not eligible" when there are no keys at all.
    if available_keys.is_empty() {
        log::warn!(
            target: LOG_TARGET,
            "🔏 No Nimbus keys available. We will not be able to author."
        );
        return None;
    }

    // Iterate keys until we find an eligible one, or run out of candidates.
    // If we are skipping prediction, then we author with the first key we find.
    // prediction skipping only really makes sense when there is a single key in the keystore.
    available_keys.into_iter().find_map(|type_public_pair| {
        if let Ok(nimbus_id) = NimbusId::from_slice(&type_public_pair) {
            // If we dont find any parachain that we are assigned to, return none
            if let Ok(Some(para_id)) =
                solochain_check_para_id_assignment(client, parent_hash, nimbus_id.clone())
            {
                log::debug!("Para id found for assignment {:?}", para_id);

                Some((nimbus_id.into(), para_id))
            } else {
                log::debug!("No Para id found for assignment {:?}", nimbus_id);

                None
            }
        } else {
            None
        }
    })
}

/// Grab the first eligible nimbus key from the keystore
/// If multiple keys are eligible this function still only returns one
/// and makes no guarantees which one as that depends on the keystore's iterator behavior.
/// This is the standard way of determining which key to author with.
/// It also returns its ParaId assignment
pub fn first_eligible_key_next_session_solochain<B: BlockT, C, P>(
    client: &C,
    parent_hash: &H256,
    keystore: KeystorePtr,
) -> Option<(AuthorityId<P>, ParaId)>
where
    C: RelayChainInterface + ?Sized,
    //C::Api: TanssiAuthorityAssignmentApi<B, AuthorityId<P>>,
    P: Pair + Send + Sync,
    P::Public: AppPublic + Hash + Member + Encode + Decode,
    P::Signature: TryFrom<Vec<u8>> + Hash + Member + Encode + Decode,
    AuthorityId<P>: From<<NimbusPair as sp_application_crypto::Pair>::Public>,
{
    // Get all the available keys
    let available_keys = Keystore::keys(&*keystore, NIMBUS_KEY_ID).ok()?;

    // Print a more helpful message than "not eligible" when there are no keys at all.
    if available_keys.is_empty() {
        log::warn!(
            target: LOG_TARGET,
            "🔏 No Nimbus keys available. We will not be able to author."
        );
        return None;
    }

    // Iterate keys until we find an eligible one, or run out of candidates.
    // If we are skipping prediction, then we author with the first key we find.
    // prediction skipping only really makes sense when there is a single key in the keystore.
    available_keys.into_iter().find_map(|type_public_pair| {
        if let Ok(nimbus_id) = NimbusId::from_slice(&type_public_pair) {
            // If we dont find any parachain that we are assigned to, return none
            if let Ok(Some(para_id)) = solochain_check_para_id_assignment_next_session(
                client,
                parent_hash,
                nimbus_id.clone(),
            ) {
                log::debug!("Para id found for assignment {:?}", para_id);

                Some((nimbus_id.into(), para_id))
            } else {
                log::debug!("No Para id found for assignment {:?}", nimbus_id);

                None
            }
        } else {
            None
        }
    })
}

fn solochain_check_para_id_assignment<C: RelayChainInterface + ?Sized>(
    client: &C,
    relay_parent: &H256,
    nimbus_id: NimbusId,
) -> Result<Option<ParaId>, ()> {
    let encoded_nimbus_id = nimbus_id.encode();
    let res: Vec<u8> = block_on(client.call_remote_runtime_function(
        "TanssiAuthorityAssignmentApi_check_para_id_assignment",
        *relay_parent,
        &encoded_nimbus_id,
    ))
    .map_err(|_| ())?;
    let res: Option<ParaId> = Decode::decode(&mut res.as_slice()).unwrap();

    Ok(res)
}

fn solochain_check_para_id_assignment_next_session<C: RelayChainInterface + ?Sized>(
    client: &C,
    relay_parent: &H256,
    nimbus_id: NimbusId,
) -> Result<Option<ParaId>, ()> {
    let encoded_nimbus_id = nimbus_id.encode();
    let res: Vec<u8> = block_on(client.call_remote_runtime_function(
        "TanssiAuthorityAssignmentApi_check_para_id_assignment_next_session",
        *relay_parent,
        &encoded_nimbus_id,
    ))
    .map_err(|_| ())?;
    let res: Option<ParaId> = Decode::decode(&mut res.as_slice()).unwrap();

    Ok(res)
}
