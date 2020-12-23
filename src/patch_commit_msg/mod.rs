use regex::Regex;

fn is_empty_line(line: &str) -> bool {
    line.len() == 0
}

fn is_comment_line(line: &str) -> bool {
    line.starts_with("#")
}

fn is_service_data_line(line: &str) -> bool {
    let re = Regex::new(r"^[A-Za-z_0-9-]+:.*").unwrap();
    re.is_match(line)
}

fn prepare_prefix(prefix: String) -> String {
    let re = Regex::new(r"^(.*[^\r\n])([\r\n]*)$").unwrap();
    re.replace(&prefix, "$1").to_string()
}

fn try_find_ticket_number(lines: &Vec<String>, ticket_number: &String) -> bool {
    lines.iter()
        .find(|line|
            !is_comment_line(line)
                && line.contains(ticket_number)).is_some()
}

fn try_find_insert_position(lines: &Vec<String>) -> Option<usize> {
    let lines_count = lines.len();
    let mut index = (lines_count - 1) as i32;
    let mut service_info_line = None;
    while index >= 0 {
        let line = lines.get(index as usize).unwrap();
        if is_comment_line(line) || is_empty_line(line) {
            if service_info_line != None {
                return service_info_line;
            }
        } else {
            if is_service_data_line(line) {
                service_info_line = Some(index as usize);
            } else {
                if service_info_line != None {
                    return service_info_line;
                }
                let next_index = (index + 1) as usize;
                if next_index < lines_count {
                    return Some(next_index);
                }
                return None;
            }
        }
        index -= 1;
    }
    None
}

pub fn patch_commit_msg(commit_msg: &Vec<String>,
                        ticket_number: &Option<String>,
                        ticket_prefix: &Option<String>) -> Vec<String> {
    let mut lines: Vec<String> = commit_msg
        .into_iter()
        .map(|x| x.clone())
        .collect();
    match ticket_number {
        None => {}
        Some(ticket) => {
            let ticket_number_found = try_find_ticket_number(&lines, ticket);
            if !ticket_number_found {
                let position = try_find_insert_position(&lines);
                let new_line = format!("{}{}",
                                       prepare_prefix(ticket_prefix
                                           .clone()
                                           .unwrap_or("".to_string())),
                                       ticket.clone());
                match position {
                    Some(index) => {
                        lines.insert(index, new_line);
                    }
                    None => {
                        lines.push(new_line);
                    }
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
                "2"
            ]);
        assert_eq!(&expected, &result);
    }

    #[test]
    fn commit_message_does_not_contain_ticket_number_expect_adding_ticket_number() {
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
                "ISSUE-123"
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn commit_message_does_not_contain_ticket_number_expect_adding_ticket_number_with_prefix() {
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
                "PREFIX: ISSUE-123"
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_inside_commit_message_expect_do_nothing() {
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
                "blablabla ISSUE-123 blablabla"
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn ticket_number_is_found_only_inside_commented_line_expect_adding_ticket_number() {
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
                "ISSUE-123",
                "# comment ISSUE-123",
                "# another comment line"
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
                "PREFIX: ISSUE-123"
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
                "ISSUE-123",
                "Change-Id: 111222",
                "Another-Service_Info: 333444"
            ]
        );
        assert_eq!(&expected, &result);
    }

    #[test]
    fn prefer_inserting_to_the_end_of_the_file() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "Change-Id: 111222",
                    "# tmp line",
                    "",
                    "Change-Id: 333444",
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
                "# large",
                "# commented",
                "# block"
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
                "# block"
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