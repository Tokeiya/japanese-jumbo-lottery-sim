pub mod counter;
pub mod draw;
mod ticket;

pub use draw::{Group, Number, Outcome, PrizeCategory, Report, group, number, outcome, report};
pub use ticket::Ticket;