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
    let mut found = false;
    for line in lines.iter() {
        if !is_comment_line(line) && line.contains(ticket_number) {
            found = true;
            break;
        }
    }
    found
}

fn try_find_insert_position(lines: &Vec<String>) -> Option<usize> {
    let lines_count = lines.len();
    let mut index = (lines_count - 1) as i32;
    while index >= 0 {
        let line = lines.get(index as usize).unwrap();
        if is_comment_line(line) || is_empty_line(line) {
            index -= 1;
        } else {
            if is_service_data_line(line) {
                return Some(index as usize);
            } else {
                let next_index = (index + 1) as usize;
                if next_index < lines_count {
                    return Some(next_index);
                }
                break;
            }
        }
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
        assert_eq!(2, result.len());
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
        assert_eq!(3, result.len());
        assert_eq!("ISSUE-123", result.get(2).unwrap());
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
        assert_eq!(3, result.len());
        assert_eq!("PREFIX: ISSUE-123", result.get(2).unwrap());
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
        assert_eq!(2, result.len());
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
        assert_eq!(4, result.len());
        assert_eq!("1", result.get(0).unwrap());
        assert_eq!("ISSUE-123", result.get(1).unwrap());
        assert_eq!("# comment ISSUE-123", result.get(2).unwrap());
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
        assert_eq!(3, result.len());
        assert_eq!("PREFIX: ISSUE-123", result.get(2).unwrap());
    }

    #[test]
    fn add_ticket_number_before_service_data() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "Change-Id: 111222"]),
            &Some("ISSUE-123".to_string()),
            &None);
        assert_eq!(3, result.len());
        assert_eq!("1", result.get(0).unwrap());
        assert_eq!("ISSUE-123", result.get(1).unwrap());
        assert_eq!("Change-Id: 111222", result.get(2).unwrap());
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
        assert_eq!(9, result.len());
        assert_eq!("ISSUE-123", result.get(4).unwrap());
        assert_eq!("Change-Id: 333444", result.get(5).unwrap());
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
        assert_eq!(11, result.len());
        assert_eq!("ISSUE-123", result.get(4).unwrap());
        assert_eq!("Change-Id: 333444", result.get(5).unwrap());
    }

    fn vector_of_string(v: Vec<&str>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for item in v.iter() {
            result.push(item.clone().to_string());
        }
        result
    }
}