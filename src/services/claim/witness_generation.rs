use anyhow::{ensure, Ok};
use intmax2_zkp::{
    ethereum_types::{address::Address, bytes32::Bytes32, u32limb_trait::U32LimbTrait},
    utils::leafable::Leafable,
};
use log::info;
use mining_circuit_v1::claim::claim_inner_circuit::ClaimInnerValue;

use crate::{
    external_api::contracts::events::Deposited,
    services::claim::MAX_CLAIMS,
    state::{key::Key, state::State},
    utils::derive_key::{derive_pubkey_from_private_key, derive_salt_from_private_key_nonce},
};

pub async fn generate_claim_witness(
    state: &State,
    key: &Key,
    is_short_term: bool,
    events: &[Deposited],
) -> anyhow::Result<Vec<ClaimInnerValue>> {
    info!(
        "Generating claim witness for {:?}. is_short_term = {}",
        events, is_short_term
    );
    ensure!(events.len() > 0, "No event to generate witness");
    ensure!(
        events.len() <= MAX_CLAIMS,
        format!("Max {} events to generate witness", MAX_CLAIMS)
    );
    let deposit_tree_root = state.deposit_hash_tree.get_root();
    let eligible_tree = if is_short_term {
        &state.short_term_eligible_tree
    } else {
        &state.long_term_eligible_tree
    };

    let eligible_tree_root: Bytes32 = eligible_tree.get_root().into();
    let pubkey = derive_pubkey_from_private_key(key.deposit_private_key);
    let recipient = Address::from_bytes_be(&key.withdrawal_address.as_bytes());
    let mut witnesses = Vec::new();
    let mut prev_claim_hash = Bytes32::default();
    for event in events {
        let deposit_index = state
            .deposit_hash_tree
            .get_index(event.deposit().hash())
            .unwrap();
        let deposit_merkle_proof = state.deposit_hash_tree.prove(deposit_index);
        let deposit = event.deposit();

        let eligible_index = eligible_tree.get_leaf_index(deposit_index).unwrap();
        let eligible_merkle_proof = eligible_tree.tree.prove(eligible_index as usize);
        let eligible_leaf = eligible_tree.tree.get_leaf(eligible_index as usize);
        let salt = derive_salt_from_private_key_nonce(key.deposit_private_key, event.tx_nonce);
        let value = ClaimInnerValue::new(
            deposit_tree_root,
            deposit_index,
            deposit_merkle_proof,
            deposit,
            eligible_tree_root,
            eligible_index,
            eligible_merkle_proof,
            eligible_leaf,
            pubkey,
            salt,
            recipient,
            prev_claim_hash,
        )?;
        prev_claim_hash = value.new_claim_hash;
        witnesses.push(value);
    }
    Ok(witnesses)
}
