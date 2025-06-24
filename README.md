# Dzengi

Official information:
- [Website](https://dzengi.com/)
- [API](https://dzengi.com/api)
- [Swagger](https://apitradedoc.dzengi.com/swagger-ui.html)

## Disclaimer

This software is for educational purposes only. Do not risk money which you are afraid to lose.
USE THE SOFTWARE AT YOUR OWN RISK. THE AUTHORS AND ALL AFFILIATES ASSUME NO RESPONSIBILITY FOR YOUR TRADING RESULTS.

## Installation

```
cargo add api-dzengi-rs
```

## Usage

```rs
let settings = UserSettings::new("api_key", "secret");
let mut rest = DzengiRestClient::new().with_user_settings(Some(settings));

let resp = rest
    .v1()
    .klines(
        KlinesRequest::new(
            "BTC/USD_LEVERAGE".into(), Interval::FiveMinutes
        )
        .with_limit(Some(10)),
    )
    .await?;

println!("{:?}", resp)
```


## Endpoints

### Rest V1/V2
| Implemented | Info | API | Method |
| --- | --- | --- | --- |
| [x] | account info | [GET] /api/v#/account | `account_info` |
| [x] | trades aggregated | [GET] /api/v#/aggTrades | `trades_aggregated` |
| [x] | trading position close | [POST] /api/v#/closeTradingPosition | `close_trading_position` |
| [x] | list of currencies | [GET] /api/v#/currencies | `currencies` |
| [x] | string of address | [GET] /api/v#/depositAddress | `deposit_address` |
| [x] | list of deposits | [GET] /api/v#/deposits | `deposits` |
| [x] | order book | [GET] /api/v#/depth | `depth` |
| [x] | exchange info | [GET] /api/v#/exchangeInfo | `exchange_info` |
| [x] | order | [GET] /api/v#/fetchOrder | `fetch_order` |
| [x] | list of funding limits | [GET] /api/v#/fundingLimits | `funding_limits` |
| [x] | klines | [GET] /api/v#/klines | `klines` |
| [x] | list of ledgers | [GET] /api/v#/ledger | `ledger` |
| [x] | leverage settings | [GET] /api/v#/leverageSettings | `leverage_settings` |
| [x] | list of trades | [GET] /api/v#/myTrades | `my_trades` |
| [x] | list of open orders | [GET] /api/v#/openOrders | `open_orders` |
| [x] | create order | [POST] /api/v#/order | `order_create` |
| [x] | edit exchange order | [PUT] /api/v#/order | `order_change` |
| [x] | cancel order | [DELETE] /api/v#/order | `order_cancel` |
| [x] | price change | [GET] /api/v#/ticker/24hr | `ticker_24hr` |
| [x] | server time | [GET] /api/v#/time | `server_time` |
| [x] | list of fees | [GET] /api/v#/tradingFees | `trading_fees` |
| [x] | list of limits | [GET] /api/v#/tradingLimits | `trading_limits` |
| [x] | list of leverage trades | [GET] /api/v#/tradingPositions | `trading_positions` |
| [x] | list of historical positions | [GET] /api/v#/tradingPositionsHistory | `trading_positions_history` |
| [x] | list of transactions | [GET] ​/api​/v#​/transactions | `transactions` |
| [x] | leverage orders edit | [POST] /api/v#/updateTradingOrder | `update_trading_order` |
| [x] | leverage trade edit | [POST] /api/v#/updateTradingPosition | `update_trading_position` |
| [x] | list of withdrawals | [GET] ​/api​/v#​/withdrawals | `withdrawals` |

### Websocket V1/V2
Future versions!
