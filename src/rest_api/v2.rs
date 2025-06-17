use super::DzengiRestClient;
use crate::auto_import_models;
use std::ops::Deref;

auto_import_models! {
    get_account_info,
    get_server_time,
    get_trades_aggregated,
    post_close_trading_position,
    get_currencies,
    get_deposit_address,
    get_deposits,
    get_depth,
    get_exchange_info,
    get_fetch_order,
    get_funding_limits,
    get_klines,
    get_ledger,
    get_my_trades,
    get_open_orders,
    get_leverage_settings,
    post_order,
    put_order,
    delete_order,
    get_ticker_24hr,
    get_trading_fees,
    get_trading_limits,
    trading_positions,
    get_trading_positions_history,
    get_transactions,
    post_update_trading_order,
    post_update_trading_position,
    get_withdrawals
}

pub struct Version2<'a>(&'a DzengiRestClient);

impl<'a> Version2<'a> {
    pub fn new(client: &'a DzengiRestClient) -> Self {
        Self(client)
    }
}

impl<'a> Deref for Version2<'a> {
    type Target = DzengiRestClient;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
