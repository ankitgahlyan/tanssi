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


//! Autogenerated weights for pallet_collator_assignment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-02-04, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dancelight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_collator_assignment
// --extrinsic
// *
// --chain=dancelight-dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/dancelight_weights/pallet_collator_assignment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_collator_assignment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collator_assignment::WeightInfo for SubstrateWeight<T> {
	/// Storage: `TanssiCollatorAssignment::Randomness` (r:1 w:1)
	/// Proof: `TanssiCollatorAssignment::Randomness` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Session::CurrentIndex` (r:1 w:0)
	/// Proof: `Session::CurrentIndex` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorConfiguration::PendingConfigs` (r:1 w:0)
	/// Proof: `CollatorConfiguration::PendingConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `CollatorConfiguration::ActiveConfig` (r:1 w:0)
	/// Proof: `CollatorConfiguration::ActiveConfig` (`max_values`: Some(1), `max_size`: Some(52), added: 547, mode: `MaxEncodedLen`)
	/// Storage: `Configuration::PendingConfigs` (r:1 w:0)
	/// Proof: `Configuration::PendingConfigs` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ContainerRegistrar::PendingParaIds` (r:1 w:0)
	/// Proof: `ContainerRegistrar::PendingParaIds` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ContainerRegistrar::RegisteredParaIds` (r:1 w:0)
	/// Proof: `ContainerRegistrar::RegisteredParaIds` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `ContainerRegistrar::ParathreadParams` (r:20 w:0)
	/// Proof: `ContainerRegistrar::ParathreadParams` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `TanssiCollatorAssignment::PendingCollatorContainerChain` (r:1 w:1)
	/// Proof: `TanssiCollatorAssignment::PendingCollatorContainerChain` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `TanssiCollatorAssignment::CollatorContainerChain` (r:1 w:1)
	/// Proof: `TanssiCollatorAssignment::CollatorContainerChain` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ServicesPayment::MaxTip` (r:20 w:0)
	/// Proof: `ServicesPayment::MaxTip` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::CollatorAssignmentCredits` (r:20 w:2)
	/// Proof: `ServicesPayment::CollatorAssignmentCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `ServicesPayment::BlockProductionCredits` (r:20 w:0)
	/// Proof: `ServicesPayment::BlockProductionCredits` (`max_values`: None, `max_size`: Some(24), added: 2499, mode: `MaxEncodedLen`)
	/// Storage: `TanssiCollatorAssignment::CollatorFullnessRatio` (r:0 w:1)
	/// Proof: `TanssiCollatorAssignment::CollatorFullnessRatio` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 200]`.
	/// The range of component `y` is `[1, 20]`.
	fn new_session(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4498 + y * (385 ±0)`
		//  Estimated: `5984 + y * (2511 ±0)`
		// Minimum execution time: 144_534_000 picoseconds.
		Weight::from_parts(87_850_373, 5984)
			// Standard Error: 3_855
			.saturating_add(Weight::from_parts(261_519, 0).saturating_mul(x.into()))
			// Standard Error: 39_254
			.saturating_add(Weight::from_parts(23_839_282, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().writes(6_u64))
			.saturating_add(Weight::from_parts(0, 2511).saturating_mul(y.into()))
	}
}