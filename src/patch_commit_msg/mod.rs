use regex::{Regex};
use crate::patch_commit_msg::PatchResult::{Append, DoNothing, Insert};

struct LineDetector {
    prefix: String,
}

impl LineDetector {
    pub fn new(prefix: &str) -> LineDetector {
        return LineDetector {
            prefix: prefix.to_string()
        };
    }

    pub fn is_empty_line(&self, line: &str) -> bool {
        line.is_empty()
    }

    fn is_comment_line(&self, line: &str) -> bool {
        line.starts_with('#')
    }

    fn is_service_data_line(&self, line: &str) -> bool {
        // TODO: do not instantiate regex every time
        let re = Regex::new(r"^[A-Za-z_0-9-]+:.*").unwrap();
        return re.is_match(line);
    }

    fn is_ticket_number_line(&self, line: &str) -> bool {
        // TODO: do not instantiate regex every time
        let re = Regex::new(&format!("^{}[A-Z]+-[0-9-]+$", self.prefix)).unwrap();
        return re.is_match(line);
    }
}

enum PatchResult {
    DoNothing,
    Insert(usize),
    Append,
}

fn prepare_prefix(prefix: String) -> String {
    let re = Regex::new(r"^(.*[^\r\n])([\r\n]*)$").unwrap();
    re.replace(&prefix, "$1").to_string()
}

fn try_patch(lines: &[String], prefix: &str, ticket_number: &str) -> PatchResult {
    let line_detector = LineDetector::new(prefix);
    let lines_count = lines.len();
    let mut index = (lines_count - 1) as i32;
    let mut service_info_line = None;
    let mut non_empty_line_found = false;
    while index >= 0 {
        let line = lines.get(index as usize).unwrap();
        if line_detector.is_comment_line(line) {
            index -= 1;
            continue;
        }
        if line_detector.is_empty_line(line) {
            if non_empty_line_found {
                break;
            } else {
                index -= 1;
                continue;
            }
        }
        if line_detector.is_service_data_line(line) || line_detector.is_ticket_number_line(line) {
            if line.eq(ticket_number) {
                return DoNothing;
            }
            non_empty_line_found = true;
            service_info_line = Some(index as usize);
            index -= 1;
            continue;
        }
        break;
    }
    match service_info_line {
        None => Append,
        Some(v) => Insert(v)
    }
}

pub fn patch_commit_msg(commit_msg: &[String],
                        ticket_number: &Option<String>,
                        ticket_prefix: &Option<String>) -> Vec<String> {
    let mut lines: Vec<String> = commit_msg.to_vec();
    match ticket_number {
        None => {}
        Some(ticket) => {
            let prepared_prefix = prepare_prefix(ticket_prefix
                .clone()
                .unwrap_or_else(|| "".to_string()));
            let new_line = format!("{}{}",
                                   prepared_prefix,
                                   ticket.clone());
            let patch_result = try_patch(&lines, &prepared_prefix, &new_line);
            match patch_result {
                DoNothing => {}
                Insert(index) => {
                    lines.insert(index, new_line);
                    if index > 0 && lines.get(index - 1).unwrap() != "" {
                        lines.insert(index, "".to_string());
                    }
                }
                Append => {
                    lines.push("".to_string());
                    lines.push(new_line);
                }
            }
        }
    };
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticket_number_is_undefined_expect_do_nothing() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "2"]),
            &None,
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "2",
            ]);
        assert_eq!(&expected, &result);
    }

    #[test]
    fn empty_prefix_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "2"]),
            &Some("ISSUE-123".to_string()),
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "2",
                "",
                "ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn non_empty_prefix_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "2"]),
            &Some("ISSUE-123".to_string()),
            &Some("PREFIX: ".to_string()));
        let expected = vector_of_string(
            vec![
                "1",
                "2",
                "",
                "PREFIX: ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_inside_the_text_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "blablabla ISSUE-123 blablabla"]),
            &Some("ISSUE-123".to_string()),
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "blablabla ISSUE-123 blablabla",
                "",
                "ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_along_with_prefix_but_with_different_suffix_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "Ticket ISSUE-123 unknown suffix"]),
            &Some("ISSUE-123".to_string()),
            &Some("Ticket ".to_string()));
        let expected = vector_of_string(
            vec![
                "1",
                "Ticket ISSUE-123 unknown suffix",
                "",
                "Ticket ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_as_service_line_with_prefix_but_with_different_suffix_expect_insert() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "Ticket: ISSUE-123 unknown suffix"]),
            &Some("ISSUE-123".to_string()),
            &Some("Ticket: ".to_string()));
        let expected = vector_of_string(
            vec![
                "1",
                "",
                "Ticket: ISSUE-123",
                "Ticket: ISSUE-123 unknown suffix",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_but_with_different_prefix_and_without_suffix_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "",
                    "another prefix ISSUE-123"]),
            &Some("ISSUE-123".to_string()),
            &Some("Ticket: ".to_string()));
        let expected = vector_of_string(
            vec![
                "1",
                "",
                "another prefix ISSUE-123",
                "",
                "Ticket: ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_without_prefix_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "",
                    "ISSUE-123"]),
            &Some("ISSUE-123".to_string()),
            &Some("Ticket: ".to_string()));
        let expected = vector_of_string(
            vec![
                "1",
                "",
                "ISSUE-123",
                "",
                "Ticket: ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_only_inside_commented_line_expect_append() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "# comment ISSUE-123",
                    "# another comment line"]),
            &Some("ISSUE-123".to_string()),
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "# comment ISSUE-123",
                "# another comment line",
                "",
                "ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn prefix_contains_eoln_expect_eoln_is_removed() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "2"]),
            &Some("ISSUE-123".to_string()),
            &Some("PREFIX: \r\n\r\n\r".to_string()));
        let expected = vector_of_string(
            vec![
                "1",
                "2",
                "",
                "PREFIX: ISSUE-123",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn add_ticket_number_before_service_data() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "Change-Id: 111222",
                    "Another-Service_Info: 333444"]),
            &Some("ISSUE-123".to_string()),
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "",
                "ISSUE-123",
                "Change-Id: 111222",
                "Another-Service_Info: 333444",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ignore_trailing_empty_lines_after_service_lines() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "Change-Id: 111222",
                    "# tmp line",
                    "",
                    "Change-Id: 333444",
                    "",
                    "",
                    "# large",
                    "# commented",
                    "# block"]),
            &Some("ISSUE-123".to_string()),
            &None);

        let expected = vector_of_string(
            vec![
                "1",
                "Change-Id: 111222",
                "# tmp line",
                "",
                "ISSUE-123",
                "Change-Id: 333444",
                "",
                "",
                "# large",
                "# commented",
                "# block",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn patch_only_the_last_block_of_service_lines() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "ISSUE-123",
                    "Change-Id: 111222",
                    "",
                    "Another-Service_Info: 333444"]),
            &Some("ISSUE-123".to_string()),
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "ISSUE-123",
                "Change-Id: 111222",
                "",
                "ISSUE-123",
                "Another-Service_Info: 333444",
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn last_service_block_contains_ticket_number_expect_do_nothing() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "ISSUE-123",
                    "Change-Id: 111222",
                    "",
                    "ISSUE-124",
                    "Another-Service_Info: 333444"]),
            &Some("ISSUE-123".to_string()),
            &None);
        let expected = vector_of_string(
            vec![
                "1",
                "ISSUE-123",
                "Change-Id: 111222",
                "",
                "ISSUE-123",
                "ISSUE-124",
                "Another-Service_Info: 333444",
            ]
        );
        assert_eq!(&expected, &result);
    }

    fn vector_of_string(v: Vec<&str>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for item in v.iter() {
            result.push(item.clone().to_string());
        }
        result
    }
}