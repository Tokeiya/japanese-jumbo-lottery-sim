pub mod counter;
pub mod draw;
pub mod lottery;
mod ticket;

pub use draw::{Group, Number, Outcome, PrizeCategory, Report, group, number, outcome, report};
pub use ticket::Ticket;
