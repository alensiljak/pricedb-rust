/*
 * SeaQuery uses enums
 */

pub enum PriceSchema {
    Table,
    Id,
    Security_id,
    Date,
    Time,
    Value,
    Denom,
    Currency,
}

pub enum SecuritySchema {
    Table,
    Id,
    Namespace,
    Symbol,
    Updater,
    Currency,
    LedgerSymbol,
    Notes,
    // prices
}
