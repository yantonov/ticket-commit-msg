fn is_service_comment_line(line: &str) -> bool {
    line.starts_with("#")
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
            for (index, line) in lines.iter().enumerate() {
                if is_service_comment_line(line) {
                    if first_comment_line == None {
                        first_comment_line = Some(index);
                    }
                } else {
                    if line.contains(ticket) {
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                let new_line = format!("{}{}",
                                       ticket_prefix
                                           .clone()
                                           .unwrap_or("".to_string()),
                                       ticket.clone());
                match first_comment_line {
                    None => {
                        lines.push(new_line);
                    }
                    Some(index) => {
                        lines.insert(index, new_line);
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
            &vec![
                "1".to_string(),
                "2".to_string()],
            &None,
            &None);
        assert_eq!(2, result.len());
    }

    #[test]
    fn commit_message_does_not_contain_ticket_number_add_ticket_number() {
        let result = patch_commit_msg(
            &vec![
                "1".to_string(),
                "2".to_string()],
            &Some("ISSUE-123".to_string()),
            &None);
        assert_eq!(3, result.len());
        assert_eq!("ISSUE-123", result.get(2).unwrap());
    }

    #[test]
    fn commit_message_does_not_contain_ticket_number_add_ticket_number_with_prefix() {
        let result = patch_commit_msg(
            &vec![
                "1".to_string(),
                "2".to_string()],
            &Some("ISSUE-123".to_string()),
            &Some("ISSUE: ".to_string()));
        assert_eq!(3, result.len());
        assert_eq!("ISSUE: ISSUE-123", result.get(2).unwrap());
    }

    #[test]
    fn ticket_number_is_found_inside_commit_message_commit_message_remain_unchanged() {
        let result = patch_commit_msg(&vec![
            "1".to_string(),
            "blablabla ISSUE-123 blablabla".to_string()],
                                      &Some("ISSUE-123".to_string()),
                                      &None);
        assert_eq!(2, result.len());
    }

    #[test]
    fn ticket_number_is_found_only_inside_commented_line_add_it_to_the_commit_message() {
        let result = patch_commit_msg(&vec![
            "1".to_string(),
            "# comment ISSUE-123".to_string()],
                                      &Some("ISSUE-123".to_string()),
                                      &None);
        assert_eq!(3, result.len());
        assert_eq!("1", result.get(0).unwrap());
        assert_eq!("ISSUE-123", result.get(1).unwrap());
        assert_eq!("# comment ISSUE-123", result.get(2).unwrap());
    }
}