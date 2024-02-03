pub mod parser;
pub mod model;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{EntityKind, FireTypeIndicator, PduType};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};
    use crate::fire::model::Fire;
    use crate::model::{DescriptorRecord, EntityType, VectorF32};
    use crate::v7::model::PduStatus;

    #[test]
    fn fire_internal_consistency() {
        let header = PduHeader::new_v7(1, PduType::Acknowledge)
            .with_pdu_status(PduStatus::default()
                .with_fire_type_indicator(FireTypeIndicator::Expendable));

        let body = Fire::builder()
            .with_descriptor(DescriptorRecord::new_expendable(EntityType::default().with_kind(EntityKind::Expendable)))
            .with_velocity(VectorF32::new(50.0, 60.0, 70.0))
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf);

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref _err) => {
                println!("{_err}");
                assert!(false);
            }
        }
    }
}