use crate::records::model::{
    CdisRecord, EntityCoordinateVector, EntityId, LinearVelocity, UnitsMass, UnitsMeters,
};
use crate::types::model::{VarInt, UVINT32};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use dis_rs::enumerations::CollisionType;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Collision {
    pub units: CollisionUnits,
    pub issuing_entity_id: EntityId,
    pub colliding_entity_id: EntityId,
    pub event_id: EntityId,
    pub collision_type: CollisionType,
    pub velocity: LinearVelocity,
    pub mass: UVINT32,
    pub location: EntityCoordinateVector,
}

impl BodyProperties for Collision {
    type FieldsPresent = ();
    type FieldsPresentOutput = u8;
    const FIELDS_PRESENT_LENGTH: usize = 0;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        0
    }

    fn body_length_bits(&self) -> usize {
        const CONST_BIT_SIZE: usize = 3; // Units flags + CollisionType
        CONST_BIT_SIZE
            + self.issuing_entity_id.record_length()
            + self.colliding_entity_id.record_length()
            + self.event_id.record_length()
            + self.velocity.record_length()
            + self.mass.record_length()
            + self.location.record_length()
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Collision(self)
    }
}

impl CdisInteraction for Collision {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.issuing_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.colliding_entity_id)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct CollisionUnits {
    // C-DIS v1.0 describes location_entity_coordinates to be in Centimeters and Dekameters,
    // which is conflicting with Table 49 and the implementation in the Detonation PDU
    // The choice here is to follow Table 49 - use the UnitsMeters enum
    pub location_entity_coordinates: UnitsMeters,
    pub mass: UnitsMass,
}

impl From<u8> for CollisionUnits {
    fn from(value: u8) -> Self {
        pub const LOCATION_IN_ENTITY_COORDINATES_BIT: u8 = 0x02;
        pub const MASS_BIT: u8 = 0x01;
        Self {
            location_entity_coordinates: UnitsMeters::from(
                (value & LOCATION_IN_ENTITY_COORDINATES_BIT) >> 1,
            ),
            mass: UnitsMass::from(value & MASS_BIT),
        }
    }
}
