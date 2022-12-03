use gloo_events::EventListener;
use yew::prelude::*;

use crate::arch::window;

/// A hook to provide media query.
///
/// This hook will return the result of whether the provided query matches and updates when the
/// result changes.
#[cfg(feature = "yew_use_media_query")]
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let match_media = || {
        window()
            .ok()
            .and_then(|m| m.match_media(query).ok())
            .flatten()
            .expect("Failed to query media")
    };

    let state = use_state(|| match_media().matches());
    let state_clone = state.clone();

    // Cached until end of component cycle.
    use_state(move || {
        let match_media = match_media();
        let match_media_clone = match_media.clone();

        EventListener::new(&match_media, "change", move |_event| {
            state_clone.set(match_media_clone.matches());
        })
    });

    *state
}
