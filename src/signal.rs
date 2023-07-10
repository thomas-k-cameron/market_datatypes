#[derive(
    serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Default,
)]
pub enum Signal {
    Buy = 1,
    Sell = -1,
    #[default]
    None = 0,
}
