// Copyright (c) Zefchain Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! Conversions from types generated by [`wit-bindgen`] to types declared in [`linera-sdk`].

use linera_base::{
    crypto::CryptoHash,
    data_types::{Amount, BlockHeight, TimeDelta, Timestamp},
    http,
    identifiers::{AccountOwner, ApplicationId, ChainId, ModuleId, Owner},
    ownership::{ChainOwnership, TimeoutConfig},
    vm::VmRuntime,
};

use crate::{
    contract::wit::base_runtime_api as base_contract_api,
    service::wit::base_runtime_api as base_service_api,
};

macro_rules! impl_from_wit {
    ($wit_base_api:ident) => {
        impl From<$wit_base_api::CryptoHash> for CryptoHash {
            fn from(hash_value: $wit_base_api::CryptoHash) -> Self {
                CryptoHash::from([
                    hash_value.part1,
                    hash_value.part2,
                    hash_value.part3,
                    hash_value.part4,
                ])
            }
        }

        impl From<$wit_base_api::Owner> for Owner {
            fn from(owner: $wit_base_api::Owner) -> Self {
                Owner(owner.inner0.into())
            }
        }

        impl From<$wit_base_api::AccountOwner> for AccountOwner {
            fn from(account_owner: $wit_base_api::AccountOwner) -> Self {
                match account_owner {
                    $wit_base_api::AccountOwner::User(owner) => AccountOwner::User(owner.into()),
                    $wit_base_api::AccountOwner::Application(owner) => {
                        AccountOwner::Application(owner.into())
                    }
                }
            }
        }

        impl From<$wit_base_api::Amount> for Amount {
            fn from(balance: $wit_base_api::Amount) -> Self {
                let (lower_half, upper_half) = balance.inner0;
                let value = ((upper_half as u128) << 64) | (lower_half as u128);
                Amount::from_attos(value)
            }
        }

        impl From<$wit_base_api::BlockHeight> for BlockHeight {
            fn from(block_height: $wit_base_api::BlockHeight) -> Self {
                BlockHeight(block_height.inner0)
            }
        }

        impl From<$wit_base_api::ChainId> for ChainId {
            fn from(chain_id: $wit_base_api::ChainId) -> Self {
                ChainId(chain_id.inner0.into())
            }
        }

        impl From<$wit_base_api::ModuleId> for ModuleId {
            fn from(module_id: $wit_base_api::ModuleId) -> Self {
                ModuleId::new(
                    module_id.contract_blob_hash.into(),
                    module_id.service_blob_hash.into(),
                    module_id.vm_runtime.into(),
                )
            }
        }

        impl From<$wit_base_api::VmRuntime> for VmRuntime {
            fn from(vm_runtime: $wit_base_api::VmRuntime) -> Self {
                match vm_runtime {
                    $wit_base_api::VmRuntime::Wasm => VmRuntime::Wasm,
                    $wit_base_api::VmRuntime::Evm => VmRuntime::Evm,
                }
            }
        }

        impl From<$wit_base_api::ApplicationId> for ApplicationId {
            fn from(application_id: $wit_base_api::ApplicationId) -> Self {
                ApplicationId {
                    application_description_hash: application_id
                        .application_description_hash
                        .into(),
                    module_id: application_id.module_id.into(),
                }
            }
        }

        impl From<$wit_base_api::Timestamp> for Timestamp {
            fn from(timestamp: $wit_base_api::Timestamp) -> Self {
                Timestamp::from(timestamp.inner0)
            }
        }

        impl From<$wit_base_api::TimeDelta> for TimeDelta {
            fn from(guest: $wit_base_api::TimeDelta) -> Self {
                TimeDelta::from_micros(guest.inner0)
            }
        }

        impl From<$wit_base_api::TimeoutConfig> for TimeoutConfig {
            fn from(guest: $wit_base_api::TimeoutConfig) -> TimeoutConfig {
                let $wit_base_api::TimeoutConfig {
                    fast_round_duration,
                    base_timeout,
                    timeout_increment,
                    fallback_duration,
                } = guest;
                TimeoutConfig {
                    fast_round_duration: fast_round_duration.map(TimeDelta::from),
                    base_timeout: base_timeout.into(),
                    timeout_increment: timeout_increment.into(),
                    fallback_duration: fallback_duration.into(),
                }
            }
        }

        impl From<$wit_base_api::ChainOwnership> for ChainOwnership {
            fn from(guest: $wit_base_api::ChainOwnership) -> ChainOwnership {
                let $wit_base_api::ChainOwnership {
                    super_owners,
                    owners,
                    multi_leader_rounds,
                    open_multi_leader_rounds,
                    timeout_config,
                } = guest;
                ChainOwnership {
                    super_owners: super_owners.into_iter().map(Into::into).collect(),
                    owners: owners
                        .into_iter()
                        .map(|(owner, weight)| (owner.into(), weight))
                        .collect(),
                    multi_leader_rounds,
                    open_multi_leader_rounds,
                    timeout_config: timeout_config.into(),
                }
            }
        }

        impl From<$wit_base_api::HttpResponse> for http::Response {
            fn from(response: $wit_base_api::HttpResponse) -> http::Response {
                http::Response {
                    status: response.status,
                    headers: response
                        .headers
                        .into_iter()
                        .map(http::Header::from)
                        .collect(),
                    body: response.body,
                }
            }
        }

        impl From<$wit_base_api::HttpHeader> for http::Header {
            fn from(header: $wit_base_api::HttpHeader) -> http::Header {
                http::Header::new(header.name, header.value)
            }
        }
    };
}

impl_from_wit!(base_service_api);
impl_from_wit!(base_contract_api);
