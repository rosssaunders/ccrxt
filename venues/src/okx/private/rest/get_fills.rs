use serde::{Deserialize, Serialize};
use crate::okx::{EndpointType, InstrumentType, OrderSide, RestResult};
use super::{RestClient, common::OkxApiResponse};

/// Request to get recent fills
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFillsRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,
    
    /// Underlying
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uly: Option<String>,
    
    /// Instrument family
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_family: Option<String>,
    
    /// Instrument ID, e.g. "BTC-USDT"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_id: Option<String>,
    
    /// Order ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ord_id: Option<String>,
    
    /// Pagination of data to return records earlier than the requested bill ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    
    /// Pagination of data to return records newer than the requested bill ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    
    /// Number of results per request. The maximum is 100; the default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Fill details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    /// Instrument type
    pub inst_type: String,
    
    /// Instrument ID
    pub inst_id: String,
    
    /// Trade ID
    pub trade_id: String,
    
    /// Order ID
    pub ord_id: String,
    
    /// Client Order ID
    pub cl_ord_id: Option<String>,
    
    /// Bill ID
    pub bill_id: String,
    
    /// Order tag
    pub tag: Option<String>,
    
    /// Fill price
    pub fill_px: String,
    
    /// Fill quantity
    pub fill_sz: String,
    
    /// Order side
    pub side: OrderSide,
    
    /// Position side
    pub pos_side: Option<String>,
    
    /// Execution type
    /// T: taker, M: maker
    pub exec_type: String,
    
    /// Fee currency
    pub fee_ccy: String,
    
    /// Fee amount
    pub fee: String,
    
    /// Fill time
    pub ts: String,
    
    /// Underlying
    pub uly: Option<String>,
    
    /// Currency
    pub ccy: Option<String>,
    
    /// Category
    pub category: Option<String>,
    
    /// Profit and loss
    pub pnl: Option<String>,
    
    /// PnL currency
    pub pnl_ccy: Option<String>,
    
    /// Rebate currency
    pub rebate_ccy: Option<String>,
    
    /// Rebate amount
    pub rebate: Option<String>,
    
    /// Source of the trade
    pub source: Option<String>,
    
    /// Index price
    pub idx_px: Option<String>,
    
    /// Trade time
    pub fill_time: String,
    
    /// Fill profit and loss
    pub fill_pnl: Option<String>,
    
    /// Fill profit and loss currency
    pub fill_pnl_ccy: Option<String>,
    
    /// Fill mark price
    pub fill_mark_px: Option<String>,
    
    /// Fill volatility
    pub fill_vol: Option<String>,
    
    /// Fill forward price
    pub fill_fwd_px: Option<String>,
    
    /// Fill mark volatility
    pub fill_mark_vol: Option<String>,
}

impl RestClient {
    /// Get recent fills
    ///
    /// # Arguments
    /// * `request` - The get fills request
    ///
    /// # Returns
    /// A result containing the recent fills or an error
    pub async fn get_fills(
        &self,
        request: &GetFillsRequest,
    ) -> RestResult<OkxApiResponse<Fill>> {
        self.send_request(
            "api/v5/trade/fills",
            reqwest::Method::GET,
            Some(request),
            EndpointType::PrivateTrading,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fills_request_serialization() {
        let request = GetFillsRequest {
            inst_type: Some(InstrumentType::Spot),
            uly: None,
            inst_family: None,
            inst_id: Some("BTC-USDT".to_string()),
            ord_id: None,
            after: None,
            before: None,
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("instId=BTC-USDT"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_fills_minimal_request() {
        let request = GetFillsRequest {
            inst_type: None,
            uly: None,
            inst_family: None,
            inst_id: None,
            ord_id: None,
            after: None,
            before: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_fill_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instType": "SPOT",
                    "instId": "BTC-USDT",
                    "tradeId": "12345678",
                    "ordId": "312269865356374016",
                    "clOrdId": "my_order_123",
                    "billId": "987654321",
                    "tag": "",
                    "fillPx": "50000.0",
                    "fillSz": "0.01",
                    "side": "buy",
                    "posSide": "",
                    "execType": "T",
                    "feeCcy": "USDT",
                    "fee": "-0.5",
                    "ts": "1597026383085",
                    "uly": "",
                    "ccy": "",
                    "category": "normal",
                    "pnl": "0",
                    "pnlCcy": "",
                    "rebateCcy": "",
                    "rebate": "0",
                    "source": "",
                    "idxPx": "",
                    "fillTime": "1597026383085",
                    "fillPnl": "",
                    "fillPnlCcy": "",
                    "fillMarkPx": "",
                    "fillVol": "",
                    "fillFwdPx": "",
                    "fillMarkVol": ""
                }
            ]
        }"#;

        let response: OkxApiResponse<Fill> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);
        
        let fill = &response.data[0];
        assert_eq!(fill.inst_id, "BTC-USDT");
        assert_eq!(fill.trade_id, "12345678");
        assert_eq!(fill.ord_id, "312269865356374016");
        assert_eq!(fill.fill_px, "50000.0");
        assert_eq!(fill.fill_sz, "0.01");
        assert_eq!(fill.side, OrderSide::Buy);
        assert_eq!(fill.exec_type, "T");
        assert_eq!(fill.fee_ccy, "USDT");
        assert_eq!(fill.fee, "-0.5");
    }
}