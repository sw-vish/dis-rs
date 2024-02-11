pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{EventType, PduType, VariableRecordType};
    use crate::common::model::{EntityId, Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};
    use crate::event_report_r::model::EventReportR;
    use crate::model::{FixedDatum, VariableDatum};

    #[test]
    fn event_report_r_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::EventReportR);
        let body = EventReportR::builder()
            .with_origination_id(EntityId::new(10,10,10))
            .with_receiving_id(EntityId::new(20,20,20))
            .with_event_type(EventType::MobilityDisabled)
            .with_fixed_datums(vec![FixedDatum::new(VariableRecordType::_7_62mm_quantity_24010, 100)])
            .with_variable_datums(vec![VariableDatum::new(VariableRecordType::Country_11130, vec![0x1, 0x2, 0x3])])
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