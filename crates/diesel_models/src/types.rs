#[cfg(feature = "v2")]
use common_enums::enums::PaymentConnectorTransmission;
#[cfg(feature = "v2")]
use common_utils::id_type;
use common_utils::{hashing::HashedString, pii, types::MinorUnit};
use diesel::{
    sql_types::{Json, Jsonb},
    AsExpression, FromSqlRow,
};
use masking::{Secret, WithType};
use serde::{self, Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Jsonb)]
pub struct OrderDetailsWithAmount {
    /// Name of the product that is being purchased
    pub product_name: String,
    /// The quantity of the product to be purchased
    pub quantity: u16,
    /// the amount per quantity of product
    pub amount: MinorUnit,
    // Does the order includes shipping
    pub requires_shipping: Option<bool>,
    /// The image URL of the product
    pub product_img_link: Option<String>,
    /// ID of the product that is being purchased
    pub product_id: Option<String>,
    /// Category of the product that is being purchased
    pub category: Option<String>,
    /// Sub category of the product that is being purchased
    pub sub_category: Option<String>,
    /// Brand of the product that is being purchased
    pub brand: Option<String>,
    /// Type of the product that is being purchased
    pub product_type: Option<common_enums::ProductType>,
    /// The tax code for the product
    pub product_tax_code: Option<String>,
    /// tax rate applicable to the product
    pub tax_rate: Option<f64>,
    /// total tax amount applicable to the product
    pub total_tax_amount: Option<MinorUnit>,
}

impl masking::SerializableSecret for OrderDetailsWithAmount {}

common_utils::impl_to_sql_from_sql_json!(OrderDetailsWithAmount);

#[cfg(feature = "v2")]
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Json)]
pub struct FeatureMetadata {
    /// Redirection response coming in request as metadata field only for redirection scenarios
    pub redirect_response: Option<RedirectResponse>,
    /// Additional tags to be used for global search
    pub search_tags: Option<Vec<HashedString<WithType>>>,
    /// Recurring payment details required for apple pay Merchant Token
    pub apple_pay_recurring_details: Option<ApplePayRecurringDetails>,
    /// revenue recovery data for payment intent
    pub payment_revenue_recovery_metadata: Option<PaymentRevenueRecoveryMetadata>,
}

#[cfg(feature = "v2")]
impl FeatureMetadata {
    pub fn get_payment_method_sub_type(&self) -> Option<common_enums::PaymentMethodType> {
        self.payment_revenue_recovery_metadata
            .as_ref()
            .map(|rrm| rrm.payment_method_subtype)
    }

    pub fn get_payment_method_type(&self) -> Option<common_enums::PaymentMethod> {
        self.payment_revenue_recovery_metadata
            .as_ref()
            .map(|recovery_metadata| recovery_metadata.payment_method_type)
    }

    pub fn get_billing_merchant_connector_account_id(
        &self,
    ) -> Option<id_type::MerchantConnectorAccountId> {
        self.payment_revenue_recovery_metadata
            .as_ref()
            .map(|recovery_metadata| recovery_metadata.billing_connector_id.clone())
    }

    // TODO: Check search_tags for relevant payment method type
    // TODO: Check redirect_response metadata if applicable
    // TODO: Check apple_pay_recurring_details metadata if applicable
}

#[cfg(feature = "v1")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Json)]
pub struct FeatureMetadata {
    /// Redirection response coming in request as metadata field only for redirection scenarios
    pub redirect_response: Option<RedirectResponse>,
    /// Additional tags to be used for global search
    pub search_tags: Option<Vec<HashedString<WithType>>>,
    /// Recurring payment details required for apple pay Merchant Token
    pub apple_pay_recurring_details: Option<ApplePayRecurringDetails>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Json)]
pub struct ApplePayRecurringDetails {
    /// A description of the recurring payment that Apple Pay displays to the user in the payment sheet
    pub payment_description: String,
    /// The regular billing cycle for the recurring payment, including start and end dates, an interval, and an interval count
    pub regular_billing: ApplePayRegularBillingDetails,
    /// A localized billing agreement that the payment sheet displays to the user before the user authorizes the payment
    pub billing_agreement: Option<String>,
    /// A URL to a web page where the user can update or delete the payment method for the recurring payment
    pub management_url: common_utils::types::Url,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Json)]
pub struct ApplePayRegularBillingDetails {
    /// The label that Apple Pay displays to the user in the payment sheet with the recurring details
    pub label: String,
    /// The date of the first payment
    #[serde(with = "common_utils::custom_serde::iso8601::option")]
    pub recurring_payment_start_date: Option<time::PrimitiveDateTime>,
    /// The date of the final payment
    #[serde(with = "common_utils::custom_serde::iso8601::option")]
    pub recurring_payment_end_date: Option<time::PrimitiveDateTime>,
    /// The amount of time — in calendar units, such as day, month, or year — that represents a fraction of the total payment interval
    pub recurring_payment_interval_unit: Option<RecurringPaymentIntervalUnit>,
    /// The number of interval units that make up the total payment interval
    pub recurring_payment_interval_count: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromSqlRow, AsExpression)]
#[diesel(sql_type = Json)]
#[serde(rename_all = "snake_case")]
pub enum RecurringPaymentIntervalUnit {
    Year,
    Month,
    Day,
    Hour,
    Minute,
}

common_utils::impl_to_sql_from_sql_json!(ApplePayRecurringDetails);
common_utils::impl_to_sql_from_sql_json!(ApplePayRegularBillingDetails);
common_utils::impl_to_sql_from_sql_json!(RecurringPaymentIntervalUnit);

common_utils::impl_to_sql_from_sql_json!(FeatureMetadata);

#[derive(Default, Debug, Eq, PartialEq, Deserialize, Serialize, Clone)]
pub struct RedirectResponse {
    pub param: Option<Secret<String>>,
    pub json_payload: Option<pii::SecretSerdeValue>,
}
impl masking::SerializableSecret for RedirectResponse {}
common_utils::impl_to_sql_from_sql_json!(RedirectResponse);

#[cfg(feature = "v2")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PaymentRevenueRecoveryMetadata {
    /// Total number of billing connector + recovery retries for a payment intent.
    pub total_retry_count: u16,
    /// Flag for the payment connector's call
    pub payment_connector_transmission: PaymentConnectorTransmission,
    /// Billing Connector Id to update the invoices
    pub billing_connector_id: id_type::MerchantConnectorAccountId,
    /// Payment Connector Id to retry the payments
    pub active_attempt_payment_connector_id: id_type::MerchantConnectorAccountId,
    /// Billing Connector Payment Details
    pub billing_connector_payment_details: BillingConnectorPaymentDetails,
    ///Payment Method Type
    pub payment_method_type: common_enums::enums::PaymentMethod,
    /// PaymentMethod Subtype
    pub payment_method_subtype: common_enums::enums::PaymentMethodType,
    /// The name of the payment connector through which the payment attempt was made.
    pub connector: common_enums::connector_enums::Connector,
    /// Time at which next invoice will be created
    pub invoice_next_billing_time: Option<time::PrimitiveDateTime>,
    /// Time at which invoice started
    pub invoice_billing_started_at_time: Option<time::PrimitiveDateTime>,
    /// Extra Payment Method Details that are needed to be stored
    pub billing_connector_payment_method_details: Option<BillingConnectorPaymentMethodDetails>,
    /// First Payment Attempt Payment Gateway Error Code
    pub first_payment_attempt_pg_error_code: Option<String>,
    /// First Payment Attempt Network Error Code
    pub first_payment_attempt_network_decline_code: Option<String>,
    /// First Payment Attempt Network Advice Code
    pub first_payment_attempt_network_advice_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[cfg(feature = "v2")]
pub struct BillingConnectorPaymentDetails {
    /// Payment Processor Token to process the Revenue Recovery Payment
    pub payment_processor_token: String,
    /// Billing Connector's Customer Id
    pub connector_customer_id: String,
}

#[cfg(feature = "v2")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum BillingConnectorPaymentMethodDetails {
    Card(BillingConnectorAdditionalCardInfo),
}

#[cfg(feature = "v2")]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct BillingConnectorAdditionalCardInfo {
    /// Card Network
    pub card_network: Option<common_enums::enums::CardNetwork>,
    /// Card Issuer
    pub card_issuer: Option<String>,
}
