use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter};

use crate::utils::{clear_screen, wait_for_enter};

pub fn program() {
    clear_screen();
    execute();
    wait_for_enter();
}

#[derive(AsRefStr, EnumIter, PartialEq, PartialOrd, Eq, Ord)]
enum EDay {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
    Ninth,
    Tenth,
    Eleventh,
    Twelfth,
}

fn execute() {
    for current_day in EDay::iter() {
        println!(
            "On the {} day of Christmas,\n\
            My true love sent to me",
            current_day.as_ref().to_lowercase()
        );
        for day_to_get_gifts in EDay::iter().rev() {
            if day_to_get_gifts > current_day {
                continue;
            }
            let gifts = get_day_gifts(&day_to_get_gifts);
            println!(
                "{}",
                match day_to_get_gifts {
                    EDay::First => format!("{}.", gifts),
                    EDay::Second => format!("{gifts}, and"),
                    _ => format!("{gifts},"),
                }
            )
        }
        println!("");
    }
}

fn get_day_gifts<'a>(day: &EDay) -> &'a str {
    match day {
        EDay::First => "A partridge in a pear tree",
        EDay::Second => "Two turtle doves",
        EDay::Third => "Three French hens",
        EDay::Fourth => "Four calling birds",
        EDay::Fifth => "Five gold rings",
        EDay::Sixth => "Six geese a-laying",
        EDay::Seventh => "Seven swans a-swimming",
        EDay::Eighth => "Eight maids a-milking",
        EDay::Ninth => "Nine ladies dancing",
        EDay::Tenth => "Ten lords a-leaping",
        EDay::Eleventh => "Eleven pipers piping",
        EDay::Twelfth => "Twelve drummers drumming",
    }
}
