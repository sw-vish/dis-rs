use crate::common::model::{DescriptorRecord, EntityId, EventId, Location, PduBody, VectorF32};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::fire::builder::FireBuilder;

const FIRE_BODY_LENGTH: u16 = 28;

/// 5.4.3 Fire PDU
///
/// 7.3.2 Fire PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Fire {
    pub firing_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub entity_id: EntityId,
    pub event_id: EventId,
    pub fire_mission_index: u32,
    pub location_in_world: Location,
    pub descriptor: DescriptorRecord,
    pub velocity: VectorF32,
    pub range: f32,
}

impl Fire {
    #[must_use]
    pub fn builder() -> FireBuilder {
        FireBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> FireBuilder {
        FireBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Fire(self)
    }
}

impl BodyInfo for Fire {
    fn body_length(&self) -> u16 {
        FIRE_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Fire
    }
}

impl Interaction for Fire {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.firing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}
