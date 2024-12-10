pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{EntityId, Pdu, PduHeader, SupplyQuantity};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{EntityKind, PduType};
    use crate::model::EntityType;
    use crate::resupply_offer::model::ResupplyOffer;
    use bytes::BytesMut;

    #[test]
    fn resupply_offer_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::ResupplyOffer);

        let body = ResupplyOffer::builder()
            .with_requesting_id(EntityId::new(1, 1, 2))
            .with_servicing_id(EntityId::new(9, 1, 1))
            .with_supply(
                SupplyQuantity::default()
                    .with_supply_type(EntityType::default().with_kind(EntityKind::Supply))
                    .with_quantity(678.0),
            )
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf).unwrap();

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
