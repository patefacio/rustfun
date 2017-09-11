//! Support for time-series data that represent a rate curve

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

// --- module use statements ---

use Time;
use TimeSearchable;
use TimeSeries;
use TimeValue;
use time_revalue::revalue_on;

// --- module trait definitions ---

/// An interface for dealing with rate curves, or time-series data whose values are
/// growth/interest rates.
///
/// Functionality provided includes moving values on time along the `RateCurve`
/// and calculating the discount factor between points on the curve.
///
trait RateCurve {
    /// Type for *Time*
    type T: Time<T = Self::T>;

    /// Revalue the *time_value* to a new *target_time* given *Self*
    ///
    ///  * `time_value` - A *TimeValue* to be revalued on *target_time*
    ///  * `target_time` - Target time for revaluation
    ///  * return - Returns `TimeValue` representing `time_value` moved from it's time to `target_time`
    ///
    fn revalue_on(&self,
                  time_value: TimeValue<Self::T>,
                  target_time: Self::T)
                  -> TimeValue<Self::T>;

    /// Calculates the *discount* implied by *Self* to move $1.0 from *from* to *to*
    ///
    ///  * `from` - Start of discount period
    ///  * `to` - End of discount period
    ///  * return - Returns _discount_ such that _x_ * _discount_ represents
    ///                   value of _x_ moved from *from* to *to* along curve
    ///
    fn discount_from_to(&self, from: Self::T, to: Self::T) -> f64;

    /// Merges *other* *RateCurve* with *Self* to produce new *RateCurve*
    ///
    ///  * `other` - `RateCurve` to merge with *self*
    ///  * return - TODO: document return
    ///
    fn merge(self: &Self, other: &Self) -> Self;

    // custom <trait_rate_curve>
    // end <trait_rate_curve>
}

// --- module impl definitions ---

/// Provide implementation for `TimeSeries<T>`
impl<T> RateCurve for TimeSeries<T>
    where T: Time<T = T> + Ord
{
    type T = T;

    /// Revalue the *time_value* to a new *target_time* given *Self*
    ///
    ///  * `time_value` - A *TimeValue* to be revalued on *target_time*
    ///  * `target_time` - Target time for revaluation
    ///  * return - Returns `TimeValue` representing `time_value` moved from it's time to `target_time`
    ///
    fn revalue_on(&self,
                  time_value: TimeValue<Self::T>,
                  target_time: Self::T)
                  -> TimeValue<Self::T> {
        // custom <fn rate_curve_time_series_t_revalue_on>

        TimeValue {
            time: target_time,
            value: time_value.value * self.discount_from_to(time_value.time(), target_time),
        }

        // end <fn rate_curve_time_series_t_revalue_on>
    }

    /// Calculates the *discount* implied by *Self* to move $1.0 from *from* to *to*
    ///
    ///  * `from` - Start of discount period
    ///  * `to` - End of discount period
    ///  * return - Returns _discount_ such that _x_ * _discount_ represents
    ///                   value of _x_ moved from *from* to *to* along curve
    ///
    fn discount_from_to(&self, from: Self::T, to: Self::T) -> f64 {
        // custom <fn rate_curve_time_series_t_discount_from_to>

        if from > to {
            1.0 / self.discount_from_to(to, from)
        } else {
            let mut value = 1.0;
            let range = self.in_range(from..to);
            let mut rate = if range.start > 0 {
                self[range.start - 1].value
            } else {
                0.0
            };

            for i in range {
                value = revalue_on(value, from, to, rate);
                rate = self[i].value;
            }
            value
        }

        // end <fn rate_curve_time_series_t_discount_from_to>
    }

    /// Merges *other* *RateCurve* with *Self* to produce new *RateCurve*
    ///
    ///  * `other` - `RateCurve` to merge with *self*
    ///  * return - TODO: document return
    ///
    fn merge(self: &Self, other: &Self) -> Self {
        // custom <fn rate_curve_time_series_t_merge>
        TimeSeries { data: vec![] }
        // end <fn rate_curve_time_series_t_merge>
    }

    // custom <impl RateCurve for TimeSeries<T>>
    // end <impl RateCurve for TimeSeries<T>>
}

/// Test module for rate_curve module
#[cfg(test)]
mod tests {
    use Year;
    use super::*;
    mod rate_curve_time_series_t {
        use super::*;

        #[test]
        fn revalue_on() -> () {
            // custom <test fn rate_curve_time_series_t_revalue_on>

            let tv = TimeValue {
                time: 2,
                value: 100.0,
            };

            println!("T before revalue {:?}", tv);
            let x = ts().revalue_on(tv, 5);
            println!("T after revalue {:?}", x);

            // end <test fn rate_curve_time_series_t_revalue_on>
        }

        #[test]
        fn discount_from_to() -> () {
            // custom <test fn rate_curve_time_series_t_discount_from_to>

            println!("Discount on {:?} -> {:?}",
                     ts(),
                     ts().discount_from_to(1, 4));
            // end <test fn rate_curve_time_series_t_discount_from_to>
        }

        #[test]
        fn merge() -> () {
            // custom <test fn rate_curve_time_series_t_merge>
            // end <test fn rate_curve_time_series_t_merge>
        }

        // custom <module ModuleCodeBlock.moduleBottom>

        fn ts() -> TimeSeries<Year> {
            TimeSeries {
                data: vec![TimeValue {
                               time: 1,
                               value: 0.03,
                           },
                           TimeValue {
                               time: 2,
                               value: 0.04,
                           },
                           TimeValue {
                               time: 3,
                               value: 0.05,
                           },
                           TimeValue {
                               time: 4,
                               value: 0.06,
                           }],
            }
        }

        // end <module ModuleCodeBlock.moduleBottom>
    }
}
