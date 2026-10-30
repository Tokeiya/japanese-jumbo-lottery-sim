pub mod counter;
pub mod draw;
mod lottery;
mod ticket;

//pub use lottery::Lottery;
pub use draw::{Group, Number, Outcome, PrizeCategory, Report, group, number, outcome, report};
pub use ticket::Ticket;
