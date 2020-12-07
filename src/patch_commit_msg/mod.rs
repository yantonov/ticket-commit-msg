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
            for line in lines.iter() {
                if line.contains(ticket) {
                    found = true;
                    break;
                }
            }
            if !found {
                lines.push(format!("{}{}",
                                   ticket_prefix
                                       .clone()
                                       .unwrap_or("".to_string()),
                                   ticket.clone()));
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
}