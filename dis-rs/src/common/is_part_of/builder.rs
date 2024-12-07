use crate::is_part_of::model::{IsPartOf, NamedLocationId, Relationship};
use crate::model::{EntityId, VectorF32};

pub struct IsPartOfBuilder(IsPartOf);

impl IsPartOfBuilder {
    #[must_use]
    pub fn new() -> Self {
        IsPartOfBuilder(IsPartOf::default())
    }

    #[must_use]
    pub fn new_from_body(body: IsPartOf) -> Self {
        IsPartOfBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> IsPartOf {
        self.0
    }

    #[must_use]
    pub fn with_originating_simulation_id(mut self, originating_simulation_id: EntityId) -> Self {
        self.0.originating_simulation_id = originating_simulation_id;
        self
    }

    #[must_use]
    pub fn with_receiving_entity_id(mut self, receiving_entity_id: EntityId) -> Self {
        self.0.receiving_entity_id = receiving_entity_id;
        self
    }

    #[must_use]
    pub fn with_relationship(mut self, relationship: Relationship) -> Self {
        self.0.relationship = relationship;
        self
    }

    #[must_use]
    pub fn with_part_location(mut self, part_location: VectorF32) -> Self {
        self.0.part_location = part_location;
        self
    }

    #[must_use]
    pub fn with_named_location_id(mut self, named_location_id: NamedLocationId) -> Self {
        self.0.named_location_id = named_location_id;
        self
    }
}
