//! Provide for comparisons of data with time component

// --- module use statements ---

use Debug;
use Time;
use TimeValue;

// --- module trait definitions ---

/// An interface for comparing time component of data with timestamps
pub trait TimeComparable {
    /// Type for *Time*
    type T;

    /// Determines if *self* is `is_same_time` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_same_time` other
    ///
    fn is_same_time<TV>(&self, other: &TV) -> bool where TV: Time<T = Self::T>;

    /// Determines if *self* is `is_before` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_before` other
    ///
    fn is_before<TV>(&self, other: &TV) -> bool where TV: Time<T = Self::T>;

    /// Determines if *self* is `is_on_or_before` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_on_or_before` other
    ///
    fn is_on_or_before<TV>(&self, other: &TV) -> bool where TV: Time<T = Self::T>;

    /// Determines if *self* is `is_after` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_after` other
    ///
    fn is_after<TV>(&self, other: &TV) -> bool where TV: Time<T = Self::T>;

    /// Determines if *self* is `is_on_or_after` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_on_or_after` other
    ///
    fn is_on_or_after<TV>(&self, other: &TV) -> bool where TV: Time<T = Self::T>;

    // custom <trait_time_comparable>
    // end <trait_time_comparable>
}

// --- module impl definitions ---

/// Provide ability to compare time component of`TimeValue`
impl<T> TimeComparable for TimeValue<T>
    where T: Time<T = T> + Debug + PartialEq + Ord + Copy
{
    type T = T;

    /// Determines if *self* is `is_same_time` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_same_time` other
    ///
    fn is_same_time<TV>(&self, other: &TV) -> bool
        where TV: Time<T = Self::T>
    {
        // custom <fn time_comparable_time_value_t_is_same_time>
        self.time() == other.time()
        // end <fn time_comparable_time_value_t_is_same_time>
    }

    /// Determines if *self* is `is_before` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_before` other
    ///
    fn is_before<TV>(&self, other: &TV) -> bool
        where TV: Time<T = Self::T>
    {
        // custom <fn time_comparable_time_value_t_is_before>
        self.time() < other.time()
        // end <fn time_comparable_time_value_t_is_before>
    }

    /// Determines if *self* is `is_on_or_before` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_on_or_before` other
    ///
    fn is_on_or_before<TV>(&self, other: &TV) -> bool
        where TV: Time<T = Self::T>
    {
        // custom <fn time_comparable_time_value_t_is_on_or_before>
        self.time() <= other.time()
        // end <fn time_comparable_time_value_t_is_on_or_before>
    }

    /// Determines if *self* is `is_after` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_after` other
    ///
    fn is_after<TV>(&self, other: &TV) -> bool
        where TV: Time<T = Self::T>
    {
        // custom <fn time_comparable_time_value_t_is_after>
        self.time() > other.time()
        // end <fn time_comparable_time_value_t_is_after>
    }

    /// Determines if *self* is `is_on_or_after` *other*
    ///
    ///  * `other` - Other time being compared to *self* time
    ///  * return - Returns true if *self* `is_on_or_after` other
    ///
    fn is_on_or_after<TV>(&self, other: &TV) -> bool
        where TV: Time<T = Self::T>
    {
        // custom <fn time_comparable_time_value_t_is_on_or_after>
        self.time() >= other.time()
        // end <fn time_comparable_time_value_t_is_on_or_after>
    }

    // custom <impl TimeComparable for TimeValue<T>>
    // end <impl TimeComparable for TimeValue<T>>
}

/// Test module for time_comparable module
#[cfg(test)]
mod tests {
    use DateTimeValue;
    use YearValue;
    use super::*;
    mod time_comparable_time_value_t {
        use super::*;

        #[test]
        fn is_same_time() -> () {
            // custom <test fn time_comparable_time_value_t_is_same_time>
            assert_eq!(tv1().is_same_time(&tv1()), true);
            assert_eq!(tv1().is_same_time(&tv2()), false);

            let dtv1 = dtv1();
            assert_eq!(dtv1.is_same_time(&dtv1), true);
            let dtv2 = dtv2();
            assert_eq!(dtv1.is_same_time(&dtv2), false);
            assert_eq!(dtv2.is_same_time(&dtv1), false);
            assert_eq!(dtv2.is_same_time(&dtv2), true);

            // end <test fn time_comparable_time_value_t_is_same_time>
        }

        #[test]
        fn is_before() -> () {
            // custom <test fn time_comparable_time_value_t_is_before>
            assert_eq!(tv1().is_before(&tv1()), false);
            assert_eq!(tv1().is_before(&tv2()), true);

            let dtv1 = dtv1();
            assert_eq!(dtv1.is_before(&dtv1), false);
            let dtv2 = dtv2();
            assert_eq!(dtv1.is_before(&dtv2), true);
            assert_eq!(dtv2.is_before(&dtv1), false);
            assert_eq!(dtv2.is_before(&dtv2), false);

            // end <test fn time_comparable_time_value_t_is_before>
        }

        #[test]
        fn is_on_or_before() -> () {
            // custom <test fn time_comparable_time_value_t_is_on_or_before>

            assert_eq!(tv1().is_on_or_before(&tv1()), true);
            assert_eq!(tv1().is_on_or_before(&tv2()), true);
            assert_eq!(tv2().is_on_or_before(&tv1()), false);

            let dtv1 = dtv1();
            assert_eq!(dtv1.is_on_or_before(&dtv1), true);
            let dtv2 = dtv2();
            assert_eq!(dtv1.is_on_or_before(&dtv2), true);
            assert_eq!(dtv2.is_on_or_before(&dtv1), false);
            assert_eq!(dtv2.is_on_or_before(&dtv2), true);

            // end <test fn time_comparable_time_value_t_is_on_or_before>
        }

        #[test]
        fn is_after() -> () {
            // custom <test fn time_comparable_time_value_t_is_after>
            assert_eq!(tv1().is_after(&tv1()), false);
            assert_eq!(tv1().is_after(&tv2()), false);
            assert_eq!(tv2().is_after(&tv1()), true);

            let dtv1 = dtv1();
            assert_eq!(dtv1.is_after(&dtv1), false);
            let dtv2 = dtv2();
            assert_eq!(dtv1.is_after(&dtv2), false);
            assert_eq!(dtv2.is_after(&dtv1), true);
            assert_eq!(dtv2.is_after(&dtv2), false);
            // end <test fn time_comparable_time_value_t_is_after>
        }

        #[test]
        fn is_on_or_after() -> () {
            // custom <test fn time_comparable_time_value_t_is_on_or_after>

            assert_eq!(tv1().is_on_or_after(&tv1()), true);
            assert_eq!(tv1().is_on_or_after(&tv2()), false);
            assert_eq!(tv2().is_on_or_after(&tv1()), true);

            let dtv1 = dtv1();
            assert_eq!(dtv1.is_on_or_after(&dtv1), true);
            let dtv2 = dtv2();
            assert_eq!(dtv1.is_on_or_after(&dtv2), false);
            assert_eq!(dtv2.is_on_or_after(&dtv1), true);
            assert_eq!(dtv2.is_on_or_after(&dtv2), true);
            // end <test fn time_comparable_time_value_t_is_on_or_after>
        }

        // custom <module ModuleCodeBlock.moduleBottom>
        use chrono::prelude::*;

        fn tv1() -> YearValue {
            let tv1 = YearValue {
                time: 1,
                value: 3.14,
            };
            tv1
        }

        fn tv2() -> YearValue {
            let tv2 = YearValue {
                time: 2,
                value: 3.14,
            };
            tv2
        }

        fn dtv1() -> DateTimeValue {
            let dtv1 = DateTimeValue {
                time: Utc::now(),
                value: 3.14,
            };
            dtv1
        }

        fn dtv2() -> DateTimeValue {
            let dtv2 = DateTimeValue {
                time: Utc::now(),
                value: 3.14,
            };
            dtv2
        }

        // end <module ModuleCodeBlock.moduleBottom>
    }
}
