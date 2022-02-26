extern crate regex;
extern crate chrono;
extern crate time;

use self::regex::Regex;
use self::chrono::prelude::*;
use self::time::Duration;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
enum GuardAction {
    BeginShift(u32),
    FallAsleep,
    WakeUp,
}

pub struct TimeTable {
    shifts: Vec<Shift>
}

struct Shift {
    _date: NaiveDate,
    id: u32,
    asleep: [bool; 60],
}

impl Shift {
    fn new(mut shift_info: VecDeque<(NaiveDateTime, GuardAction)>) -> Shift {
        let (start_date_time, start_guard_action) = shift_info.pop_front().unwrap();

        // Get id of the guard on that shift
        let id = match start_guard_action {
            GuardAction::BeginShift(id) => id,
            _ => panic!("First actions should begin shift! Found other")
        };

        // Get date of the shift
        let _date = Shift::get_date(&start_date_time);

        // Calculate on which minutes guard was asleep
        let asleep = Shift::get_asleep(shift_info);

        Shift {
            _date,
            id,
            asleep
        }
    }

    ///
    /// Calculates at which minutes guard was asleep based on guard actions.
    /// True means guard was asleep at the minute/index, false - guard was awake.
    ///
    fn get_asleep(shift_info: VecDeque<(NaiveDateTime, GuardAction)>) -> [bool; 60] {
        let mut asleep = [false; 60];

        for (date_time, guard_action) in shift_info {
            let action_minute = date_time.time().minute() as usize;

            match guard_action {
                GuardAction::FallAsleep => {
                    for v in &mut asleep[action_minute..] {
                        *v = true;
                    }
                },
                GuardAction::WakeUp => {
                    for v in &mut asleep[action_minute..] {
                        *v = false;
                    }
                },
                _ => panic!("Found start of the shift inside another shift!")
            }
        }

        asleep
    }

    ///
    /// Gets date of the shift.
    /// If shift started after 23:00, the date of the shift is set to the next day.
    ///
    fn get_date(start_date_time: &NaiveDateTime) -> NaiveDate {
        let start_time = start_date_time.time();
        let minimal_start_time = NaiveTime::from_hms(23, 0, 0);

        if start_time > minimal_start_time {
            start_date_time.date() + Duration::days(1)
        } else {
            start_date_time.date()
        }
    }
}

impl TimeTable {
    pub fn new(text: &str) -> TimeTable {
        let mut shifts = Vec::new();
        let mut entries = TimeTable::parse(text);

        'outer: loop {
            let mut current_entries = VecDeque::new();
            current_entries.push_back(entries.pop_front().unwrap());

            loop {
                let temp = entries.pop_front();

                match temp {
                    Some(v) => {
                        if v.1 == GuardAction::WakeUp {
                            current_entries.push_back(v);
                        } else if v.1 == GuardAction::FallAsleep {
                            current_entries.push_back(v);
                        } else {
                            entries.push_front(v);
                            shifts.push(Shift::new(current_entries));
                            break;
                        }
                    },
                    None => {
                        shifts.push(Shift::new(current_entries));
                        break 'outer;
                    }
                }
            }
        }

        TimeTable { shifts }
    }

    ///
    /// Uses regex to parse text and extract info from each line.
    /// After extraction, data is sorted by chronological order.
    ///
    fn parse(text: &str) -> VecDeque<(NaiveDateTime, GuardAction)> {
        let date_time_re: Regex = Regex::new(r"(?m)\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})] (?P<data>[a-zA-Z0-9 #]+)").unwrap();
        let guard_id_re: Regex = Regex::new(r"(?m)#(?P<id>\d+)").unwrap();
        let mut entries: Vec<(NaiveDateTime, GuardAction)> = Vec::new(); // Container for date, guard action

        for capture in date_time_re.captures_iter(text) {
            let year: i32 = capture["year"].parse().unwrap();
            let month: u32 = capture["month"].parse().unwrap();
            let day: u32 = capture["day"].parse().unwrap();

            let hour: u32 = capture["hour"].parse().unwrap();
            let min: u32 = capture["minute"].parse().unwrap();

            let date_time = NaiveDate::from_ymd(year, month, day).and_hms(hour, min, 0);//Utc.ymd(year, month, day).and_hms(hour, min, 0);
            let other_text = &capture["data"];
            let id_option = guard_id_re.captures(other_text);

            let guard_action = match id_option {
                Some(id) => GuardAction::BeginShift(id["id"].parse().unwrap()),
                None => {
                    if other_text == "falls asleep" {
                        GuardAction::FallAsleep
                    } else if other_text == "wakes up" {
                        GuardAction::WakeUp
                    } else {
                        panic!("Unknown text after date-time.");
                    }
                }
            };

            entries.push((date_time, guard_action));
        }
        entries.sort_by_key(|k| k.0);

        VecDeque::from(entries)
    }

    ///
    /// Calculates how many minutes, each guard was asleep in total.
    /// Returns HashMap, where key is the id of the guard and value
    /// is total number of minutes, the guard was asleep.
    ///
    fn guard_asleep_times(&self) -> HashMap<u32, usize> {
        let mut guard_asleep_map = HashMap::new();

        for shift in &self.shifts {
            let asleep_time = shift.asleep
                .iter()
                .filter(|&&v| v == true)
                .count();

            *guard_asleep_map.entry(shift.id).or_insert(0) += asleep_time;
        }

        guard_asleep_map
    }

    ///
    /// Calculates which guard is asleep the most.
    /// Returns guard id.
    ///
    pub fn most_asleep_guard(&self) -> u32 {
        let guard_asleep_map = self.guard_asleep_times();

        *guard_asleep_map.iter().max_by_key(|&(_, &b)| b).unwrap().0
    }

    ///
    /// Calculates on which minute, guard is asleep the most.
    /// Returns that minute.
    ///
    pub fn most_asleep_minute(&self, guard_id: u32) -> usize {
        let guard_shifts: Vec<&Shift> = self.shifts.iter()
            .filter(|shift| shift.id == guard_id)
            .collect();
        let mut minutes: Vec<u32> = vec![0; 60];

        for shift in guard_shifts {
            let shift_minutes: Vec<u32> = shift.asleep.iter().map(|v| *v as u32).collect();

            for (a, b) in minutes.iter_mut().zip(shift_minutes) {
                *a += b;
            }
        }

        let mut longest_minute = 0;
        let mut max = 0;

        for (i, minute) in minutes.iter().enumerate() {
            if *minute > max {
                max = *minute;
                longest_minute = i;
            }
        }

        longest_minute
    }

    ///
    /// Calculates which guard is most frequently asleep on the same minute.
    /// Returns tuple (guard id, minute).
    ///
    pub fn most_frequent_guard_on_minute(&self) -> (u32, usize) {
        let mut data: Vec<HashMap<u32, usize>> = vec![HashMap::new(); 60];

        for shift in &self.shifts {
            for (i, v) in shift.asleep.iter().enumerate() {
                if *v == true {
                    *data[i].entry(shift.id).or_insert(0) += 1;
                }
            }
        }

        let mut minute = 0;
        let mut max_id = 0;
        let mut max = 0;

        for (i, d) in data.iter().enumerate() {
            let (id, times) = match d.iter().max_by_key(|&(_,b)| *b) {
                Some(v) => v,
                None => continue
            };

            if *times > max {
                max = *times;
                max_id = *id;
                minute = i;
            }
        }

        (max_id, minute)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_new() {
        let info1 = (
            NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 0, 0),
            GuardAction::BeginShift(10)
        );
        let info2 = (
            NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 5, 0),
            GuardAction::FallAsleep
        );
        let info3 = (
            NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 25, 0),
            GuardAction::WakeUp
        );
        let info4 = (
            NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 30, 0),
            GuardAction::FallAsleep
        );
        let info5 = (
            NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 55, 0),
            GuardAction::WakeUp
        );

        let shift_info = VecDeque::from(vec![info1, info2, info3, info4, info5]);
        let shift = Shift::new(shift_info);
        let mut asleep = vec![false; 5];
        asleep.extend(vec![true; 20]);
        asleep.extend(vec![false; 5]);
        asleep.extend(vec![true; 25]);
        asleep.extend(vec![false; 5]);

        assert_eq!(NaiveDate::from_ymd(1518, 11, 01), shift._date);
        assert_eq!(10, shift.id);
        assert_eq!(asleep, shift.asleep.to_vec());
    }

    ///
    /// Tests if text parsing is correct.
    ///
    #[test]
    fn test_parse() {
        let text = include_str!("../test");
        let entries = TimeTable::parse(text);

        let test1 = (
            NaiveDate::from_ymd(1518, 11, 01).and_hms(0, 0, 0),
            GuardAction::BeginShift(10)
        );
        let test2 = (
            NaiveDate::from_ymd(1518, 11, 02).and_hms(0, 40, 0),
            GuardAction::FallAsleep
        );
        let test3 = (
            NaiveDate::from_ymd(1518, 11, 03).and_hms(0, 29, 0),
            GuardAction::WakeUp
        );
        let test4 = (
            NaiveDate::from_ymd(1518, 11, 05).and_hms(0, 55, 0),
            GuardAction::WakeUp
        );

        assert_eq!(test1, entries[0]);
        assert_eq!(test2, entries[6]);
        assert_eq!(test3, entries[10]);
        assert_eq!(test4, entries[16]);
    }

    #[test]
    fn test_asleep_times() {
        let text = include_str!("../test");
        let time_table = TimeTable::new(text);

        let guard_asleep_times = time_table.guard_asleep_times();
        let mut true_guard_asleep_times: HashMap<u32, usize> = HashMap::new();
        true_guard_asleep_times.insert(10, 50);
        true_guard_asleep_times.insert(99, 30);

        assert_eq!(true_guard_asleep_times, guard_asleep_times);
    }

    #[test]
    fn test_most_asleep_guard() {
        let text = include_str!("../test");
        let time_table = TimeTable::new(text);

        let most_asleep_guard = time_table.most_asleep_guard();

        assert_eq!(10, most_asleep_guard);
    }

    #[test]
    fn test_most_asleep_minute() {
        let text = include_str!("../test");
        let time_table = TimeTable::new(text);

        let most_asleep_minute10 = time_table.most_asleep_minute(10);
        let most_asleep_minute99 = time_table.most_asleep_minute(99);

        assert_eq!(24, most_asleep_minute10);
        assert_eq!(45, most_asleep_minute99);
    }

    #[test]
    fn test_most_frequent_guard_on_minute() {
        let text = include_str!("../test");
        let time_table = TimeTable::new(text);

        let (guard_id, minute) = time_table.most_frequent_guard_on_minute();

        assert_eq!((99, 45), (guard_id, minute));
    }
}
