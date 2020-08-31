// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! This module contains the cost schedule and supporting code that constructs a
//! sane default schedule from a `WeightInfo` implementation.

use crate::{Trait, WeightInfo};

#[cfg(feature = "std")]
use serde::{Serialize, Deserialize};
use frame_support::weights::Weight;
use sp_runtime::RuntimeDebug;
use sp_std::marker::PhantomData;
use codec::{Encode, Decode};

/// How many API calls are executed in a single batch. The reason for increasing the amount
/// of API calls in batches (per benchmark component increase) is so that the linear regression
/// has an easier time determining the contribution of that component.
pub const API_BENCHMARK_BATCH_SIZE: u32 = 100;

macro_rules! api_overhead {
	($name:tt) => {
		(T::WeightInfo::$name(1).saturating_sub(T::WeightInfo::$name(0)))
			/ Weight::from(API_BENCHMARK_BATCH_SIZE)
	}
}

/// Definition of the cost schedule and other parameterizations for wasm vm.
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug)]
pub struct Schedule<T: Trait> {
	/// The type parameter is used in the default implementation.
	pub phantom: PhantomData<T>,

	/// Version of the schedule.
	pub version: u32,

	/// Weight cost of a growing memory by single page.
	pub op_cost_grow_mem: Weight,

	/// Weight cost of a regular operation.
	pub op_cost_regular: Weight,

	/// Weight costs of calling `seal_caller`
	pub api_cost_caller: Weight,

	/// Weight costs of calling `seal_address`
	pub api_cost_address: Weight,

	/// Weight costs of calling `seal_gas_left`
	pub api_cost_gas_left: Weight,

	/// Weight costs of calling `seal_balance`
	pub api_cost_balance: Weight,

	/// Weight costs of calling `seal_value_transferred`
	pub api_cost_value_transferred: Weight,

	/// Weight costs of calling `seal_minimum_balance`
	pub api_cost_minimum_balance: Weight,

	/// Weight costs of calling `seal_tombstone_deposit`
	pub api_cost_tombstone_deposit: Weight,

	/// Weight costs of calling `seal_rent_allowance`
	pub api_cost_rent_allowance: Weight,

	/// Weight costs of calling `seal_block_number`
	pub api_cost_block_number: Weight,

	/// Weight costs of calling `seal_weight_to_fee`
	pub api_cost_weight_to_fee: Weight,

	/// Weight costs of calling `gas`
	pub api_cost_gas: Weight,

	/// Whether the `seal_println` function is allowed to be used contracts.
	/// MUST only be enabled for `dev` chains, NOT for production chains
	pub enable_println: bool,

	/// The maximum number of topics supported by an event.
	pub max_event_topics: u32,

	/// Maximum allowed stack height.
	///
	/// See https://wiki.parity.io/WebAssembly-StackHeight to find out
	/// how the stack frame cost is calculated.
	pub max_stack_height: u32,

	/// Maximum number of memory pages allowed for a contract.
	pub max_memory_pages: u32,

	/// Maximum allowed size of a declared table.
	pub max_table_size: u32,

	/// The maximum length of a subject used for PRNG generation.
	pub max_subject_len: u32,

	/// The maximum length of a contract code in bytes. This limit applies to the uninstrumented
	/// and pristine form of the code as supplied to `put_code`.
	pub max_code_size: u32,
}

/// 500 (2 instructions per nano second on 2GHZ) * 1000x slowdown through wasmi
/// This is a wild guess and should be viewed as a rough estimation.
/// Proper benchmarks are needed before this value and its derivatives can be used in production.
const WASM_INSTRUCTION_COST: Weight = 500_000;

impl<T: Trait> Default for Schedule<T> {
	fn default() -> Self {
		Self {
			phantom: PhantomData,
			version: 0,
			op_cost_grow_mem: WASM_INSTRUCTION_COST,
			op_cost_regular: WASM_INSTRUCTION_COST,
			api_cost_caller: api_overhead!(seal_caller),
			api_cost_address: api_overhead!(seal_address),
			api_cost_gas_left: api_overhead!(seal_gas_left),
			api_cost_balance: api_overhead!(seal_balance),
			api_cost_value_transferred: api_overhead!(seal_value_transferred),
			api_cost_minimum_balance: api_overhead!(seal_minimum_balance),
			api_cost_tombstone_deposit: api_overhead!(seal_tombstone_deposit),
			api_cost_rent_allowance: api_overhead!(seal_rent_allowance),
			api_cost_block_number: api_overhead!(seal_block_number),
			api_cost_weight_to_fee: api_overhead!(seal_weight_to_fee),
			api_cost_gas: api_overhead!(seal_gas),
			enable_println: false,
			max_event_topics: 4,
			max_stack_height: 64 * 1024,
			max_memory_pages: 16,
			max_table_size: 16 * 1024,
			max_subject_len: 32,
			max_code_size: 512 * 1024,
		}
	}
}
