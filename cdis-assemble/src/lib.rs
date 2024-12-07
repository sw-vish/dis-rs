pub mod constants;
pub mod records;
pub mod types;

pub mod acknowledge;
pub mod action_request;
pub mod action_response;
pub mod collision;
pub mod comment;
pub mod create_entity;
pub mod data;
pub mod data_query;
pub mod designator;
pub mod detonation;
pub mod electromagnetic_emission;
pub mod entity_state;
pub mod event_report;
pub mod fire;
pub mod iff;
pub mod receiver;
pub mod remove_entity;
pub mod set_data;
pub mod signal;
pub mod start_resume;
pub mod stop_freeze;
pub mod transmitter;
pub mod unsupported;

pub mod codec;
pub(crate) mod parsing;
pub(crate) mod writing;

use dis_rs::enumerations::PduType;
use dis_rs::model::TimeStamp;
use thiserror::Error;

use crate::acknowledge::model::Acknowledge;
use crate::action_request::model::ActionRequest;
use crate::action_response::model::ActionResponse;
use crate::collision::model::Collision;
use crate::comment::model::Comment;
use crate::create_entity::model::CreateEntity;
use crate::data::model::Data;
use crate::data_query::model::DataQuery;
use crate::designator::model::Designator;
use crate::detonation::model::Detonation;
use crate::electromagnetic_emission::model::ElectromagneticEmission;
use crate::entity_state::model::EntityState;
use crate::event_report::model::EventReport;
use crate::fire::model::Fire;
use crate::iff::model::Iff;
use crate::receiver::model::Receiver;
use crate::records::model::{CdisHeader, CdisRecord, EntityId};
use crate::remove_entity::model::RemoveEntity;
use crate::set_data::model::SetData;
use crate::signal::model::Signal;
use crate::start_resume::model::StartResume;
use crate::stop_freeze::model::StopFreeze;
use crate::transmitter::model::Transmitter;
use crate::unsupported::Unsupported;
pub use parsing::parse;
pub use writing::create_bit_buffer;
pub use writing::BitBuffer;
pub use writing::SerializeCdisPdu;

pub trait BodyProperties {
    type FieldsPresent;
    type FieldsPresentOutput;
    const FIELDS_PRESENT_LENGTH: usize;
    fn fields_present_field(&self) -> Self::FieldsPresentOutput;

    fn body_length_bits(&self) -> usize;

    fn fields_present_length(&self) -> usize {
        Self::FIELDS_PRESENT_LENGTH
    }

    fn into_cdis_body(self) -> CdisBody;
}

pub trait CdisInteraction {
    fn originator(&self) -> Option<&EntityId>;
    fn receiver(&self) -> Option<&EntityId>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct CdisPdu {
    pub header: CdisHeader,
    pub body: CdisBody,
}

impl CdisPdu {
    pub fn finalize_from_parts(
        header: CdisHeader,
        body: CdisBody,
        time_stamp: Option<impl Into<TimeStamp>>,
    ) -> Self {
        let time_stamp: TimeStamp = if let Some(time_stamp) = time_stamp {
            time_stamp.into()
        } else {
            header.timestamp
        };
        Self {
            header: CdisHeader {
                timestamp: time_stamp,
                length: (header.record_length() + body.body_length()) as u16,
                ..header
            },
            body,
        }
    }

    /// Calculates the on-wire length of the C-DIS PDU in bits.
    #[must_use]
    pub fn pdu_length(&self) -> usize {
        self.header.record_length() + self.body.body_length()
    }
}

impl CdisInteraction for CdisPdu {
    fn originator(&self) -> Option<&EntityId> {
        self.body.originator()
    }

    fn receiver(&self) -> Option<&EntityId> {
        self.body.receiver()
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, PartialEq)]
pub enum CdisBody {
    Unsupported(Unsupported),
    EntityState(EntityState),
    Fire(Fire),
    Detonation(Detonation),
    Collision(Collision),
    CreateEntity(CreateEntity),
    RemoveEntity(RemoveEntity),
    StartResume(StartResume),
    StopFreeze(StopFreeze),
    Acknowledge(Acknowledge),
    ActionRequest(ActionRequest),
    ActionResponse(ActionResponse),
    DataQuery(DataQuery),
    SetData(SetData),
    Data(Data),
    EventReport(EventReport),
    Comment(Comment),
    ElectromagneticEmission(ElectromagneticEmission),
    Designator(Designator),
    Transmitter(Transmitter),
    Signal(Signal),
    Receiver(Receiver),
    Iff(Iff),
}

impl CdisBody {
    #[must_use]
    pub fn body_length(&self) -> usize {
        match self {
            CdisBody::Unsupported(_) => 0,
            CdisBody::EntityState(body) => body.body_length_bits(),
            CdisBody::Fire(body) => body.body_length_bits(),
            CdisBody::Detonation(body) => body.body_length_bits(),
            CdisBody::Collision(body) => body.body_length_bits(),
            CdisBody::CreateEntity(body) => body.body_length_bits(),
            CdisBody::RemoveEntity(body) => body.body_length_bits(),
            CdisBody::StartResume(body) => body.body_length_bits(),
            CdisBody::StopFreeze(body) => body.body_length_bits(),
            CdisBody::Acknowledge(body) => body.body_length_bits(),
            CdisBody::ActionRequest(body) => body.body_length_bits(),
            CdisBody::ActionResponse(body) => body.body_length_bits(),
            CdisBody::DataQuery(body) => body.body_length_bits(),
            CdisBody::SetData(body) => body.body_length_bits(),
            CdisBody::Data(body) => body.body_length_bits(),
            CdisBody::EventReport(body) => body.body_length_bits(),
            CdisBody::Comment(body) => body.body_length_bits(),
            CdisBody::ElectromagneticEmission(body) => body.body_length_bits(),
            CdisBody::Designator(body) => body.body_length_bits(),
            CdisBody::Transmitter(body) => body.body_length_bits(),
            CdisBody::Signal(body) => body.body_length_bits(),
            CdisBody::Receiver(body) => body.body_length_bits(),
            CdisBody::Iff(body) => body.body_length_bits(),
        }
    }
}

impl CdisInteraction for CdisBody {
    fn originator(&self) -> Option<&EntityId> {
        match self {
            CdisBody::Unsupported(_) => None,
            CdisBody::EntityState(body) => body.originator(),
            CdisBody::Fire(body) => body.originator(),
            CdisBody::Detonation(body) => body.originator(),
            CdisBody::Collision(body) => body.originator(),
            CdisBody::CreateEntity(body) => body.originator(),
            CdisBody::RemoveEntity(body) => body.originator(),
            CdisBody::StartResume(body) => body.originator(),
            CdisBody::StopFreeze(body) => body.originator(),
            CdisBody::Acknowledge(body) => body.originator(),
            CdisBody::ActionRequest(body) => body.originator(),
            CdisBody::ActionResponse(body) => body.originator(),
            CdisBody::DataQuery(body) => body.originator(),
            CdisBody::SetData(body) => body.originator(),
            CdisBody::Data(body) => body.originator(),
            CdisBody::EventReport(body) => body.originator(),
            CdisBody::Comment(body) => body.originator(),
            CdisBody::ElectromagneticEmission(body) => body.originator(),
            CdisBody::Designator(body) => body.originator(),
            CdisBody::Transmitter(body) => body.originator(),
            CdisBody::Signal(body) => body.originator(),
            CdisBody::Receiver(body) => body.originator(),
            CdisBody::Iff(body) => body.originator(),
        }
    }

    fn receiver(&self) -> Option<&EntityId> {
        match self {
            CdisBody::Unsupported(_) => None,
            CdisBody::EntityState(body) => body.receiver(),
            CdisBody::Fire(body) => body.receiver(),
            CdisBody::Detonation(body) => body.receiver(),
            CdisBody::Collision(body) => body.receiver(),
            CdisBody::CreateEntity(body) => body.receiver(),
            CdisBody::RemoveEntity(body) => body.receiver(),
            CdisBody::StartResume(body) => body.receiver(),
            CdisBody::StopFreeze(body) => body.receiver(),
            CdisBody::Acknowledge(body) => body.receiver(),
            CdisBody::ActionRequest(body) => body.receiver(),
            CdisBody::ActionResponse(body) => body.receiver(),
            CdisBody::DataQuery(body) => body.receiver(),
            CdisBody::SetData(body) => body.receiver(),
            CdisBody::Data(body) => body.receiver(),
            CdisBody::EventReport(body) => body.receiver(),
            CdisBody::Comment(body) => body.receiver(),
            CdisBody::ElectromagneticEmission(body) => body.receiver(),
            CdisBody::Designator(body) => body.receiver(),
            CdisBody::Transmitter(body) => body.receiver(),
            CdisBody::Signal(body) => body.receiver(),
            CdisBody::Receiver(body) => body.receiver(),
            CdisBody::Iff(body) => body.receiver(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum CdisError {
    #[error("{0}")]
    ParseError(String), // the parsing of a CDIS PDU resulted in an error
    #[error(
        "The buffer does not contain enough bytes for a valid C-DIS header. {0} bits available."
    )]
    InsufficientHeaderLength(u16), // the input was too small to contain a valid CDIS header; (u16 found)
    #[error("C-DIS PDU has insufficient length. Expected {0} bits, found {1} bits.")]
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid CDIS PDU based on the header and parsing; (u16 expected, u16 found)
    #[error("C-DIS PDU is larger than size of the buffer for serialisation. Needs {0} bits, available {1} bits.")]
    InsufficientBufferSize(u16, usize), // the buffer for serialisation has insufficient capacity to hold the provided CDIS PDU; (u16 PDU size, usize available capacity)
    #[error("Encountered a C-DIS PDU of an unsupported type: {0}.")]
    UnsupportedPdu(u8), // encountered a CDIS PDU of an unsupported type; (u8 PduType found)
}

/// Trait that indicates whether a PDU is supported in the C-DIS standard
pub trait Supported {
    /// Returns true when a PDUs having a certain `PduType` is supported by the C-DIS standard, false otherwise.
    fn is_supported(&self) -> bool;
}

impl Supported for PduType {
    fn is_supported(&self) -> bool {
        matches!(
            self,
            PduType::EntityState
                | PduType::Fire
                | PduType::Detonation
                | PduType::Collision
                | PduType::CreateEntity
                | PduType::RemoveEntity
                | PduType::StartResume
                | PduType::StopFreeze
                | PduType::Acknowledge
                | PduType::ActionRequest
                | PduType::ActionResponse
                | PduType::DataQuery
                | PduType::SetData
                | PduType::Data
                | PduType::EventReport
                | PduType::Comment
                | PduType::ElectromagneticEmission
                | PduType::Designator
                | PduType::Transmitter
                | PduType::Signal
                | PduType::Receiver
                | PduType::IFF
        )
    }
}

/// Trait that indicates whether a PDU is implemented in C-DIS
pub trait Implemented {
    /// Returns true when the library implements PDUs having a certain `PduType`, false otherwise.
    fn is_implemented(&self) -> bool;
}

impl Implemented for PduType {
    /// A `PduType` is properly implemented by the C-DIS library when:
    /// - There is a model for the pdu body
    /// - `CdisBody` enum is adapted, including the trait implementation for `CdisInteraction` and method `body_length(..)`
    /// - There is a parser, and it is called in function `crate::parsing::cdis_body(..)`
    /// - There is a serializer, and it is called in the `SerializeCdisPdu` trait impl for `CdisBody` in `crate::writing`.
    /// - The codec implementations are present, and are called in `crate::codec` in the `CdisBody::encode` and `CdisBody::decode` implementations.
    fn is_implemented(&self) -> bool {
        matches!(
            self,
            PduType::EntityState
                | PduType::Fire
                | PduType::Detonation
                | PduType::Collision
                | PduType::CreateEntity
                | PduType::RemoveEntity
                | PduType::StartResume
                | PduType::StopFreeze
                | PduType::Acknowledge
                | PduType::ActionRequest
                | PduType::ActionResponse
                | PduType::DataQuery
                | PduType::SetData
                | PduType::Data
                | PduType::EventReport
                | PduType::Comment
                | PduType::ElectromagneticEmission
                | PduType::Designator
                | PduType::Transmitter
                | PduType::Signal
                | PduType::Receiver
                | PduType::IFF
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{Implemented, Supported};
    use dis_rs::enumerations::PduType;

    #[test]
    fn ensure_supported_pdus() {
        assert!(PduType::EntityState.is_supported());
        assert!(PduType::Fire.is_supported());
        assert!(PduType::Detonation.is_supported());
        assert!(PduType::Collision.is_supported());
        assert!(PduType::CreateEntity.is_supported());
        assert!(PduType::RemoveEntity.is_supported());
        assert!(PduType::StartResume.is_supported());
        assert!(PduType::StopFreeze.is_supported());
        assert!(PduType::Acknowledge.is_supported());
        assert!(PduType::ActionRequest.is_supported());
        assert!(PduType::ActionResponse.is_supported());
        assert!(PduType::DataQuery.is_supported());
        assert!(PduType::SetData.is_supported());
        assert!(PduType::Data.is_supported());
        assert!(PduType::EventReport.is_supported());
        assert!(PduType::Comment.is_supported());
        assert!(PduType::ElectromagneticEmission.is_supported());
        assert!(PduType::Designator.is_supported());
        assert!(PduType::Transmitter.is_supported());
        assert!(PduType::Signal.is_supported());
        assert!(PduType::Receiver.is_supported());
        assert!(PduType::IFF.is_supported());

        assert!(!PduType::Other.is_supported());
        assert!(!PduType::ServiceRequest.is_supported());
        assert!(!PduType::ResupplyOffer.is_supported());
        assert!(!PduType::ResupplyReceived.is_supported());
        assert!(!PduType::ResupplyCancel.is_supported());
        assert!(!PduType::RepairComplete.is_supported());
        assert!(!PduType::RepairResponse.is_supported());
        assert!(!PduType::UnderwaterAcoustic.is_supported());
        assert!(!PduType::SupplementalEmissionEntityState.is_supported());
        assert!(!PduType::IntercomSignal.is_supported());
        assert!(!PduType::IntercomControl.is_supported());
        assert!(!PduType::AggregateState.is_supported());
        assert!(!PduType::IsGroupOf.is_supported());
        assert!(!PduType::TransferOwnership.is_supported());
        assert!(!PduType::IsPartOf.is_supported());
        assert!(!PduType::MinefieldState.is_supported());
        assert!(!PduType::MinefieldQuery.is_supported());
        assert!(!PduType::MinefieldData.is_supported());
        assert!(!PduType::MinefieldResponseNACK.is_supported());
        assert!(!PduType::EnvironmentalProcess.is_supported());
        assert!(!PduType::GriddedData.is_supported());
        assert!(!PduType::PointObjectState.is_supported());
        assert!(!PduType::LinearObjectState.is_supported());
        assert!(!PduType::ArealObjectState.is_supported());
        assert!(!PduType::TSPI.is_supported());
        assert!(!PduType::Appearance.is_supported());
        assert!(!PduType::ArticulatedParts.is_supported());
        assert!(!PduType::LEFire.is_supported());
        assert!(!PduType::LEDetonation.is_supported());
        assert!(!PduType::CreateEntityR.is_supported());
        assert!(!PduType::RemoveEntityR.is_supported());
        assert!(!PduType::StartResumeR.is_supported());
        assert!(!PduType::StopFreezeR.is_supported());
        assert!(!PduType::AcknowledgeR.is_supported());
        assert!(!PduType::ActionRequestR.is_supported());
        assert!(!PduType::ActionResponseR.is_supported());
        assert!(!PduType::DataQueryR.is_supported());
        assert!(!PduType::SetDataR.is_supported());
        assert!(!PduType::DataR.is_supported());
        assert!(!PduType::EventReportR.is_supported());
        assert!(!PduType::CommentR.is_supported());
        assert!(!PduType::RecordR.is_supported());
        assert!(!PduType::SetRecordR.is_supported());
        assert!(!PduType::RecordQueryR.is_supported());
        assert!(!PduType::CollisionElastic.is_supported());
        assert!(!PduType::EntityStateUpdate.is_supported());
        assert!(!PduType::DirectedEnergyFire.is_supported());
        assert!(!PduType::EntityDamageStatus.is_supported());
        assert!(!PduType::InformationOperationsAction.is_supported());
        assert!(!PduType::InformationOperationsReport.is_supported());
        assert!(!PduType::Attribute.is_supported());
        assert!(!PduType::Unspecified(0).is_supported());
    }

    #[test]
    fn validate_implemented_pdus() {
        assert!(PduType::EntityState.is_implemented());
        assert!(PduType::Fire.is_implemented());
        assert!(PduType::Detonation.is_implemented());
        assert!(PduType::Collision.is_implemented());
        assert!(PduType::CreateEntity.is_implemented());
        assert!(PduType::RemoveEntity.is_implemented());
        assert!(PduType::StartResume.is_implemented());
        assert!(PduType::StopFreeze.is_implemented());
        assert!(PduType::Acknowledge.is_implemented());
        assert!(PduType::ActionRequest.is_implemented());
        assert!(PduType::ActionResponse.is_implemented());
        assert!(PduType::DataQuery.is_implemented());
        assert!(PduType::SetData.is_implemented());
        assert!(PduType::Data.is_implemented());
        assert!(PduType::EventReport.is_implemented());
        assert!(PduType::Comment.is_implemented());
        assert!(PduType::ElectromagneticEmission.is_implemented());
        assert!(PduType::Designator.is_implemented());
        assert!(PduType::Transmitter.is_implemented());
        assert!(PduType::Signal.is_implemented());
        assert!(PduType::Receiver.is_implemented());
        assert!(PduType::IFF.is_implemented());
    }
}
