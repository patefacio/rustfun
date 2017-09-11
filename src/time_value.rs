//! TODO: comment module timeValue

// --- module use statements ---

use Add;
use DateTime;
use Debug;
use Into;
use Year;
use num::traits::NumOps;

// --- module struct definitinos ---

/// Associates a *time* with a *value* for time-series data
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct TimeValue<T>
    where T: Time<T = T>
{
    /// Time associated with `self`
    pub time: T,
    /// Value associated with `self`
    pub value: f64,
}

// --- module trait definitions ---

/// Represents a type with a time component
pub trait Time: Debug + PartialEq + PartialOrd + Copy {
    /// Type of time
    type T;

    /// Acess the time field of data
    ///
    ///  * return - The time associated with the data
    ///
    fn time<'a>(&'a self) -> Self::T;

    /// Years from self until `time`
    ///
    ///  * `time` - The `time` being compared, as in (`time` - self) in years
    ///  * return - Number of years from self until `time`
    ///
    fn years_until<'a>(&'a self, time: &Self::T) -> f64;

    // custom <trait_time>
    // end <trait_time>
}

// --- module impl definitions ---

/// `Year` is its own time component
impl Time for Year {
    type T = Self;

    /// Acess the time field of data
    ///
    ///  * return - The time associated with the data
    ///
    fn time<'a>(&'a self) -> Self::T {
        // custom <fn time_year_time>
        *self
        // end <fn time_year_time>
    }

    /// Years from self until `time`
    ///
    ///  * `time` - The `time` being compared, as in (`time` - self) in years
    ///  * return - Number of years from self until `time`
    ///
    fn years_until<'a>(&'a self, time: &Self::T) -> f64 {
        // custom <fn time_year_years_until>

        (time - *self).into()

        // end <fn time_year_years_until>
    }

    // custom <impl Time for Year>
    // end <impl Time for Year>
}

/// `DateTime` is its own time component
impl Time for DateTime {
    type T = Self;

    /// Acess the time field of data
    ///
    ///  * return - The time associated with the data
    ///
    fn time<'a>(&'a self) -> Self::T {
        // custom <fn time_date_time_time>
        *self
        // end <fn time_date_time_time>
    }

    /// Years from self until `time`
    ///
    ///  * `time` - The `time` being compared, as in (`time` - self) in years
    ///  * return - Number of years from self until `time`
    ///
    fn years_until<'a>(&'a self, time: &Self::T) -> f64 {
        // custom <fn time_date_time_years_until>

        let days: f64 = (self.signed_duration_since(*time).num_days() as u32).into();
        return days / 364.22; // TODO: Make constant

        // end <fn time_date_time_years_until>
    }

    // custom <impl Time for DateTime>
    // end <impl Time for DateTime>
}

/// Provide access to the `time` component of `TimeValue<T>`
impl<T> Time for TimeValue<T>
    where T: Time<T = T> + Debug + PartialEq + Ord + Copy
{
    type T = T;

    /// Acess the time field of data
    ///
    ///  * return - The time associated with the data
    ///
    fn time<'a>(&'a self) -> Self::T {
        // custom <fn time_time_value_t_time>
        self.time
        // end <fn time_time_value_t_time>
    }

    /// Years from self until `time`
    ///
    ///  * `time` - The `time` being compared, as in (`time` - self) in years
    ///  * return - Number of years from self until `time`
    ///
    fn years_until<'a>(&'a self, time: &Self::T) -> f64 {
        // custom <fn time_time_value_t_years_until>
        self.time().years_until(time)
        // end <fn time_time_value_t_years_until>
    }

    // custom <impl Time for TimeValue<T>>
    // end <impl Time for TimeValue<T>>
}

// custom <module ModuleCodeBlock.moduleBottom>

impl<'a, T> Add<f64> for &'a TimeValue<T>
    where T: Time<T = T>
{
    type Output = TimeValue<T>;

    fn add(self, rhs: f64) -> Self::Output {
        TimeValue { time: self.time, value: self.value + rhs }
    }
}

// end <module ModuleCodeBlock.moduleBottom>
