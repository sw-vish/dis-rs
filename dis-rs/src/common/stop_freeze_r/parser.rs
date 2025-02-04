use crate::common::model::PduBody;
use crate::common::parser::{clock_time, entity_id};
use crate::enumerations::{RequiredReliabilityService, StopFreezeFrozenBehavior, StopFreezeReason};
use crate::stop_freeze_r::model::StopFreezeR;
use nom::number::complete::{be_u32, be_u8};
use nom::IResult;

pub(crate) fn stop_freeze_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, real_world_time) = clock_time(input)?;
    let (input, reason) = be_u8(input)?;
    let (input, behavior) = be_u8(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let (input, _padding) = be_u8(input)?;
    let (input, request_id) = be_u32(input)?;

    let reason = StopFreezeReason::from(reason);
    let behavior = StopFreezeFrozenBehavior::from(behavior);
    let required_reliability_service =
        RequiredReliabilityService::from(required_reliability_service);

    let body = StopFreezeR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_real_world_time(real_world_time)
        .with_reason(reason)
        .with_frozen_behavior(behavior)
        .with_request_id(request_id)
        .with_required_reliability_service(required_reliability_service)
        .build();

    Ok((input, body.into_pdu_body()))
}
