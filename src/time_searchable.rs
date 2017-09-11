//! TODO: comment module timeSearchable

// --- module use statements ---

use Range;
use Time;
use TimeSeries;
use TimeValue;

// --- module trait definitions ---

/// Provide search into `TimeSeries` by time
pub trait TimeSearchable {
    /// Type for *Time*
    type T;

    /// Type for *TimeValue*
    type TV;

    /// Finds range of `Self` that is `after` `time`
    ///
    ///  * `time` - Time being searched to idenify range `after`
    ///  * return - Returns range to resulting items
    ///
    fn after(&self, time: Self::T) -> Range<usize>;

    /// Finds range of `Self` that is `before` `time`
    ///
    ///  * `time` - Time being searched to idenify range `before`
    ///  * return - Returns range to resulting items
    ///
    fn before(&self, time: Self::T) -> Range<usize>;

    /// Finds range of `Self` that is `on_or_after` `time`
    ///
    ///  * `time` - Time being searched to idenify range `onOrAfter`
    ///  * return - Returns range to resulting items
    ///
    fn on_or_after(&self, time: Self::T) -> Range<usize>;

    /// Finds range of `Self` that is `on_or_before` `time`
    ///
    ///  * `time` - Time being searched to idenify range `onOrBefore`
    ///  * return - Returns range to resulting items
    ///
    fn on_or_before(&self, time: Self::T) -> Range<usize>;

    /// Find range in `TimeSeries`
    ///
    ///  * `range` - TODO: comment parm
    ///  * return - Returns range in `TimeSeries` based on `range`
    ///
    fn in_range<'a>(&'a self, range: Range<Self::T>) -> Range<usize>;

    // custom <trait_time_searchable>
    // end <trait_time_searchable>
}

// --- module impl definitions ---

/// Provide ability to compare time component of`TimeValue`
impl<T> TimeSearchable for TimeSeries<T>
    where T: Time<T = T> + Ord
{
    type T = T;
    type TV = TimeValue<T>;

    /// Finds range of `Self` that is `after` `time`
    ///
    ///  * `time` - Time being searched to idenify range `after`
    ///  * return - Returns range to resulting items
    ///
    fn after(&self, time: Self::T) -> Range<usize> {
        // custom <fn time_searchable_time_series_t_after>

        match self.binary_search_by(|tv| tv.time.cmp(&time)) {
            Ok(i) => (i + 1)..self.len(),
            Err(insert_index) => insert_index..self.len(),
        }

        // end <fn time_searchable_time_series_t_after>
    }

    /// Finds range of `Self` that is `before` `time`
    ///
    ///  * `time` - Time being searched to idenify range `before`
    ///  * return - Returns range to resulting items
    ///
    fn before(&self, time: Self::T) -> Range<usize> {
        // custom <fn time_searchable_time_series_t_before>

        match self.binary_search_by(|tv| tv.time.cmp(&time)) {
            Ok(i) => 0..i,
            Err(insert_index) => 0..insert_index,
        }

        // end <fn time_searchable_time_series_t_before>
    }

    /// Finds range of `Self` that is `on_or_after` `time`
    ///
    ///  * `time` - Time being searched to idenify range `onOrAfter`
    ///  * return - Returns range to resulting items
    ///
    fn on_or_after(&self, time: Self::T) -> Range<usize> {
        // custom <fn time_searchable_time_series_t_on_or_after>

        match self.binary_search_by(|tv| tv.time.cmp(&time)) {
            Ok(i) => i..self.len(),
            Err(insert_index) => insert_index..self.len(),
        }

        // end <fn time_searchable_time_series_t_on_or_after>
    }

    /// Finds range of `Self` that is `on_or_before` `time`
    ///
    ///  * `time` - Time being searched to idenify range `onOrBefore`
    ///  * return - Returns range to resulting items
    ///
    fn on_or_before(&self, time: Self::T) -> Range<usize> {
        // custom <fn time_searchable_time_series_t_on_or_before>

        match self.binary_search_by(|tv| tv.time.cmp(&time)) {
            Ok(i) => 0..(i + 1),
            Err(insert_index) => 0..insert_index,
        }

        // end <fn time_searchable_time_series_t_on_or_before>
    }

    /// Find range in `TimeSeries`
    ///
    ///  * `range` - TODO: comment parm
    ///  * return - Returns range in `TimeSeries` based on `range`
    ///
    fn in_range<'a>(&'a self, range: Range<Self::T>) -> Range<usize> {
        // custom <fn time_searchable_time_series_t_in_range>

        self.on_or_after(range.start).start..self.before(range.end).end

        // end <fn time_searchable_time_series_t_in_range>
    }

    // custom <impl TimeSearchable for TimeSeries<T>>
    // end <impl TimeSearchable for TimeSeries<T>>
}

/// Test module for time_searchable module
#[cfg(test)]
mod tests {
    use Year;
    use super::*;
    mod time_searchable_time_series_t {
        use super::*;

        #[test]
        fn after() -> () {
            // custom <test fn time_searchable_time_series_t_after>

            let v = v();
            assert_eq!(v.after(1), 1..v.len());
            assert_eq!(v.after(2), 2..v.len());
            assert_eq!(v.after(3), 2..v.len());

            // end <test fn time_searchable_time_series_t_after>
        }

        #[test]
        fn before() -> () {
            // custom <test fn time_searchable_time_series_t_before>


            let v = v();
            assert_eq!(v.before(1), 0..0);
            assert_eq!(v.before(2), 0..1);
            assert_eq!(v.before(3), 0..2);

            // end <test fn time_searchable_time_series_t_before>
        }

        #[test]
        fn on_or_after() -> () {
            // custom <test fn time_searchable_time_series_t_on_or_after>

            let v = v();
            assert_eq!(v.on_or_after(1), 0..v.len());
            assert_eq!(v.on_or_after(2), 1..v.len());
            assert_eq!(v.on_or_after(3), 2..v.len());

            // end <test fn time_searchable_time_series_t_on_or_after>
        }

        #[test]
        fn on_or_before() -> () {
            // custom <test fn time_searchable_time_series_t_on_or_before>

            let v = v();
            assert_eq!(v.on_or_before(1), 0..1);
            assert_eq!(v.on_or_before(2), 0..2);
            assert_eq!(v.on_or_before(3), 0..2);

            // end <test fn time_searchable_time_series_t_on_or_before>
        }

        #[test]
        fn in_range() -> () {
            // custom <test fn time_searchable_time_series_t_in_range>
            let v = v();
            assert_eq!(v.in_range(0..0), 0..0);
            assert_eq!(v.in_range(1..1), 0..0);
            assert_eq!(v.in_range(1..2), 0..1);
            assert_eq!(v.in_range(1..8), 0..4);
            assert_eq!(v.in_range(2..2), 1..1);
            assert_eq!(v.in_range(2..3), 1..2);
            assert_eq!(v.in_range(2..4), 1..2);
            assert_eq!(v.in_range(2..6), 1..4);

            // end <test fn time_searchable_time_series_t_in_range>
        }

        // custom <module ModuleCodeBlock.moduleBottom>

        fn v() -> TimeSeries<Year> {
            TimeSeries {
                data: vec![TimeValue {
                               time: 1,
                               value: 1.0,
                           },
                           TimeValue {
                               time: 2,
                               value: 2.0,
                           },
                           TimeValue {
                               time: 4,
                               value: 3.0,
                           },
                           TimeValue {
                               time: 5,
                               value: 4.0,
                           }],
            }
        }
        // end <module ModuleCodeBlock.moduleBottom>
    }
}
