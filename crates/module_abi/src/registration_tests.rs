use super::*;
use crate::events;

fn event(name: &str) -> EventReg {
    EventReg {
        name: name.to_string(),
        handler: "h".to_string(),
        priority: Priority::Normal,
    }
}

fn route(method: &str, path: &str) -> RouteReg {
    RouteReg {
        method: method.to_string(),
        path: path.to_string(),
        handler: "h".to_string(),
        auth: Auth::User,
    }
}

#[test]
fn an_empty_registration_is_valid() {
    assert!(Registration::default().violations().is_empty());
}

/// The declaration is brought in by somebody else's code: even with complete
/// trust in the author a typo stays a typo, and a dead handler is better caught
/// at install time.
#[test]
fn an_unknown_event_is_rejected() {
    let r = Registration {
        events: vec![event("player.teleported")],
        ..Default::default()
    };
    assert_eq!(r.violations().len(), 1);

    let r = Registration {
        events: vec![event(events::EV_PLAYER_JOINED)],
        ..Default::default()
    };
    assert!(r.violations().is_empty());
}

#[test]
fn routes_are_checked_for_shape() {
    let r = Registration {
        routes: vec![route("FETCH", "top")],
        ..Default::default()
    };
    // Two violations: the method, and a path without a slash.
    assert_eq!(r.violations().len(), 2);
}

/// Two identical endpoints raise the question of which one fires, and there is
/// no good answer to it.
#[test]
fn a_duplicate_route_is_rejected() {
    let r = Registration {
        routes: vec![route("GET", "/me"), route("get", "/me")],
        ..Default::default()
    };
    assert_eq!(r.violations().len(), 1);

    // The same path with a different method is a different endpoint; that is fine.
    let r = Registration {
        routes: vec![route("GET", "/me"), route("POST", "/me")],
        ..Default::default()
    };
    assert!(r.violations().is_empty());
}

#[test]
fn task_intervals_are_checked() {
    let r = Registration {
        tasks: vec![TaskReg {
            handler: "h".to_string(),
            every: "an hour or so".to_string(),
        }],
        ..Default::default()
    };
    assert_eq!(r.violations().len(), 1);
}

/// The handler order is the one familiar from Bukkit: the deciders run after
/// the watchers.
#[test]
fn subscriptions_come_back_in_priority_order() {
    let mut high = event(events::EV_PLAYER_JOINED);
    high.handler = "late".to_string();
    high.priority = Priority::High;

    let mut low = event(events::EV_PLAYER_JOINED);
    low.handler = "early".to_string();
    low.priority = Priority::Lowest;

    let r = Registration {
        events: vec![high, low, event(events::EV_PLAYER_LEFT)],
        ..Default::default()
    };

    let order: Vec<&str> = r
        .subscriptions(events::EV_PLAYER_JOINED)
        .iter()
        .map(|e| e.handler.as_str())
        .collect();
    assert_eq!(order, vec!["early", "late"]);
    assert_eq!(r.subscriptions(events::EV_PLAYER_LEFT).len(), 1);
}

#[test]
fn a_route_is_found_by_method_and_path() {
    let r = Registration {
        routes: vec![route("GET", "/me")],
        ..Default::default()
    };
    assert!(r.route("get", "/me").is_some());
    assert!(r.route("POST", "/me").is_none());
    assert!(r.route("GET", "/other").is_none());
}
