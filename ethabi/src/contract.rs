use rstd::fmt;
use rstd::collections::btree_map::BTreeMap;
use rstd::collections::btree_map::Values;
use rstd::iter::Flatten;
#[cfg(feature = "std")]
use serde::{Deserialize, Deserializer};
#[cfg(feature = "std")]
use serde::de::{Visitor, SeqAccess};

#[cfg(feature = "std")]
use serde_json;
#[cfg(feature = "std")]
use std::io;

use operation::Operation;
use {errors, ErrorKind, Event, Constructor, Function};

use rstd::prelude::*;
use rstd::vec::Vec;

#[cfg(not(feature = "std"))]
use alloc::string::String;

/// API building calls to contracts ABI.
#[derive(Clone, Debug, PartialEq)]
pub struct Contract {
	/// Contract constructor.
	pub constructor: Option<Constructor>,
	/// Contract functions.
	pub functions: BTreeMap<String, Function>,
	/// Contract events, maps signature to event.
	pub events: BTreeMap<String, Vec<Event>>,
	/// Contract has fallback function.
	pub fallback: bool,
}

#[cfg(feature = "std")]
impl<'a> Deserialize<'a> for Contract {
	fn deserialize<D>(deserializer: D) -> Result<Contract, D::Error> where D: Deserializer<'a> {
		deserializer.deserialize_any(ContractVisitor)
	}
}

#[cfg(feature = "std")]
struct ContractVisitor;

#[cfg(feature = "std")]
impl<'a> Visitor<'a> for ContractVisitor {
	type Value = Contract;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("valid abi spec file")
	}

	fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error> where A: SeqAccess<'a> {
		let mut result = Contract {
			constructor: None,
			functions: BTreeMap::default(),
			events: BTreeMap::default(),
			fallback: false,
		};

		while let Some(operation) = seq.next_element()? {
			match operation {
				Operation::Constructor(constructor) => {
					result.constructor = Some(constructor);
				},
				Operation::Function(func) => {
					result.functions.insert(func.name.clone(), func);
				},
				Operation::Event(event) => {
					result.events.entry(event.name.clone()).or_default().push(event);
				},
				Operation::Fallback => {
					result.fallback = true;
				},
			}
		}

		Ok(result)
	}
}

impl Contract {
	#[cfg(feature = "std")]
	/// Loads contract from json.
	pub fn load<T: io::Read>(reader: T) -> errors::Result<Self> {
		serde_json::from_reader(reader).map_err(From::from)
	}

	/// Creates constructor call builder.
	pub fn constructor(&self) -> Option<&Constructor> {
		self.constructor.as_ref()
	}

	/// Creates function call builder.
	pub fn function(&self, name: &str) -> Result<&Function, &'static str> {
		self.functions.get(name).ok_or_else(|| "Invalid name")
	}

	/// Get the contract event named `name`, the first if there are multiple.
	pub fn event(&self, name: &str) -> Result<&Event, &'static str> {
		self.events.get(name).into_iter()
							.flatten()
							.next()
							.ok_or_else(|| "Invalid name")
	}

	/// Get all contract events named `name`.
	pub fn events_by_name(&self, name: &str) -> Result<&Vec<Event>, &'static str> {
		self.events.get(name)
					.ok_or_else(|| "Invalid name")
	}

	/// Iterate over all functions of the contract in arbitrary order.
	pub fn functions(&self) -> Functions {
		Functions(self.functions.values())
	}

	/// Iterate over all events of the contract in arbitrary order.
	pub fn events(&self) -> Events {
		Events(self.events.values().flatten())
	}

	/// Returns true if contract has fallback
	pub fn fallback(&self) -> bool {
		self.fallback
	}
}

/// Contract functions interator.
pub struct Functions<'a>(Values<'a, String, Function>);

impl<'a> Iterator for Functions<'a> {
	type Item = &'a Function;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}

/// Contract events interator.
pub struct Events<'a>(Flatten<Values<'a, String, Vec<Event>>>);

impl<'a> Iterator for Events<'a> {
	type Item = &'a Event;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}
