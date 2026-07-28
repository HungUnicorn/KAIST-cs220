//! Small problems.

use std::collections::{HashMap, HashSet};
use std::fmt;

/// Day of week.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayOfWeek {
    /// Sunday.
    Sun,
    /// Monday.
    Mon,
    /// Tuesday.
    Tue,
    /// Wednesday.
    Wed,
    /// Thursday.
    Thu,
    /// Friday.
    Fri,
    /// Saturday.
    Sat,
}

/// The next day of week.
///
/// `next_weekday(Thu)` is `Fri`; and `next_weekday(Fri)` is `Mon`.
pub fn next_weekday(day: DayOfWeek) -> DayOfWeek {
    match day {
        DayOfWeek::Mon => DayOfWeek::Tue,
        DayOfWeek::Tue => DayOfWeek::Wed,
        DayOfWeek::Wed => DayOfWeek::Thu,
        DayOfWeek::Thu => DayOfWeek::Fri,
        DayOfWeek::Fri | DayOfWeek::Sat | DayOfWeek::Sun => DayOfWeek::Mon,
    }
}

/// Given a list of integers, returns its median (when sorted, the value in the middle position).
///
/// For a data set `x` of `n` elements, the median can be defined as follows:
///
/// - If `n` is odd, the median is `(n+1)/2`-th smallest element of `x`.
/// - If `n` is even, the median is `(n/2)+1`-th smallest element of `x`.
///
/// For example, the following list of seven numbers,
///
/// ```ignore
/// vec![1, 3, 3, 6, 7, 8, 9]
/// ```
///
/// has the median of 6, which is the fourth value. And for this data set of eight numbers,
///
/// ```ignore
/// vec![1, 2, 3, 4, 5, 6, 8, 9]
/// ```
///
/// it has the median of 5, which is the fifth value.
///
/// Returns `None` if the list is empty.
pub fn median(values: Vec<isize>) -> Option<isize> {
    let n = values.len();
    if n == 0 {
        return None;
    }

    let mut sorted_values = values;
    sorted_values.sort();

    return Some(sorted_values[n / 2]);
}

/// Given a list of integers, returns its smallest mode (the value that occurs most often; a hash
/// map will be helpful here).
///
/// Returns `None` if the list is empty.
pub fn mode(values: Vec<isize>) -> Option<isize> {
    let mut counts = HashMap::new();
    for &val in &values {
        *counts.entry(val).or_insert(0) += 1;
    }

    let mut max_count = 0;
    let mut mode = None;

    for (val, count) in counts {
        if count > max_count {
            max_count = count;
            mode = Some(val);
        } else if count == max_count {
            if mode.is_none() || mode.unwrap() > val {
                mode = Some(val);
            }
        }
    }
    mode
}

/// Converts the given string to Pig Latin. Use the rules below to translate normal English into Pig
/// Latin.
///
/// 1. If a word starts with a consonant and a vowel, move the first letter of the word at the end
///    of the word and add "ay".
///
/// Example: "happy" -> "appyh" + "ay" -> "appyhay"
///
/// 2. If a word starts with multiple consonants, move them to the end of the word and add "ay".
///
/// Example: "string" -> "ingstr" + "ay" -> "ingstray"
///
/// 3. If a word starts with a vowel, add the word "hay" at the end of the word.
///
/// Example: "explain" -> "explain" + "hay" -> "explainhay"
///
/// Keep in mind the details about UTF-8 encoding!
///
/// You may assume the string only contains lowercase alphabets, and it contains at least one vowel.
pub fn piglatin(input: String) -> String {
    if input.starts_with(['a', 'e', 'i', 'o', 'u']) {
        return input + "hay";
    }

    let first_vowel_pos = input.find(['a', 'e', 'i', 'o', 'u']).unwrap();

    format!(
        "{}{}ay",
        &input[first_vowel_pos..],
        &input[..first_vowel_pos]
    )
}

/// Converts HR commands to the organization table.
///
/// If the commands are as follows:
///
/// ```ignore
/// vec!["Add Amir to Engineering", "Add Sally to Sales", "Remove Jeehoon from Sales", "Move Amir from Engineering to Sales"]
/// ```
///
/// The return value should be:
///
/// ```ignore
/// ["Sales" -> ["Amir", "Sally"]]
/// ```
///
/// - The result is a map from department to the list of its employees.
/// - An empty department should not appear in the result.
/// - There are three commands: "Add {person} to {department}", "Remove {person} from {department}",
///   and "Move {person} from {department} to {department}".
/// - If a command is not executable, then it's ignored.
/// - There is no space in the name of the person and department.
///
/// See the test function for more details.
pub fn organize(commands: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();

    for command in commands {
        let words: Vec<&str> = command.split_whitespace().collect();
        match words.as_slice() {
            ["Add", person, "to", department] => {
                let _ = result
                    .entry(department.to_string())
                    .or_default()
                    .insert(person.to_string());
            }
            ["Remove", person, "from", department] => {
                let _ = remove_person(&mut result, department, person);
            }
            ["Move", person, "from", dept1, "to", dept2] => {
                if remove_person(&mut result, dept1, person) {
                    let _ = result
                        .entry(dept2.to_string())
                        .or_default()
                        .insert(person.to_string());
                }
            }
            _ => {}
        }
    }

    result
}

fn remove_person(result: &mut HashMap<String, HashSet<String>>, dept: &str, person: &str) -> bool {
    let Some(employees) = result.get_mut(dept) else {
        return false;
    };

    let removed = employees.remove(person);
    if employees.is_empty() {
        let _unused = result.remove(dept);
    }
    removed
}

/// Events in a text editor.
#[derive(Debug)]
pub enum TypeEvent {
    /// A character is typed.
    Type(char),
    /// The last character is removed.
    Backspace,
    /// The whole string is copied to the clipboard.
    Copy,
    /// The string in the clipboard is appended.
    Paste,
}

/// Starting from an empty string and an empty clipboard,
/// processes the given `events` in order and returns the resulting string.
///
/// See the test function `test_editor` for examples.
pub fn use_editor(events: Vec<TypeEvent>) -> String {
    let mut text = String::new();
    let mut clipboard = String::new();

    for event in events {
        match event {
            TypeEvent::Type(c) => {
                text.push(c);
            }
            TypeEvent::Backspace => {
                let _ = text.pop();
            }
            TypeEvent::Copy => {
                clipboard = text.clone();
            }
            TypeEvent::Paste => {
                text.push_str(&clipboard);
            }
        }
    }

    text
}
