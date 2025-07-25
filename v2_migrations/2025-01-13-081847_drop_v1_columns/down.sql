ALTER TABLE ORGANIZATION
    ADD COLUMN org_id VARCHAR(32),
    ADD COLUMN org_name TEXT;

ALTER TABLE merchant_account
    ADD COLUMN merchant_id VARCHAR(64),
    ADD COLUMN return_url VARCHAR(255),
    ADD COLUMN enable_payment_response_hash BOOLEAN DEFAULT FALSE,
    ADD COLUMN payment_response_hash_key VARCHAR(255),
    ADD COLUMN redirect_to_merchant_with_http_post BOOLEAN DEFAULT FALSE,
    ADD COLUMN sub_merchants_enabled BOOLEAN DEFAULT FALSE,
    ADD COLUMN parent_merchant_id VARCHAR(64),
    ADD COLUMN locker_id VARCHAR(64),
    ADD COLUMN intent_fulfillment_time BIGINT,
    ADD COLUMN default_profile VARCHAR(64),
    ADD COLUMN payment_link_config JSONB NULL,
    ADD COLUMN pm_collect_link_config JSONB NULL,
    ADD COLUMN is_recon_enabled BOOLEAN,
    ADD COLUMN webhook_details JSON NULL,
    ADD COLUMN routing_algorithm JSON,
    ADD COLUMN frm_routing_algorithm JSONB,
    ADD COLUMN payout_routing_algorithm JSONB;

-- The default value is for temporary purpose only
ALTER TABLE merchant_account
    ADD COLUMN primary_business_details JSON;

ALTER TABLE business_profile
    ADD COLUMN profile_id VARCHAR(64),
    ADD COLUMN routing_algorithm JSON DEFAULT NULL,
    ADD COLUMN intent_fulfillment_time BIGINT DEFAULT NULL,
    ADD COLUMN frm_routing_algorithm JSONB DEFAULT NULL,
    ADD COLUMN payout_routing_algorithm JSONB DEFAULT NULL;

ALTER TABLE merchant_connector_account
    ADD COLUMN IF NOT EXISTS business_country "CountryAlpha2",
    ADD COLUMN IF NOT EXISTS business_label VARCHAR(255),
    ADD COLUMN IF NOT EXISTS business_sub_label VARCHAR(64),
    ADD COLUMN IF NOT EXISTS test_mode BOOLEAN,
    ADD COLUMN IF NOT EXISTS frm_configs jsonb,
    ADD COLUMN IF NOT EXISTS merchant_connector_id VARCHAR(128);

ALTER TABLE customers
    ADD COLUMN customer_id VARCHAR(64),
    ADD COLUMN address_id VARCHAR(64);

ALTER TABLE payment_intent
    ADD COLUMN IF NOT EXISTS payment_id VARCHAR(64),
    ADD COLUMN connector_id VARCHAR(64),
    ADD COLUMN shipping_address_id VARCHAR(64),
    ADD COLUMN billing_address_id VARCHAR(64),
    ADD COLUMN shipping_details BYTEA,
    ADD COLUMN billing_details BYTEA,
    ADD COLUMN statement_descriptor_suffix VARCHAR(255),
    ADD COLUMN business_country "CountryAlpha2",
    ADD COLUMN business_label VARCHAR(64),
    ADD COLUMN incremental_authorization_allowed BOOLEAN,
    ADD COLUMN merchant_decision VARCHAR(64),
    ADD COLUMN fingerprint_id VARCHAR(64),
    ADD COLUMN statement_descriptor_name VARCHAR(255),
    ADD COLUMN amount_to_capture BIGINT,
    ADD COLUMN off_session BOOLEAN,
    ADD COLUMN payment_confirm_source "PaymentSource",
    ADD COLUMN merchant_order_reference_id VARCHAR(255),
    ADD COLUMN is_payment_processor_token_flow BOOLEAN,
    ADD COLUMN charges jsonb;

ALTER TABLE payment_attempt
    ADD COLUMN IF NOT EXISTS attempt_id VARCHAR(64),
    ADD COLUMN amount bigint,
    ADD COLUMN currency "Currency",
    ADD COLUMN save_to_locker BOOLEAN,
    ADD COLUMN offer_amount bigint,
    ADD COLUMN payment_method VARCHAR,
    ADD COLUMN connector_transaction_id VARCHAR(128),
    ADD COLUMN connector_transaction_data VARCHAR(512),
    ADD COLUMN processor_transaction_data text,
    ADD COLUMN capture_method "CaptureMethod",
    ADD COLUMN capture_on TIMESTAMP,
    ADD COLUMN mandate_id VARCHAR(64),
    ADD COLUMN payment_method_type VARCHAR(64),
    ADD COLUMN business_sub_label VARCHAR(64),
    ADD COLUMN mandate_details JSONB,
    ADD COLUMN mandate_data JSONB,
    ADD COLUMN tax_amount bigint,
    ADD COLUMN straight_through_algorithm JSONB,
    ADD COLUMN confirm BOOLEAN,
    ADD COLUMN authentication_data JSON,
    ADD COLUMN payment_method_billing_address_id VARCHAR(64),
    ADD COLUMN connector_mandate_detail JSONB,
    ADD COLUMN charge_id VARCHAR(64);

-- Create the index which was dropped because of dropping the column
CREATE INDEX payment_attempt_connector_transaction_id_merchant_id_index ON payment_attempt (connector_transaction_id, merchant_id);

CREATE UNIQUE INDEX payment_attempt_payment_id_merchant_id_attempt_id_index ON payment_attempt (payment_id, merchant_id, attempt_id);

-- Payment Methods
CREATE TYPE "PaymentMethodIssuerCode" AS ENUM (
    'jp_hdfc',
    'jp_icici',
    'jp_googlepay',
    'jp_applepay',
    'jp_phonepe',
    'jp_wechat',
    'jp_sofort',
    'jp_giropay',
    'jp_sepa',
    'jp_bacs'
);

ALTER TABLE payment_methods
    ADD COLUMN IF NOT EXISTS payment_method_id VARCHAR(64),
    ADD COLUMN IF NOT EXISTS accepted_currency "Currency" [ ],
    ADD COLUMN IF NOT EXISTS scheme VARCHAR(32),
    ADD COLUMN IF NOT EXISTS token VARCHAR(128),
    ADD COLUMN IF NOT EXISTS cardholder_name VARCHAR(255),
    ADD COLUMN IF NOT EXISTS issuer_name VARCHAR(64),
    ADD COLUMN IF NOT EXISTS issuer_country VARCHAR(64),
    ADD COLUMN IF NOT EXISTS payer_country TEXT [ ],
    ADD COLUMN IF NOT EXISTS is_stored BOOLEAN,
    ADD COLUMN IF NOT EXISTS direct_debit_token VARCHAR(128),
    ADD COLUMN IF NOT EXISTS swift_code VARCHAR(32),
    ADD COLUMN IF NOT EXISTS payment_method_issuer VARCHAR(128),
    ADD COLUMN IF NOT EXISTS metadata JSON,
    ADD COLUMN IF NOT EXISTS payment_method VARCHAR,
    ADD COLUMN IF NOT EXISTS payment_method_type VARCHAR(64),
    ADD COLUMN IF NOT EXISTS payment_method_issuer_code "PaymentMethodIssuerCode";

ALTER TABLE refund ADD COLUMN connector_refund_data VARCHAR(512),
    ADD COLUMN connector_transaction_data VARCHAR(512);

ALTER TABLE captures ADD COLUMN connector_capture_data VARCHAR(512);

ALTER TABLE refund 
    ADD COLUMN IF NOT EXISTS internal_reference_id VARCHAR(64),
    ADD COLUMN IF NOT EXISTS refund_id VARCHAR(64),
    ADD COLUMN IF NOT EXISTS merchant_connector_id VARCHAR(64);

ALTER TABLE payment_attempt ADD COLUMN IF NOT EXISTS connector_request_reference_id VARCHAR(255);