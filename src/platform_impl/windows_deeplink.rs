use crate::NotificationResponseAction;

pub(crate) fn decode_action_path(path: &str) -> NotificationResponseAction {
    match path {
        "/__default__" => NotificationResponseAction::Default,
        "/__dismiss__" => NotificationResponseAction::Dismiss,
        action => {
            NotificationResponseAction::Other(action.strip_prefix('/').unwrap_or(action).to_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_url_path_separator_from_custom_actions() {
        assert_eq!(
            decode_action_path("/reply"),
            NotificationResponseAction::Other("reply".to_owned()),
        );
    }

    #[test]
    fn preserves_nested_custom_action_segments() {
        assert_eq!(
            decode_action_path("/message/reply"),
            NotificationResponseAction::Other("message/reply".to_owned()),
        );
    }

    #[test]
    fn keeps_builtin_actions() {
        assert_eq!(
            decode_action_path("/__default__"),
            NotificationResponseAction::Default
        );
        assert_eq!(
            decode_action_path("/__dismiss__"),
            NotificationResponseAction::Dismiss
        );
    }
}
