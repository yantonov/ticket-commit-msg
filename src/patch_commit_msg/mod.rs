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
            let mut found = false;
            let mut first_comment_line = None;
            let mut first_service_data_line = None;
            for (index, line) in lines.iter().enumerate() {
                if is_empty_line(line) {
                    first_comment_line = None;
                    first_service_data_line = None;
                } else {
                    if is_comment_line(line) {
                        if first_comment_line == None {
                            first_comment_line = Some(index);
                        }
                    } else {
                        if is_service_data_line(line) {
                            if first_service_data_line == None {
                                first_service_data_line = Some(index);
                            }
                        }
                        if line.contains(ticket) {
                            found = true;
                            break;
                        }
                    }
                }
            }
            if !found {
                let new_line = format!("{}{}",
                                       prepare_prefix(ticket_prefix
                                           .clone()
                                           .unwrap_or("".to_string())),
                                       ticket.clone());
                match first_service_data_line {
                    Some(index) => {
                        lines.insert(index, new_line);
                    }
                    None => {
                        match first_comment_line {
                            Some(index) => {
                                lines.insert(index, new_line);
                            }
                            None => {
                                lines.push(new_line);
                            }
                        }
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
    fn ticket_number_is_undefined_commit_message_remain_unchanged() {
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
    fn commit_message_does_not_contain_ticket_number_add_ticket_number() {
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
    fn commit_message_does_not_contain_ticket_number_add_ticket_number_with_prefix() {
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
    fn ticket_number_is_found_inside_commit_message_commit_message_remain_unchanged() {
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
    fn ticket_number_is_found_only_inside_commented_line_add_it_to_the_commit_message() {
        let result = patch_commit_msg(
            &vector_of_string(
                vec![
                    "1",
                    "# comment ISSUE-123"]),
            &Some("ISSUE-123".to_string()),
            &None);
        assert_eq!(3, result.len());
        assert_eq!("1", result.get(0).unwrap());
        assert_eq!("ISSUE-123", result.get(1).unwrap());
        assert_eq!("# comment ISSUE-123", result.get(2).unwrap());
    }

    #[test]
    fn prefix_contains_eoln_wait_eoln_is_removed() {
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

    fn vector_of_string(v: Vec<&str>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for item in v.iter() {
            result.push(item.clone().to_string());
        }
        result
    }
}