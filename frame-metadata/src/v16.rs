// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[cfg(feature = "decode")]
use codec::Decode;
#[cfg(feature = "serde_full")]
use serde::Serialize;

use super::{RuntimeMetadataPrefixed, META_RESERVED};
use codec::Encode;
use scale_info::{
	form::{Form, MetaForm, PortableForm},
	prelude::{vec::Vec},
	IntoPortable, MetaType, PortableRegistry, Registry,
};

pub use super::v15::{
	PalletCallMetadata, PalletConstantMetadata, PalletErrorMetadata, PalletEventMetadata, StorageEntryModifier,
	StorageHasher, ExtrinsicMetadata, RuntimeApiMetadata, OuterEnums, CustomMetadata,
};

/// Latest runtime metadata
pub type RuntimeMetadataLastVersion = RuntimeMetadataV16;

impl From<RuntimeMetadataLastVersion> for super::RuntimeMetadataPrefixed {
	fn from(metadata: RuntimeMetadataLastVersion) -> RuntimeMetadataPrefixed {
		RuntimeMetadataPrefixed(META_RESERVED, super::RuntimeMetadata::V16(metadata))
	}
}

/// The metadata of a runtime.
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
pub struct RuntimeMetadataV16 {
	/// Type registry containing all types used in the metadata.
	pub types: PortableRegistry,
	/// Metadata of all the pallets.
	pub pallets: Vec<PalletMetadata<PortableForm>>,
	/// Metadata of the extrinsic.
	pub extrinsic: ExtrinsicMetadata<PortableForm>,
	/// The type of the `Runtime`.
	pub ty: <PortableForm as Form>::Type,
	/// Metadata of the Runtime API.
	pub apis: Vec<RuntimeApiMetadata<PortableForm>>,
	/// The outer enums types as found in the runtime.
	pub outer_enums: OuterEnums<PortableForm>,
	/// Allows users to add custom types to the metadata.
	pub custom: CustomMetadata<PortableForm>,
}

impl RuntimeMetadataV16 {
	/// Create a new instance of [`RuntimeMetadataV16`].
	pub fn new(
		pallets: Vec<PalletMetadata>,
		extrinsic: ExtrinsicMetadata,
		runtime_type: MetaType,
		apis: Vec<RuntimeApiMetadata>,
		outer_enums: OuterEnums,
		custom: CustomMetadata,
	) -> Self {
		let mut registry = Registry::new();
		let pallets = registry.map_into_portable(pallets);
		let extrinsic = extrinsic.into_portable(&mut registry);
		let ty = registry.register_type(&runtime_type);
		let apis = registry.map_into_portable(apis);
		let outer_enums = outer_enums.into_portable(&mut registry);
		let custom = custom.into_portable(&mut registry);

		Self {
			types: registry.into(),
			pallets,
			extrinsic,
			ty,
			apis,
			outer_enums,
			custom,
		}
	}
}

/// All metadata about an runtime pallet.
// Copied from V14
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletMetadata<T: Form = MetaForm> {
	/// Pallet name.
	pub name: T::String,
	/// Pallet storage metadata.
	pub storage: Option<PalletStorageMetadata<T>>,
	/// Pallet calls metadata.
	pub calls: Option<PalletCallMetadata<T>>,
	/// Pallet event metadata.
	pub event: Option<PalletEventMetadata<T>>,
	/// Pallet constants metadata.
	pub constants: Vec<PalletConstantMetadata<T>>,
	/// Pallet error metadata.
	pub error: Option<PalletErrorMetadata<T>>,
	/// Define the index of the pallet, this index will be used for the encoding of pallet event,
	/// call and origin variants.
	pub index: u8,
}

impl IntoPortable for PalletMetadata {
	type Output = PalletMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletMetadata {
			name: self.name.into_portable(registry),
			storage: self.storage.map(|storage| storage.into_portable(registry)),
			calls: self.calls.map(|calls| calls.into_portable(registry)),
			event: self.event.map(|event| event.into_portable(registry)),
			constants: registry.map_into_portable(self.constants),
			error: self.error.map(|error| error.into_portable(registry)),
			index: self.index,
		}
	}
}

/// All metadata of the pallet's storage.
// Copied from V14
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct PalletStorageMetadata<T: Form = MetaForm> {
	/// The common prefix used by all storage entries.
	pub prefix: T::String,
	/// Metadata for all storage entries.
	pub entries: Vec<StorageEntryMetadata<T>>,
}

impl IntoPortable for PalletStorageMetadata {
	type Output = PalletStorageMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		PalletStorageMetadata {
			prefix: self.prefix.into_portable(registry),
			entries: registry.map_into_portable(self.entries),
		}
	}
}

/// Metadata about one storage entry.
// Copied from V14
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub struct StorageEntryMetadata<T: Form = MetaForm> {
	/// Variable name of the storage entry.
	pub name: T::String,
	/// An `Option` modifier of that storage entry.
	pub modifier: StorageEntryModifier,
	/// Type of the value stored in the entry.
	pub ty: StorageEntryType<T>,
	/// Default value (SCALE encoded).
	pub default: Vec<u8>,
	/// Storage entry documentation.
	pub docs: Vec<T::String>,
}

impl IntoPortable for StorageEntryMetadata {
	type Output = StorageEntryMetadata<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		StorageEntryMetadata {
			name: self.name.into_portable(registry),
			modifier: self.modifier,
			ty: self.ty.into_portable(registry),
			default: self.default,
			docs: registry.map_into_portable(self.docs),
		}
	}
}

/// A type of storage value.
// Changed from V14
#[derive(Clone, PartialEq, Eq, Encode, Debug)]
#[cfg_attr(feature = "decode", derive(Decode))]
#[cfg_attr(feature = "serde_full", derive(Serialize))]
#[cfg_attr(
	feature = "serde_full",
	serde(bound(serialize = "T::Type: Serialize, T::String: Serialize"))
)]
pub enum StorageEntryType<T: Form = MetaForm> {
	/// Plain storage entry (just the value).
	Plain(T::Type),
	/// A storage map.
	Map {
		/// One or more hashers, should be one hasher per key element.
		hashers: Vec<StorageHasher>,
		/// The type of the key, can be a tuple with elements for each of the hashers.
		key: T::Type,
		/// The type of the value.
		value: T::Type,
	},
	/// A storage (multi) map that stores data in a paged manner.
	PagedMap {
		/// One or more hashers, should be one hasher per key element.
		hashers: Vec<StorageHasher>,
		/// The type of the key, can be a tuple with elements for each of the hashers.
		key: T::Type,
		/// The type of the value.
		value: T::Type,
		/// The maximal length of a single page in bytes.
		page_size: u32,
	}
}

impl IntoPortable for StorageEntryType {
	type Output = StorageEntryType<PortableForm>;

	fn into_portable(self, registry: &mut Registry) -> Self::Output {
		match self {
			Self::Plain(plain) => StorageEntryType::Plain(registry.register_type(&plain)),
			Self::Map {
				hashers,
				key,
				value,
			} => StorageEntryType::Map {
				hashers,
				key: registry.register_type(&key),
				value: registry.register_type(&value),
			},
			Self::PagedMap {
				hashers,
				key,
				value,
				page_size,
			} => StorageEntryType::PagedMap {
				hashers,
				key: registry.register_type(&key),
				value: registry.register_type(&value),
				page_size,
			},
		}
	}
}
