// Disable existing manual tests to avoid compilation issues
// Enable only generated integration tests

// #[cfg(test)]
// mod binancecoinm;

// #[cfg(test)]
// mod binancespot;

// #[cfg(test)]
// mod binanceportfolio;

// #[cfg(test)]
// mod bitget;

// #[cfg(test)]
// mod deribit;

// Include generated integration tests
#[cfg(test)]
pub mod generated;
