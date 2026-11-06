#![expect(unused_variables, reason = "Under construction")]
#![expect(unfulfilled_lint_expectations, reason = "Under construction")]


use std::fmt::Formatter;
use super::formatter::ThousandsSeparator;

pub struct DecimalFormatter;

impl ThousandsSeparator for DecimalFormatter{
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}