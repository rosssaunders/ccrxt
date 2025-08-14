use serde::{Deserialize, Serialize};

use super::RestClient;
use crate::okx::{EndpointType, InstrumentType, RestResult};

const ACCOUNT_BILLS_ENDPOINT: &str = "api/v5/account/bills";

/// Request to get account bills
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBillsRequest {
    /// Instrument type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inst_type: Option<InstrumentType>,

    /// Currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<String>,

    /// Margin mode
    /// "isolated", "cross"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mgn_mode: Option<String>,

    /// Contract type
    /// "linear", "inverse"
    /// Only applicable to FUTURES/SWAP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ct_type: Option<String>,

    /// Bill type
    /// "1": Transfer, "2": Trade, "3": Delivery, "4": Auto token conversion, "5": Liquidation, "6": Margin transfer
    /// "7": Interest deduction, "8": Funding fee, "9": ADL, "10": Clawback, "11": System token conversion
    /// "12": Strategy transfer, "13": DDH, "14": Block trade, "15": Quick Margin, "18": Profit sharing expenses
    /// "19": Profit sharing refund, "20": Profit sharing, "21": Option premium, "22": Option exercise
    /// "23": Option automatic exercise, "24": Option delivery, "25": Option settlement, "100": Partial liquidation close
    /// "101": Partial liquidation open, "102": Liquidation fees, "103": Insurance fund fees, "104": Auto-deleveraging penalty fees
    /// "105": Liquidation spread compensation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,

    /// Bill subtype
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,

    /// Pagination of data to return records earlier than the requested billId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// Pagination of data to return records newer than the requested billId
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,

    /// Filter with a begin timestamp. Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<String>,

    /// Filter with an end timestamp. Unix timestamp format in milliseconds, e.g. 1597026383085
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    /// Number of results per request. Maximum is 100. Default is 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

/// Bill details
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bill {
    /// Instrument ID
    pub inst_id: String,

    /// Instrument type
    pub inst_type: String,

    /// Currency
    pub ccy: String,

    /// Margin mode
    pub mgn_mode: String,

    /// Bill ID
    pub bill_id: String,

    /// Order ID
    pub ord_id: Option<String>,

    /// Trade ID
    pub trade_id: Option<String>,

    /// Client Order ID
    pub cl_ord_id: Option<String>,

    /// Bill type
    pub r#type: String,

    /// Bill subtype
    pub sub_type: String,

    /// Timestamp
    pub ts: String,

    /// Balance change amount
    pub bal_chg: String,

    /// Position change amount
    pub pos_bal_chg: String,

    /// Balance
    pub bal: String,

    /// Position balance
    pub pos_bal: String,

    /// Size
    pub sz: String,

    /// Price
    pub px: String,

    /// From
    pub from: Option<String>,

    /// To
    pub to: Option<String>,

    /// Profit and loss
    pub pnl: Option<String>,

    /// Fee
    pub fee: Option<String>,

    /// Interest
    pub interest: Option<String>,

    /// Notes
    pub notes: Option<String>,

    /// Fill time
    pub fill_time: Option<String>,

    /// Trade time
    pub trade_time: Option<String>,

    /// Execution type
    pub exec_type: Option<String>,
}

impl RestClient {
    /// Get account bills
    ///
    /// [docs](https://www.okx.com/docs-v5/en/#trading-account-rest-api-get-bills-details-last-7-days)
    ///
    /// # Arguments
    /// * `request` - The get bills request
    ///
    /// # Returns
    /// A result containing the bills or an error
    pub async fn get_bills(&self, request: &GetBillsRequest) -> RestResult<Bill> {
        self.send_get_request(
            ACCOUNT_BILLS_ENDPOINT,
            Some(request),
            EndpointType::PrivateAccount,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::okx::response::OkxApiResponse;

    #[test]
    fn test_get_bills_request_serialization() {
        let request = GetBillsRequest {
            inst_type: Some(InstrumentType::Spot),
            ccy: Some("BTC".to_string()),
            mgn_mode: Some("cross".to_string()),
            ct_type: None,
            r#type: Some("2".to_string()),
            sub_type: None,
            after: None,
            before: None,
            begin: Some("1597026383085".to_string()),
            end: Some("1597026383086".to_string()),
            limit: Some("50".to_string()),
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert!(serialized.contains("instType=SPOT"));
        assert!(serialized.contains("ccy=BTC"));
        assert!(serialized.contains("mgnMode=cross"));
        assert!(serialized.contains("type=2"));
        assert!(serialized.contains("begin=1597026383085"));
        assert!(serialized.contains("end=1597026383086"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_get_bills_minimal_request() {
        let request = GetBillsRequest {
            inst_type: None,
            ccy: None,
            mgn_mode: None,
            ct_type: None,
            r#type: None,
            sub_type: None,
            after: None,
            before: None,
            begin: None,
            end: None,
            limit: None,
        };

        let serialized = serde_urlencoded::to_string(&request).unwrap();
        assert_eq!(serialized, "");
    }

    #[test]
    fn test_bill_deserialization() {
        let response_json = r#"
        {
            "code": "0",
            "msg": "",
            "data": [
                {
                    "instId": "BTC-USDT",
                    "instType": "SPOT",
                    "ccy": "USDT",
                    "mgnMode": "cash",
                    "billId": "123456789",
                    "ordId": "987654321",
                    "tradeId": "111222333",
                    "clOrdId": "my_order_123",
                    "type": "2",
                    "subType": "1",
                    "ts": "1597026383085",
                    "balChg": "-100.5",
                    "posBalChg": "0",
                    "bal": "9899.5",
                    "posBal": "0",
                    "sz": "0.001",
                    "px": "50000",
                    "from": "",
                    "to": "",
                    "pnl": "0",
                    "fee": "-0.1",
                    "interest": "0",
                    "notes": "",
                    "fillTime": "1597026383085",
                    "tradeTime": "1597026383085",
                    "execType": "T"
                }
            ]
        }"#;

        let response: OkxApiResponse<Bill> = serde_json::from_str(response_json).unwrap();
        assert_eq!(response.code, "0");
        assert_eq!(response.data.len(), 1);

        let bill = &response.data[0];
        assert_eq!(bill.inst_id, "BTC-USDT");
        assert_eq!(bill.bill_id, "123456789");
        assert_eq!(bill.r#type, "2");
        assert_eq!(bill.bal_chg, "-100.5");
        assert_eq!(bill.px, "50000");
    }
}
