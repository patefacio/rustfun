//! TODO: comment module timeRevalue

// --- module use statements ---

use Time;
use TimeValue;

// --- module trait definitions ---

/// Provide search into `TimeSeries` by time
pub trait TimeRevalue {
    /// Type for *Time*
    type T;

    /// Revalue `self` from it's time to `target_time` with rate `r`
    ///
    ///  * `target_time` - Time to revalue to
    ///  * `cc_rate` - Countinously compounded rate for revalue
    ///  * return - Returns original value revalued to `target_time` with `cc_rate`
    ///
    fn revalue_on<'a>(&'a self, target_time: Self::T, cc_rate: f64) -> Self;

    // custom <trait_time_revalue>
    // end <trait_time_revalue>
}

// --- module impl definitions ---

/// Provide `TimeRevalue` implementation for `TimeValue<T>`
impl<T> TimeRevalue for TimeValue<T>
    where T: Time<T = T> + Ord
{
    type T = T;

    /// Revalue `self` from it's time to `target_time` with rate `r`
    ///
    ///  * `target_time` - Time to revalue to
    ///  * `cc_rate` - Countinously compounded rate for revalue
    ///  * return - Returns original value revalued to `target_time` with `cc_rate`
    ///
    fn revalue_on<'a>(&'a self, target_time: Self::T, cc_rate: f64) -> Self {
        // custom <fn time_revalue_time_value_t_revalue_on>

        TimeValue {
            time: target_time,
            value: self.value * (cc_rate * self.years_until(&target_time)).exp(),
        }

        // end <fn time_revalue_time_value_t_revalue_on>
    }

    // custom <impl TimeRevalue for TimeValue<T>>
    // end <impl TimeRevalue for TimeValue<T>>
}

// --- module function definitions ---

/// Move `srouce_value` from `source_time` to `target_time` given continously compounded rate `ccRate`
///
///  * `source_value` - TODO: comment parm
///  * `source_time` - TODO: comment parm
///  * `target_time` - TODO: comment parm
///  * `cc_rate` - TODO: comment parm
///  * return - TODO: document return
///
pub fn revalue_on<T>(source_value: f64, source_time: T, target_time: T, cc_rate: f64) -> f64
    where T: Time<T = T>
{
    // custom <fn revalue_on>

    source_value * (cc_rate * source_time.years_until(&target_time)).exp()

    // end <fn revalue_on>
}

/// Test module for time_revalue module
#[cfg(test)]
mod tests {
    use super::*;
    mod time_revalue_time_value_t {
        use super::*;

        #[test]
        fn revalue_on() -> () {
            // custom <test fn time_revalue_time_value_t_revalue_on>

            let tv1 = TimeValue {
                time: 1,
                value: 1.0,
            };

            assert_relative_eq!(tv1.revalue_on(2, 0.03).value, (0.03_f64 * 1.0_f64).exp());
            assert_relative_ne!(tv1.revalue_on(2, 0.03).value, (0.0301_f64 * 1.0_f64).exp());

            // end <test fn time_revalue_time_value_t_revalue_on>
        }

        // custom <module ModuleCodeBlock.moduleBottom>
        // end <module ModuleCodeBlock.moduleBottom>
    }
}
