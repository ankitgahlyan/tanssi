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


//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-02-21, STEPS: `16`, REPEAT: `1`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_identity
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
// tmp/dancebox_weights/pallet_identity.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for SubstrateWeight<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 10_192_000 picoseconds.
		Weight::from_parts(16_753_551, 2626)
			// Standard Error: 314_791
			.saturating_add(Weight::from_parts(55_728, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6976 + r * (5 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 101_682_000 picoseconds.
		Weight::from_parts(158_197_196, 11003)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `11003 + s * (2589 ±0)`
		// Minimum execution time: 12_805_000 picoseconds.
		Weight::from_parts(20_630_322, 11003)
			// Standard Error: 67_198
			.saturating_add(Weight::from_parts(2_924_496, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `192 + p * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 12_136_000 picoseconds.
		Weight::from_parts(28_732_647, 11003)
			// Standard Error: 177_979
			.saturating_add(Weight::from_parts(1_181_256, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(_r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7068 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 47_076_000 picoseconds.
		Weight::from_parts(79_882_211, 11003)
			// Standard Error: 102_231
			.saturating_add(Weight::from_parts(873_851, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6966 + r * (57 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 68_016_000 picoseconds.
		Weight::from_parts(70_425_006, 11003)
			// Standard Error: 143_732
			.saturating_add(Weight::from_parts(113_080, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6997`
		//  Estimated: `11003`
		// Minimum execution time: 65_779_000 picoseconds.
		Weight::from_parts(71_257_168, 11003)
			// Standard Error: 1_062_034
			.saturating_add(Weight::from_parts(868_014, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(_r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 7_058_000 picoseconds.
		Weight::from_parts(23_947_254, 2626)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 7_720_000 picoseconds.
		Weight::from_parts(9_716_621, 2626)
			// Standard Error: 192_094
			.saturating_add(Weight::from_parts(496_928, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 7_414_000 picoseconds.
		Weight::from_parts(11_326_997, 2626)
			// Standard Error: 164_305
			.saturating_add(Weight::from_parts(232_948, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7044 + r * (57 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 98_684_000 picoseconds.
		Weight::from_parts(161_424_915, 11003)
			// Standard Error: 1_852_797
			.saturating_add(Weight::from_parts(222_210, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(_r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7275 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 80_681_000 picoseconds.
		Weight::from_parts(174_666_012, 11003)
			// Standard Error: 517_161
			.saturating_add(Weight::from_parts(1_672_500, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467 + s * (36 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 25_592_000 picoseconds.
		Weight::from_parts(30_480_321, 11003)
			// Standard Error: 33_335
			.saturating_add(Weight::from_parts(77_287, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `558 + s * (3 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 19_131_000 picoseconds.
		Weight::from_parts(20_426_091, 11003)
			// Standard Error: 76_890
			.saturating_add(Weight::from_parts(92_626, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `606 + s * (35 ±0)`
		//  Estimated: `11003`
		// Minimum execution time: 31_081_000 picoseconds.
		Weight::from_parts(28_454_816, 11003)
			// Standard Error: 173_027
			.saturating_add(Weight::from_parts(226_473, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(_s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `703 + s * (37 ±0)`
		//  Estimated: `6723`
		// Minimum execution time: 25_649_000 picoseconds.
		Weight::from_parts(32_262_903, 6723)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::AuthorityOf` (r:0 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_375_000 picoseconds.
		Weight::from_parts(18_375_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AuthorityOf` (r:1 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78`
		//  Estimated: `3517`
		// Minimum execution time: 18_550_000 picoseconds.
		Weight::from_parts(18_550_000, 3517)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AuthorityOf` (r:1 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 1]`.
	fn set_username_for(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `181`
		//  Estimated: `3593`
		// Minimum execution time: 54_351_000 picoseconds.
		Weight::from_parts(76_034_000, 3593)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameInfoOf` (r:0 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `115`
		//  Estimated: `3567`
		// Minimum execution time: 25_971_000 picoseconds.
		Weight::from_parts(25_971_000, 3567)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 1]`.
	fn remove_expired_approval(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `309`
		//  Estimated: `3593`
		// Minimum execution time: 24_728_000 picoseconds.
		Weight::from_parts(49_288_444, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:0)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:0 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `171`
		//  Estimated: `3563`
		// Minimum execution time: 16_955_000 picoseconds.
		Weight::from_parts(16_955_000, 3563)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:0)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UnbindingUsernames` (r:1 w:1)
	/// Proof: `Identity::UnbindingUsernames` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn unbind_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `235`
		//  Estimated: `3563`
		// Minimum execution time: 17_459_000 picoseconds.
		Weight::from_parts(17_459_000, 3563)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UnbindingUsernames` (r:1 w:1)
	/// Proof: `Identity::UnbindingUsernames` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `296`
		//  Estimated: `3563`
		// Minimum execution time: 33_213_000 picoseconds.
		Weight::from_parts(33_213_000, 3563)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:1 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:1 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UnbindingUsernames` (r:1 w:1)
	/// Proof: `Identity::UnbindingUsernames` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AuthorityOf` (r:1 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 1]`.
	fn kill_username(_p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `470`
		//  Estimated: `3593`
		// Minimum execution time: 18_511_000 picoseconds.
		Weight::from_parts(36_495_888, 3593)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:2 w:0)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:2 w:0)
	/// Storage: `Identity::AuthorityOf` (r:0 w:1)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn migration_v2_authority_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `6086`
		// Minimum execution time: 32_390_000 picoseconds.
		Weight::from_parts(32_390_000, 6086)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:2 w:0)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:2 w:0)
	/// Storage: `Identity::UsernameInfoOf` (r:0 w:1)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	fn migration_v2_username_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `158`
		//  Estimated: `6098`
		// Minimum execution time: 17_510_000 picoseconds.
		Weight::from_parts(17_510_000, 6098)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::IdentityOf` (r:2 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// Storage: `Identity::UsernameOf` (r:0 w:1)
	/// Proof: `Identity::UsernameOf` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn migration_v2_identity_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7061`
		//  Estimated: `21016`
		// Minimum execution time: 56_610_000 picoseconds.
		Weight::from_parts(56_610_000, 21016)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Identity::PendingUsernames` (r:2 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(102), added: 2577, mode: `MaxEncodedLen`)
	fn migration_v2_pending_username_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `200`
		//  Estimated: `6144`
		// Minimum execution time: 9_663_000 picoseconds.
		Weight::from_parts(9_663_000, 6144)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::AuthorityOf` (r:2 w:0)
	/// Proof: `Identity::AuthorityOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f99622d1423cdd16f5c33e2b531c34a53d` (r:1 w:1)
	fn migration_v2_cleanup_authority_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `287`
		//  Estimated: `6044`
		// Minimum execution time: 17_005_000 picoseconds.
		Weight::from_parts(17_005_000, 6044)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Identity::UsernameInfoOf` (r:2 w:0)
	/// Proof: `Identity::UsernameInfoOf` (`max_values`: None, `max_size`: Some(98), added: 2573, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x2aeddc77fe58c98d50bd37f1b90840f97c182fead9255863460affdd63116be3` (r:1 w:1)
	fn migration_v2_cleanup_username_step() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `289`
		//  Estimated: `6136`
		// Minimum execution time: 12_885_000 picoseconds.
		Weight::from_parts(12_885_000, 6136)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}