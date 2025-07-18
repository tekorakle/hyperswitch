#[cfg(feature = "v1")]
use std::collections::HashSet;

use async_bb8_diesel::AsyncRunQueryDsl;
#[cfg(feature = "v1")]
use diesel::Table;
use diesel::{
    associations::HasTable, debug_query, pg::Pg, BoolExpressionMethods, ExpressionMethods, QueryDsl,
};
use error_stack::{report, ResultExt};

use super::generics;
#[cfg(feature = "v1")]
use crate::schema::payment_attempt::dsl;
#[cfg(feature = "v2")]
use crate::schema_v2::payment_attempt::dsl;
#[cfg(feature = "v1")]
use crate::{enums::IntentStatus, payment_attempt::PaymentAttemptUpdate, PaymentIntent};
use crate::{
    enums::{self},
    errors::DatabaseError,
    payment_attempt::{PaymentAttempt, PaymentAttemptNew, PaymentAttemptUpdateInternal},
    query::generics::db_metrics,
    PgPooledConn, StorageResult,
};

impl PaymentAttemptNew {
    pub async fn insert(self, conn: &PgPooledConn) -> StorageResult<PaymentAttempt> {
        generics::generic_insert(conn, self).await
    }
}

impl PaymentAttempt {
    #[cfg(feature = "v1")]
    pub async fn update_with_attempt_id(
        self,
        conn: &PgPooledConn,
        payment_attempt: PaymentAttemptUpdate,
    ) -> StorageResult<Self> {
        match generics::generic_update_with_unique_predicate_get_result::<
            <Self as HasTable>::Table,
            _,
            _,
            _,
        >(
            conn,
            dsl::attempt_id
                .eq(self.attempt_id.to_owned())
                .and(dsl::merchant_id.eq(self.merchant_id.to_owned())),
            PaymentAttemptUpdateInternal::from(payment_attempt).populate_derived_fields(&self),
        )
        .await
        {
            Err(error) => match error.current_context() {
                DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            result => result,
        }
    }

    #[cfg(feature = "v2")]
    pub async fn update_with_attempt_id(
        self,
        conn: &PgPooledConn,
        payment_attempt: PaymentAttemptUpdateInternal,
    ) -> StorageResult<Self> {
        match generics::generic_update_with_unique_predicate_get_result::<
            <Self as HasTable>::Table,
            _,
            _,
            _,
        >(conn, dsl::id.eq(self.id.to_owned()), payment_attempt)
        .await
        {
            Err(error) => match error.current_context() {
                DatabaseError::NoFieldsToUpdate => Ok(self),
                _ => Err(error),
            },
            result => result,
        }
    }

    #[cfg(feature = "v1")]
    pub async fn find_optional_by_payment_id_merchant_id(
        conn: &PgPooledConn,
        payment_id: &common_utils::id_type::PaymentId,
        merchant_id: &common_utils::id_type::MerchantId,
    ) -> StorageResult<Option<Self>> {
        generics::generic_find_one_optional::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_id.eq(payment_id.to_owned())),
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn find_by_connector_transaction_id_payment_id_merchant_id(
        conn: &PgPooledConn,
        connector_transaction_id: &common_utils::types::ConnectorTransactionId,
        payment_id: &common_utils::id_type::PaymentId,
        merchant_id: &common_utils::id_type::MerchantId,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::connector_transaction_id
                .eq(connector_transaction_id.get_id().to_owned())
                .and(dsl::payment_id.eq(payment_id.to_owned()))
                .and(dsl::merchant_id.eq(merchant_id.to_owned())),
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn find_last_successful_attempt_by_payment_id_merchant_id(
        conn: &PgPooledConn,
        payment_id: &common_utils::id_type::PaymentId,
        merchant_id: &common_utils::id_type::MerchantId,
    ) -> StorageResult<Self> {
        // perform ordering on the application level instead of database level
        generics::generic_filter::<<Self as HasTable>::Table, _, _, Self>(
            conn,
            dsl::payment_id
                .eq(payment_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned()))
                .and(dsl::status.eq(enums::AttemptStatus::Charged)),
            Some(1),
            None,
            Some(dsl::modified_at.desc()),
        )
        .await?
        .into_iter()
        .nth(0)
        .ok_or(report!(DatabaseError::NotFound))
    }

    #[cfg(feature = "v1")]
    pub async fn find_last_successful_or_partially_captured_attempt_by_payment_id_merchant_id(
        conn: &PgPooledConn,
        payment_id: &common_utils::id_type::PaymentId,
        merchant_id: &common_utils::id_type::MerchantId,
    ) -> StorageResult<Self> {
        // perform ordering on the application level instead of database level
        generics::generic_filter::<<Self as HasTable>::Table, _, _, Self>(
            conn,
            dsl::payment_id
                .eq(payment_id.to_owned())
                .and(dsl::merchant_id.eq(merchant_id.to_owned()))
                .and(
                    dsl::status
                        .eq(enums::AttemptStatus::Charged)
                        .or(dsl::status.eq(enums::AttemptStatus::PartialCharged)),
                ),
            Some(1),
            None,
            Some(dsl::modified_at.desc()),
        )
        .await?
        .into_iter()
        .nth(0)
        .ok_or(report!(DatabaseError::NotFound))
    }

    #[cfg(feature = "v2")]
    pub async fn find_last_successful_or_partially_captured_attempt_by_payment_id(
        conn: &PgPooledConn,
        payment_id: &common_utils::id_type::GlobalPaymentId,
    ) -> StorageResult<Self> {
        // perform ordering on the application level instead of database level
        generics::generic_filter::<<Self as HasTable>::Table, _, _, Self>(
            conn,
            dsl::payment_id.eq(payment_id.to_owned()).and(
                dsl::status
                    .eq(enums::AttemptStatus::Charged)
                    .or(dsl::status.eq(enums::AttemptStatus::PartialCharged)),
            ),
            Some(1),
            None,
            Some(dsl::modified_at.desc()),
        )
        .await?
        .into_iter()
        .nth(0)
        .ok_or(report!(DatabaseError::NotFound))
    }

    #[cfg(feature = "v1")]
    pub async fn find_by_merchant_id_connector_txn_id(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        connector_txn_id: &str,
    ) -> StorageResult<Self> {
        let (txn_id, txn_data) = common_utils::types::ConnectorTransactionId::form_id_and_data(
            connector_txn_id.to_string(),
        );
        let connector_transaction_id = txn_id
            .get_txn_id(txn_data.as_ref())
            .change_context(DatabaseError::Others)
            .attach_printable_lazy(|| {
                format!("Failed to retrieve txn_id for ({txn_id:?}, {txn_data:?})")
            })?;
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::connector_transaction_id.eq(connector_transaction_id.to_owned())),
        )
        .await
    }

    #[cfg(feature = "v2")]
    pub async fn find_by_profile_id_connector_transaction_id(
        conn: &PgPooledConn,
        profile_id: &common_utils::id_type::ProfileId,
        connector_txn_id: &str,
    ) -> StorageResult<Self> {
        let (txn_id, txn_data) = common_utils::types::ConnectorTransactionId::form_id_and_data(
            connector_txn_id.to_string(),
        );
        let connector_transaction_id = txn_id
            .get_txn_id(txn_data.as_ref())
            .change_context(DatabaseError::Others)
            .attach_printable_lazy(|| {
                format!("Failed to retrieve txn_id for ({txn_id:?}, {txn_data:?})")
            })?;
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::profile_id
                .eq(profile_id.to_owned())
                .and(dsl::connector_payment_id.eq(connector_transaction_id.to_owned())),
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn find_by_merchant_id_attempt_id(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        attempt_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::attempt_id.eq(attempt_id.to_owned())),
        )
        .await
    }

    #[cfg(feature = "v2")]
    pub async fn find_by_id(
        conn: &PgPooledConn,
        id: &common_utils::id_type::GlobalAttemptId,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::id.eq(id.to_owned()),
        )
        .await
    }

    #[cfg(feature = "v2")]
    pub async fn find_by_payment_id(
        conn: &PgPooledConn,
        payment_id: &common_utils::id_type::GlobalPaymentId,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<<Self as HasTable>::Table, _, _, _>(
            conn,
            dsl::payment_id.eq(payment_id.to_owned()),
            None,
            None,
            Some(dsl::created_at.asc()),
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn find_by_merchant_id_preprocessing_id(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        preprocessing_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::preprocessing_step_id.eq(preprocessing_id.to_owned())),
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn find_by_payment_id_merchant_id_attempt_id(
        conn: &PgPooledConn,
        payment_id: &common_utils::id_type::PaymentId,
        merchant_id: &common_utils::id_type::MerchantId,
        attempt_id: &str,
    ) -> StorageResult<Self> {
        generics::generic_find_one::<<Self as HasTable>::Table, _, _>(
            conn,
            dsl::payment_id.eq(payment_id.to_owned()).and(
                dsl::merchant_id
                    .eq(merchant_id.to_owned())
                    .and(dsl::attempt_id.eq(attempt_id.to_owned())),
            ),
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn find_by_merchant_id_payment_id(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        payment_id: &common_utils::id_type::PaymentId,
    ) -> StorageResult<Vec<Self>> {
        generics::generic_filter::<
            <Self as HasTable>::Table,
            _,
            <<Self as HasTable>::Table as Table>::PrimaryKey,
            _,
        >(
            conn,
            dsl::merchant_id
                .eq(merchant_id.to_owned())
                .and(dsl::payment_id.eq(payment_id.to_owned())),
            None,
            None,
            None,
        )
        .await
    }

    #[cfg(feature = "v1")]
    pub async fn get_filters_for_payments(
        conn: &PgPooledConn,
        pi: &[PaymentIntent],
        merchant_id: &common_utils::id_type::MerchantId,
    ) -> StorageResult<(
        Vec<String>,
        Vec<enums::Currency>,
        Vec<IntentStatus>,
        Vec<enums::PaymentMethod>,
        Vec<enums::PaymentMethodType>,
        Vec<enums::AuthenticationType>,
    )> {
        let active_attempts: Vec<String> = pi
            .iter()
            .map(|payment_intent| payment_intent.clone().active_attempt_id)
            .collect();

        let filter = <Self as HasTable>::table()
            .filter(dsl::merchant_id.eq(merchant_id.to_owned()))
            .filter(dsl::attempt_id.eq_any(active_attempts));

        let intent_status: Vec<IntentStatus> = pi
            .iter()
            .map(|payment_intent| payment_intent.status)
            .collect::<HashSet<IntentStatus>>()
            .into_iter()
            .collect();

        let filter_connector = filter
            .clone()
            .select(dsl::connector)
            .distinct()
            .get_results_async::<Option<String>>(conn)
            .await
            .change_context(DatabaseError::Others)
            .attach_printable("Error filtering records by connector")?
            .into_iter()
            .flatten()
            .collect::<Vec<String>>();

        let filter_currency = filter
            .clone()
            .select(dsl::currency)
            .distinct()
            .get_results_async::<Option<enums::Currency>>(conn)
            .await
            .change_context(DatabaseError::Others)
            .attach_printable("Error filtering records by currency")?
            .into_iter()
            .flatten()
            .collect::<Vec<enums::Currency>>();

        let filter_payment_method = filter
            .clone()
            .select(dsl::payment_method)
            .distinct()
            .get_results_async::<Option<enums::PaymentMethod>>(conn)
            .await
            .change_context(DatabaseError::Others)
            .attach_printable("Error filtering records by payment method")?
            .into_iter()
            .flatten()
            .collect::<Vec<enums::PaymentMethod>>();

        let filter_payment_method_type = filter
            .clone()
            .select(dsl::payment_method_type)
            .distinct()
            .get_results_async::<Option<enums::PaymentMethodType>>(conn)
            .await
            .change_context(DatabaseError::Others)
            .attach_printable("Error filtering records by payment method type")?
            .into_iter()
            .flatten()
            .collect::<Vec<enums::PaymentMethodType>>();

        let filter_authentication_type = filter
            .clone()
            .select(dsl::authentication_type)
            .distinct()
            .get_results_async::<Option<enums::AuthenticationType>>(conn)
            .await
            .change_context(DatabaseError::Others)
            .attach_printable("Error filtering records by authentication type")?
            .into_iter()
            .flatten()
            .collect::<Vec<enums::AuthenticationType>>();

        Ok((
            filter_connector,
            filter_currency,
            intent_status,
            filter_payment_method,
            filter_payment_method_type,
            filter_authentication_type,
        ))
    }

    #[cfg(feature = "v2")]
    #[allow(clippy::too_many_arguments)]
    pub async fn get_total_count_of_attempts(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        active_attempt_ids: &[String],
        connector: Option<String>,
        payment_method_type: Option<enums::PaymentMethod>,
        payment_method_subtype: Option<enums::PaymentMethodType>,
        authentication_type: Option<enums::AuthenticationType>,
        merchant_connector_id: Option<common_utils::id_type::MerchantConnectorAccountId>,
        card_network: Option<enums::CardNetwork>,
    ) -> StorageResult<i64> {
        let mut filter = <Self as HasTable>::table()
            .count()
            .filter(dsl::merchant_id.eq(merchant_id.to_owned()))
            .filter(dsl::id.eq_any(active_attempt_ids.to_owned()))
            .into_boxed();

        if let Some(connector) = connector {
            filter = filter.filter(dsl::connector.eq(connector));
        }

        if let Some(payment_method_type) = payment_method_type {
            filter = filter.filter(dsl::payment_method_type_v2.eq(payment_method_type));
        }
        if let Some(payment_method_subtype) = payment_method_subtype {
            filter = filter.filter(dsl::payment_method_subtype.eq(payment_method_subtype));
        }
        if let Some(authentication_type) = authentication_type {
            filter = filter.filter(dsl::authentication_type.eq(authentication_type));
        }
        if let Some(merchant_connector_id) = merchant_connector_id {
            filter = filter.filter(dsl::merchant_connector_id.eq(merchant_connector_id))
        }
        if let Some(card_network) = card_network {
            filter = filter.filter(dsl::card_network.eq(card_network))
        }

        router_env::logger::debug!(query = %debug_query::<Pg, _>(&filter).to_string());

        // TODO: Remove these logs after debugging the issue for delay in count query
        let start_time = std::time::Instant::now();
        router_env::logger::debug!("Executing count query start_time: {:?}", start_time);
        let result = db_metrics::track_database_call::<<Self as HasTable>::Table, _, _>(
            filter.get_result_async::<i64>(conn),
            db_metrics::DatabaseOperation::Filter,
        )
        .await
        .change_context(DatabaseError::Others)
        .attach_printable("Error filtering count of payments");

        let duration = start_time.elapsed();
        router_env::logger::debug!("Completed count query in {:?}", duration);

        result
    }

    #[cfg(feature = "v1")]
    #[allow(clippy::too_many_arguments)]
    pub async fn get_total_count_of_attempts(
        conn: &PgPooledConn,
        merchant_id: &common_utils::id_type::MerchantId,
        active_attempt_ids: &[String],
        connector: Option<Vec<String>>,
        payment_method: Option<Vec<enums::PaymentMethod>>,
        payment_method_type: Option<Vec<enums::PaymentMethodType>>,
        authentication_type: Option<Vec<enums::AuthenticationType>>,
        merchant_connector_id: Option<Vec<common_utils::id_type::MerchantConnectorAccountId>>,
        card_network: Option<Vec<enums::CardNetwork>>,
        card_discovery: Option<Vec<enums::CardDiscovery>>,
    ) -> StorageResult<i64> {
        let mut filter = <Self as HasTable>::table()
            .count()
            .filter(dsl::merchant_id.eq(merchant_id.to_owned()))
            .filter(dsl::attempt_id.eq_any(active_attempt_ids.to_owned()))
            .into_boxed();

        if let Some(connector) = connector {
            filter = filter.filter(dsl::connector.eq_any(connector));
        }

        if let Some(payment_method) = payment_method {
            filter = filter.filter(dsl::payment_method.eq_any(payment_method));
        }
        if let Some(payment_method_type) = payment_method_type {
            filter = filter.filter(dsl::payment_method_type.eq_any(payment_method_type));
        }
        if let Some(authentication_type) = authentication_type {
            filter = filter.filter(dsl::authentication_type.eq_any(authentication_type));
        }
        if let Some(merchant_connector_id) = merchant_connector_id {
            filter = filter.filter(dsl::merchant_connector_id.eq_any(merchant_connector_id))
        }
        if let Some(card_network) = card_network {
            filter = filter.filter(dsl::card_network.eq_any(card_network))
        }
        if let Some(card_discovery) = card_discovery {
            filter = filter.filter(dsl::card_discovery.eq_any(card_discovery))
        }

        router_env::logger::debug!(query = %debug_query::<Pg, _>(&filter).to_string());

        // TODO: Remove these logs after debugging the issue for delay in count query
        let start_time = std::time::Instant::now();
        router_env::logger::debug!("Executing count query start_time: {:?}", start_time);
        let result = db_metrics::track_database_call::<<Self as HasTable>::Table, _, _>(
            filter.get_result_async::<i64>(conn),
            db_metrics::DatabaseOperation::Filter,
        )
        .await
        .change_context(DatabaseError::Others)
        .attach_printable("Error filtering count of payments");

        let duration = start_time.elapsed();
        router_env::logger::debug!("Completed count query in {:?}", duration);

        result
    }
}
