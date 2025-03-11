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


//! Autogenerated weights for pallet_stream_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 43.0.0
//! DATE: 2025-03-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `pop-os`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dancelight-dev"), DB CACHE: 1024

// Executed Command:
// target/release/tanssi-relay
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_stream_payment
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
// tmp/dancelight_weights/pallet_stream_payment.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_stream_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_stream_payment::WeightInfo for SubstrateWeight<T> {
	/// Storage: `StreamPayment::NextStreamId` (r:1 w:1)
	/// Proof: `StreamPayment::NextStreamId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(157), added: 2632, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::Streams` (r:0 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	fn open_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3622`
		// Minimum execution time: 77_414_000 picoseconds.
		Weight::from_parts(91_463_000, 3622)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(157), added: 2632, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithTarget` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithTarget` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// Storage: `StreamPayment::LookupStreamsWithSource` (r:0 w:1)
	/// Proof: `StreamPayment::LookupStreamsWithSource` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	fn close_stream() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `685`
		//  Estimated: `6196`
		// Minimum execution time: 129_513_000 picoseconds.
		Weight::from_parts(202_789_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(157), added: 2632, mode: `MaxEncodedLen`)
	fn perform_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `685`
		//  Estimated: `6196`
		// Minimum execution time: 77_852_000 picoseconds.
		Weight::from_parts(89_332_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(157), added: 2632, mode: `MaxEncodedLen`)
	fn request_change_immediate() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `685`
		//  Estimated: `6196`
		// Minimum execution time: 109_684_000 picoseconds.
		Weight::from_parts(128_650_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	fn request_change_delayed() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3742`
		// Minimum execution time: 12_088_000 picoseconds.
		Weight::from_parts(13_481_000, 3742)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(157), added: 2632, mode: `MaxEncodedLen`)
	fn accept_requested_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `755`
		//  Estimated: `6196`
		// Minimum execution time: 103_410_000 picoseconds.
		Weight::from_parts(111_871_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	fn cancel_change_request() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `366`
		//  Estimated: `3742`
		// Minimum execution time: 9_427_000 picoseconds.
		Weight::from_parts(10_990_000, 3742)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `StreamPayment::Streams` (r:1 w:1)
	/// Proof: `StreamPayment::Streams` (`max_values`: None, `max_size`: Some(277), added: 2752, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:1 w:1)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(157), added: 2632, mode: `MaxEncodedLen`)
	fn immediately_change_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `685`
		//  Estimated: `6196`
		// Minimum execution time: 102_651_000 picoseconds.
		Weight::from_parts(116_104_000, 6196)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}