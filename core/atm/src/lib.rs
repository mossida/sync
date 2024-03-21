use std::sync::Arc;

use bus::Consumer;
use dashmap::DashSet;
use dbm::{
	relation::Relation,
	resource::{Base, Resource},
};

use err::Result;
use ractor::{factory::Factory, Actor};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use surrealdb::Action;
use svc::Service;
use trg::Trigger;
use worker::Worker;

mod worker;

const FACTORY: &str = "automator";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Automation {
	id: dbm::Id,
	name: String,
}

impl Base for Automation {
	const RESOURCE: &'static str = "automation";
}

impl Resource for Automation {
	fn id(&self) -> &dbm::Id {
		&self.id
	}
}

impl Relation<Trigger> for Automation {
	const RELATION: &'static str = "upon";
}

impl Relation<Automation> for Trigger {
	const RELATION: &'static str = "upon";
	const INVERTED: bool = true;
}

impl Relation<Service> for Automation {
	const RELATION: &'static str = "executes";
}

impl Automation {
	pub async fn trigger(self, event: bus::Event) -> Result<()> {
		// TODO: Optimize using advanced graphing queries
		let triggers: Vec<Trigger> = self.relationships().await?;
		let services: Vec<Service> = self.relationships().await?;

		let triggered: Vec<&Trigger> =
			triggers.par_iter().filter(|t| t.check(event.clone())).collect();

		if !triggered.is_empty() {
			services.par_iter().for_each(|s| s.run());
		}

		Ok(())
	}
}

pub async fn init() -> Result<(), err::Error> {
	// Get bus and fetch triggers
	let bus = bus::get();
	let automations = Automation::fetch_all().await?;
	let set = Arc::new(DashSet::from_iter(automations));

	let (factory, _) = Actor::spawn(
		Some(FACTORY.to_string()),
		Factory {
			worker_count: 24,
			..Default::default()
		},
		// TODO: Add correct triggers data structure
		Box::new(Worker {
			automations: set.clone(),
		}),
	)
	.await?;

	let stream = Automation::stream().await?;
	stream.consume(move |n| {
		let vector = set.clone();
		async move {
			let trigger = n.data;

			match n.action {
				Action::Create | Action::Update => vector.insert(trigger),
				Action::Delete => vector.remove(&trigger).is_some(),
				_ => false,
			};
		}
	});

	let subcription = bus.subscribe();
	subcription.to_factory(factory, |_| 0);

	Ok(())
}
