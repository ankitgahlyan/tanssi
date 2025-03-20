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
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_registrar
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-03-17, STEPS: `16`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Evgenys-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_registrar
// --extrinsic
// *
// --chain=dev
// --steps
// 16
// --repeat
// 1
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/dancebox_weights/pallet_registrar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_registrar using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_registrar::WeightInfo for SubstrateWeight<T> {
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaManager` (r:0 w:1)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[100, 3000000]`.
	/// The range of component `z` is `[1, 10]`.
	fn register(x: u32, _z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `266`
		//  Estimated: `3731`
		// Minimum execution time: 66_000_000 picoseconds.
		Weight::from_parts(1_193_184_714, 3731)
			// Standard Error: 193
			.saturating_add(Weight::from_parts(712, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `RelayStorageRoots::RelayStorageRoot` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaManager` (r:0 w:1)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[100, 3000000]`.
	/// The range of component `z` is `[1, 10]`.
	fn register_with_relay_proof(x: u32, _z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `364`
		//  Estimated: `3829`
		// Minimum execution time: 122_000_000 picoseconds.
		Weight::from_parts(122_000_000, 3829)
			// Standard Error: 16
			.saturating_add(Weight::from_parts(806, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `XcmCoreBuyer::InFlightOrders` (r:1 w:0)
	/// Proof: `XcmCoreBuyer::InFlightOrders` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::BufferedParasToDeregister` (r:1 w:1)
	/// Proof: `Registrar::BufferedParasToDeregister` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxTip` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxCorePrice` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxCorePrice` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:0 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaManager` (r:0 w:1)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:0 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	/// Storage: `AuthorNoting::LatestAuthor` (r:0 w:1)
	/// Proof: `AuthorNoting::LatestAuthor` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `XcmCoreBuyer::PendingBlocks` (r:0 w:1)
	/// Proof: `XcmCoreBuyer::PendingBlocks` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn deregister_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2971`
		//  Estimated: `6196`
		// Minimum execution time: 149_000_000 picoseconds.
		Weight::from_parts(149_000_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(16_u64))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:0)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:0)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::Paused` (r:1 w:0)
	/// Proof: `Registrar::Paused` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingToRemove` (r:1 w:1)
	/// Proof: `Registrar::PendingToRemove` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:1 w:0)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn deregister_scheduled() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `850`
		//  Estimated: `3493`
		// Minimum execution time: 64_000_000 picoseconds.
		Weight::from_parts(64_000_000, 3493)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `RelayStorageRoots::RelayStorageRoot` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `XcmCoreBuyer::InFlightOrders` (r:1 w:0)
	/// Proof: `XcmCoreBuyer::InFlightOrders` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::BufferedParasToDeregister` (r:1 w:1)
	/// Proof: `Registrar::BufferedParasToDeregister` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxTip` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::RefundAddress` (r:0 w:1)
	/// Proof: `ServicesPayment::RefundAddress` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::MaxCorePrice` (r:0 w:1)
	/// Proof: `ServicesPayment::MaxCorePrice` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:0 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:0 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaManager` (r:0 w:1)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Assignments` (r:0 w:1)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	/// Storage: `AuthorNoting::LatestAuthor` (r:0 w:1)
	/// Proof: `AuthorNoting::LatestAuthor` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `XcmCoreBuyer::PendingBlocks` (r:0 w:1)
	/// Proof: `XcmCoreBuyer::PendingBlocks` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	fn deregister_with_relay_proof_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3335`
		//  Estimated: `8799`
		// Minimum execution time: 253_000_000 picoseconds.
		Weight::from_parts(253_000_000, 8799)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(17_u64))
	}
	/// Storage: `RelayStorageRoots::RelayStorageRoot` (r:1 w:0)
	/// Proof: `RelayStorageRoots::RelayStorageRoot` (`max_values`: None, `max_size`: Some(44), added: 2519, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:0)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:0)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::Paused` (r:1 w:0)
	/// Proof: `Registrar::Paused` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingToRemove` (r:1 w:1)
	/// Proof: `Registrar::PendingToRemove` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::ParathreadParams` (r:1 w:0)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn deregister_with_relay_proof_scheduled() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2942`
		//  Estimated: `6196`
		// Minimum execution time: 147_000_000 picoseconds.
		Weight::from_parts(147_000_000, 6196)
			.saturating_add(T::DbWeight::get().reads(13_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `Registrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `DataPreservers::Assignments` (r:1 w:0)
	/// Proof: `DataPreservers::Assignments` (`max_values`: None, `max_size`: Some(101), added: 2576, mode: `MaxEncodedLen`)
	/// Storage: `DataPreservers::Profiles` (r:1 w:0)
	/// Proof: `DataPreservers::Profiles` (`max_values`: None, `max_size`: Some(744), added: 3219, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::GivenFreeCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::GivenFreeCredits` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:1 w:1)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParathreadParams` (r:1 w:0)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	fn mark_valid_for_collating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5165`
		//  Estimated: `6650`
		// Minimum execution time: 142_000_000 picoseconds.
		Weight::from_parts(142_000_000, 6650)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:1)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn pause_container_chain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1242`
		//  Estimated: `2727`
		// Minimum execution time: 66_000_000 picoseconds.
		Weight::from_parts(66_000_000, 2727)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Registrar::PendingParaIds` (r:1 w:1)
	/// Proof: `Registrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingPaused` (r:1 w:1)
	/// Proof: `Registrar::PendingPaused` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn unpause_container_chain() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1242`
		//  Estimated: `2727`
		// Minimum execution time: 53_000_000 picoseconds.
		Weight::from_parts(53_000_000, 2727)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(139), added: 2614, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaGenesisData` (r:1 w:1)
	/// Proof: `Registrar::ParaGenesisData` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::PendingVerification` (r:1 w:1)
	/// Proof: `Registrar::PendingVerification` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParathreadParams` (r:0 w:1)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::RegistrarDeposit` (r:0 w:1)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaManager` (r:0 w:1)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[100, 3000000]`.
	/// The range of component `z` is `[1, 10]`.
	fn register_parathread(x: u32, _z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `266`
		//  Estimated: `3731`
		// Minimum execution time: 65_000_000 picoseconds.
		Weight::from_parts(51_811_307, 3731)
			// Standard Error: 26
			.saturating_add(Weight::from_parts(743, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	/// Storage: `Registrar::ParathreadParams` (r:1 w:0)
	/// Proof: `Registrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::PendingParathreadParams` (r:1 w:1)
	/// Proof: `Registrar::PendingParathreadParams` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn set_parathread_params() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1674`
		//  Estimated: `3493`
		// Minimum execution time: 48_000_000 picoseconds.
		Weight::from_parts(48_000_000, 3493)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Registrar::RegistrarDeposit` (r:1 w:0)
	/// Proof: `Registrar::RegistrarDeposit` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Registrar::ParaManager` (r:0 w:1)
	/// Proof: `Registrar::ParaManager` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_para_manager() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `237`
		//  Estimated: `3533`
		// Minimum execution time: 14_000_000 picoseconds.
		Weight::from_parts(14_000_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}