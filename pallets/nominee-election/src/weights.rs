// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::all)]

use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_nominee_election.
pub trait WeightInfo {
    fn set_validators() -> Weight;
}

/// Weights for pallet_nominee_election using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_validators() -> Weight {
        10_000 as Weight
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn set_validators() -> Weight {
        10_000 as Weight
    }
}
