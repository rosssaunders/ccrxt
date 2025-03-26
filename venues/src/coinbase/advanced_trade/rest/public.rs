#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Client {
    venue: Venue,
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CoinbaseAdvancedTrade")
    }
}

impl Client {
    pub fn new(venue: Venue) -> Self {
        Self { venue }
    }
}