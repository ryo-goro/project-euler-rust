const TARGET_CENTURY: usize = 20;
const START_YEAR: usize = (TARGET_CENTURY - 1) * 100 + 1;
const END_YEAR: usize = TARGET_CENTURY * 100;
const DAYS_IN_MONTH: [usize; 13] = [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
const SUNDAY: usize = 0;

pub fn solve() -> u64 {
    // Weekday of the first day of the current month.
    // Initially set to 1 Jan 1901.
    // 1 Jan 1900 was a Monday (1).
    let mut first_weekday = (1 + if is_leap_year(START_YEAR - 1) { 366 } else { 365 }) % 7;
    let mut count = 0;

    for year in START_YEAR..=END_YEAR {
        for (month, days) in DAYS_IN_MONTH.iter().enumerate().skip(1) {
            if first_weekday == SUNDAY {
                count += 1;
            }

            first_weekday += days;
            if month == 2 && is_leap_year(year) {
                first_weekday += 1;
            }
            first_weekday %= 7;
        }
    }

    count
}

fn is_leap_year(year: usize) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100))
        || year.is_multiple_of(400)
}
