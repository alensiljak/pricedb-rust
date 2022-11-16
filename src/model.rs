/*
 * Model structs
 */
pub(crate) struct Security {
    id: i32,
    symbol: String,
    namespace: String,
    updater: String,
    currency: String,
    ledger_symbol: String
}
