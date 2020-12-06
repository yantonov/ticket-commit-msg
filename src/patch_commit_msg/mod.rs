pub fn patch_commit_msg(commit_msg: &Vec<String>,
                        ticket_number: &Option<String>) -> Vec<String> {
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
                lines.push(ticket.clone());
            }
        }
    };
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticket_number_is_undefined() {
        let result = patch_commit_msg(
            &vec![
                "1".to_string(),
                "2".to_string()],
            &None);
        assert_eq!(2, result.len());
    }

    #[test]
    fn ticket_number_not_found_inside_commit_message() {
        let result = patch_commit_msg(
            &vec![
                "1".to_string(),
                "2".to_string()],
            &Some("ISSUE-123".to_string()));
        assert_eq!(3, result.len());
        assert_eq!("ISSUE-123", result.get(2).unwrap());
    }

    #[test]
    fn ticket_number_is_found_inside_commit_message() {
        let result = patch_commit_msg(&vec![
            "1".to_string(),
            "blablabla ISSUE-123 blablabla".to_string()],
                                      &Some("ISSUE-123".to_string()));
        assert_eq!(2, result.len());
    }
}