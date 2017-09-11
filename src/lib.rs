//! Module for forecasting dossiers

// --- module imports ---

extern crate chrono;
#[cfg(test)]
#[macro_use]
extern crate approx;
extern crate num;

// --- module pub use statements ---

pub use self::num::traits::NumOps;
pub use std::convert::Into;
pub use std::fmt::Debug;
pub use std::ops::Add;
pub use std::ops::Deref;
pub use std::ops::Range;
pub use time_comparable::TimeComparable;
pub use time_revalue::TimeRevalue;
pub use time_searchable::TimeSearchable;
pub use time_series::DateTimeSeries;
pub use time_series::TimeSeries;
pub use time_series::YearTimeSeries;
pub use time_value::Time;
pub use time_value::TimeValue;

mod time_value;
mod time_series;
mod time_comparable;
mod time_searchable;
mod time_revalue;
mod rate_curve;

// --- module type aliases ---

pub type Year = i32;
pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type Duration = chrono::Duration;
pub type DateTimeValue = TimeValue<DateTime>;
pub type YearValue = TimeValue<Year>;
