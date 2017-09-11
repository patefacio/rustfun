//! Support for scalar operations on `TimeSeries`

// --- module use statements ---

use Add;
use DateTime;
use Deref;
use NumOps;
use Time;
use TimeValue;
use Year;

// --- module type aliases ---

pub type DateTimeSeries = TimeSeries<DateTime>;
pub type YearTimeSeries = TimeSeries<Year>;

// custom <module ModuleCodeBlock.moduleBottom>

#[derive(Debug, Default)]
pub struct TimeSeries<T>
    where T: Time<T = T>
{
    pub data: Vec<TimeValue<T>>,
}

impl<T> Deref for TimeSeries<T>
    where T: Time<T = T>
{
    type Target = Vec<TimeValue<T>>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<'a, T> Add<f64> for &'a TimeSeries<T>
    where T: Time<T = T> + Add<f64, Output = T>
{
    type Output = TimeSeries<T>;

    fn add(self, rhs: f64) -> Self::Output {
        let result = TimeSeries { data: Vec::with_capacity(self.len()) };

        for (i, &tv) in self.iter().enumerate() {
            result[i] = tv + rhs;
        }
        result
    }
}


// end <module ModuleCodeBlock.moduleBottom>
