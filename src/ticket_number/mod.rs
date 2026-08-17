// Equivalent to matching ^(.*/|)([A-Z0-9]+-[0-9]+)[^/]*$ against the branch
// name: the ticket key sits at the start of the last '/'-delimited segment,
// with any remaining suffix (other than another '/') ignored.
pub fn ticket_number(branch: &str) -> Option<String> {
    let segment = branch.rsplit('/').next().unwrap_or(branch);
    let bytes = segment.as_bytes();

    let key_end = bytes
        .iter()
        .position(|b| !(b.is_ascii_uppercase() || b.is_ascii_digit()))
        .unwrap_or(bytes.len());
    if key_end == 0 || bytes.get(key_end) != Some(&b'-') {
        return None;
    }

    let digits_start = key_end + 1;
    let digits_end = bytes[digits_start..]
        .iter()
        .position(|b| !b.is_ascii_digit())
        .map(|offset| digits_start + offset)
        .unwrap_or(bytes.len());
    if digits_end == digits_start {
        return None;
    }

    Some(segment[..digits_end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_scenario_branch_name_is_equal_to_ticket_name() {
        assert_eq!(
            "PROJECTQUEUE-1234",
            ticket_number("PROJECTQUEUE-1234").unwrap()
        );
    }

    #[test]
    fn omit_branch_suffix_after_ticket_number() {
        assert_eq!(
            "PROJECTQUEUE-1234",
            ticket_number("PROJECTQUEUE-1234_one_more_pull_request").unwrap()
        );
        assert_eq!(
            "PROJECTQUEUE-1234",
            ticket_number("PROJECTQUEUE-1234-one-more-pull-request").unwrap()
        );
    }

    #[test]
    fn omit_user_prefix() {
        assert_eq!(
            "PROJECTQUEUE-1234",
            ticket_number("users/username/PROJECTQUEUE-1234").unwrap()
        );
    }

    #[test]
    fn release_branch() {
        assert!(ticket_number("users/username/project/release/1.2.3.4").is_none());
    }
}
