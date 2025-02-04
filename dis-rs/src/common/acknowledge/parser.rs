use crate::common::acknowledge::model::Acknowledge;
use crate::common::model::PduBody;
use crate::common::parser::entity_id;
use crate::enumerations::{AcknowledgeFlag, ResponseFlag};
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

pub(crate) fn acknowledge_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, acknowledge_flag) = be_u16(input)?;
    let (input, response_flag) = be_u16(input)?;
    let (input, request_id) = be_u32(input)?;

    let acknowledge_flag = AcknowledgeFlag::from(acknowledge_flag);
    let response_flag = ResponseFlag::from(response_flag);

    let body = Acknowledge::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_acknowledge_flag(acknowledge_flag)
        .with_response_flag(response_flag)
        .with_request_id(request_id)
        .build();

    Ok((input, body.into_pdu_body()))
}
