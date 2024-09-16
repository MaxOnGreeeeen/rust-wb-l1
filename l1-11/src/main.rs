use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

fn main() {
    let mut temperature_fluctuations: Vec<f64> =
        vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    temperature_fluctuations.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let intervals: Vec<Interval> =
        Interval::from_hash_map(get_intervals(&temperature_fluctuations));

    println!("{:?}", intervals);
}

// Получение интервалов из значений
pub fn get_intervals(values: &Vec<f64>) -> HashMap<(i64, i64), Vec<f64>> {
    assert!(values.len() > 0);

    let mut intervals: HashMap<(i64, i64), Vec<f64>> = HashMap::new();
    for &temp in values {
        let lower_bound = (temp / 10.0).floor() as i64 * 10;
        let upper_bound = lower_bound + 10;

        intervals
            .entry((lower_bound, upper_bound))
            .or_insert_with(Vec::new)
            .push(temp);
    }

    intervals
}

enum IntervalBoundary {
    Inclusive,    // []
    NotInclusive, // ()
}

pub struct Interval {
    start: (i64, IntervalBoundary),
    end: (i64, IntervalBoundary),
    values: Vec<f64>,
}
impl Interval {
    fn from_hash_map(value: HashMap<(i64, i64), Vec<f64>>) -> Vec<Interval> {
        let min_interval_value = value.keys().min_by(|x, y| x.0.cmp(&y.0)).unwrap();
        let max_interval_value = value.keys().max_by(|x, y| x.1.cmp(&y.1)).unwrap();

        let mut values: Vec<Interval> = value
            .iter()
            .map(|(interval, values)| {
                let mut start_boundary = IntervalBoundary::NotInclusive;
                let mut end_boundary = IntervalBoundary::NotInclusive;

                if interval.0 == min_interval_value.0 {
                    start_boundary = IntervalBoundary::Inclusive;
                }
                if interval.1 == max_interval_value.1 {
                    end_boundary = IntervalBoundary::Inclusive;
                }

                return Interval {
                    start: (interval.0, start_boundary),
                    end: (interval.1, end_boundary),
                    values: values.clone(),
                };
            })
            .collect();
        values.sort_by(|a, b| a.start.0.partial_cmp(&(b.start.0)).unwrap());
        return values;
    }
}

// Форматирование для красивого вывода
impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_bracket = match self.start.1 {
            IntervalBoundary::Inclusive => "[",
            IntervalBoundary::NotInclusive => "(",
        };
        let end_bracket = match self.end.1 {
            IntervalBoundary::Inclusive => "]",
            IntervalBoundary::NotInclusive => ")",
        };

        write!(
            f,
            "{}{}, {}{}:{{{:?}}}",
            start_bracket, self.start.0, self.end.0, end_bracket, self.values
        )
    }
}
impl Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_bracket = match self.start.1 {
            IntervalBoundary::Inclusive => "[",
            IntervalBoundary::NotInclusive => "(",
        };
        let end_bracket = match self.end.1 {
            IntervalBoundary::Inclusive => "]",
            IntervalBoundary::NotInclusive => ")",
        };
        write!(
            f,
            "{}{}, {}{}:{{{:?}}}",
            start_bracket, self.start.0, self.end.0, end_bracket, self.values
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_intervals, Interval};

    #[test]
    fn get_intervals_formatted() {
        let temperature_fluctuations: Vec<f64> =
            vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
        let result =
            "[[-30, -20):{[-25.4, -27.0, -21.0]}, (10, 20):{[13.0, 19.0, 15.5]}, (20, 30):{[24.5]}, (30, 40]:{[32.5]}]";

        let get_intervals_call_result =
            Interval::from_hash_map(get_intervals(&temperature_fluctuations));
        let get_intervals_call_result_string = format!("{:?}", get_intervals_call_result);

        assert_eq!(get_intervals_call_result_string, result);
    }
}
