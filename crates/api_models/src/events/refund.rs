use common_utils::events::{ApiEventMetric, ApiEventsType};

use crate::refunds::{
    self, RefundAggregateResponse, RefundListFilters, RefundListMetaData, RefundListRequest,
    RefundListResponse,
};
#[cfg(feature = "v1")]
use crate::refunds::{
    RefundManualUpdateRequest, RefundRequest, RefundUpdateRequest, RefundsRetrieveRequest,
};

#[cfg(feature = "v1")]
impl ApiEventMetric for RefundRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        let payment_id = self.payment_id.clone();
        self.refund_id
            .clone()
            .map(|refund_id| ApiEventsType::Refund {
                payment_id: Some(payment_id),
                refund_id,
            })
    }
}

#[cfg(feature = "v1")]
impl ApiEventMetric for refunds::RefundResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Refund {
            payment_id: Some(self.payment_id.clone()),
            refund_id: self.refund_id.clone(),
        })
    }
}

#[cfg(feature = "v2")]
impl ApiEventMetric for refunds::RefundResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Refund {
            payment_id: Some(self.payment_id.clone()),
            refund_id: self.id.clone(),
        })
    }
}

#[cfg(feature = "v1")]
impl ApiEventMetric for RefundsRetrieveRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Refund {
            payment_id: None,
            refund_id: self.refund_id.clone(),
        })
    }
}

#[cfg(feature = "v2")]
impl ApiEventMetric for refunds::RefundsRetrieveRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Refund {
            payment_id: None,
            refund_id: self.refund_id.clone(),
        })
    }
}

#[cfg(feature = "v1")]
impl ApiEventMetric for RefundUpdateRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Refund {
            payment_id: None,
            refund_id: self.refund_id.clone(),
        })
    }
}

#[cfg(feature = "v1")]
impl ApiEventMetric for RefundManualUpdateRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::Refund {
            payment_id: None,
            refund_id: self.refund_id.clone(),
        })
    }
}

impl ApiEventMetric for RefundListRequest {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::ResourceListAPI)
    }
}

impl ApiEventMetric for RefundListResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::ResourceListAPI)
    }
}

impl ApiEventMetric for RefundAggregateResponse {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::ResourceListAPI)
    }
}

impl ApiEventMetric for RefundListMetaData {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::ResourceListAPI)
    }
}

impl ApiEventMetric for RefundListFilters {
    fn get_api_event_type(&self) -> Option<ApiEventsType> {
        Some(ApiEventsType::ResourceListAPI)
    }
}
