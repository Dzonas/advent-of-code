mod lib;

use lib::TimeTable;

fn main() {
    let input = include_str!("../input");
    let time_table = TimeTable::new(input);

    let most_asleep_guard = time_table.most_asleep_guard();
    let most_asleep_minute = time_table.most_asleep_minute(most_asleep_guard);

    println!("---Strategy 1---");
    println!("Most asleep guard: {}", most_asleep_guard);
    println!("Most asleep minute: {}", most_asleep_minute);
    println!("{} * {} = {}", most_asleep_guard, most_asleep_minute, most_asleep_guard as usize * most_asleep_minute);

    let (guard, minute) = time_table.most_frequent_guard_on_minute();

    println!("---Strategy 2---");
    println!("Most frequently asleep guard on a minute: {}", guard);
    println!("The minute: {}", minute);
    println!("{} * {} = {}", guard, minute, guard as usize * minute);
}
