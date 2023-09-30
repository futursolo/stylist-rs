use std::rc::Rc;

use gloo_events::EventListener;
use yew::prelude::*;
use yew::suspense::SuspensionResult;

use crate::arch::window;

/// A hook to provide media query.
///
/// This hook will return the result of whether the provided query matches and updates when the
/// result changes.
///
/// # Panics
///
/// This hook will panic if it is used in an environment without access to Web APIs.
#[cfg(feature = "yew_use_media_query")]
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let query: Rc<str> = Rc::from(query);
    let match_media = {
        let query = query.clone();

        move || {
            window()
                .ok()
                .and_then(|m| m.match_media(&query.clone()).ok())
                .flatten()
                .expect("Failed to query media")
        }
    };

    let state = use_state_eq(|| match_media().matches());
    let state_clone = state.clone();

    // Hold listener until end of component cycle.
    use_effect_with(query, move |_| {
        let match_media = match_media();
        let match_media_clone = match_media.clone();

        let listener = EventListener::new(&match_media, "change", move |_event| {
            state_clone.set(match_media_clone.matches());
        });

        move || {
            drop(listener);
        }
    });

    *state
}

/// A hook to provide media query with default values delivered from SSR.
///
/// This hook will return the result of whether the provided query matches and updates when the
/// result changes.
///
/// If the component has a value provided during SSR, it will use the prepared value before
/// switching to matched value.
///
/// # Panics
///
/// This hook will panic if it is used in an environment without access to Web APIs.
#[cfg(feature = "yew_use_media_query")]
#[hook]
pub fn use_prepared_media_query(query: &str, fallback: bool) -> SuspensionResult<bool> {
    let query: Rc<str> = Rc::from(query);
    let try_match_media = {
        let query = query.clone();

        move || {
            window()
                .ok()
                .and_then(|m| m.match_media(&query.clone()).ok())
                .flatten()
        }
    };

    // We only block with fallback if the current component is rendered with SSR, which this hook
    // will return Some(fallback).
    let prepared_fallback = use_prepared_state!((), |_| -> bool { fallback })?;

    let state = use_state_eq(|| {
        prepared_fallback
            .as_deref()
            .cloned()
            .or_else(|| try_match_media().map(|m| m.matches()))
            .unwrap_or(fallback)
    });
    let state_clone = state.clone();

    // Hold listener until end of component cycle.
    use_effect_with(query, move |_| {
        // Effects are only run during CSR, so this should not panic.
        let match_media = try_match_media().expect("Failed to query media");
        let match_media_clone = match_media.clone();

        // We set the media query again so it loads the actual value.
        // As this stage, hydration should already complete.
        state_clone.set(match_media.matches());

        let listener = EventListener::new(&match_media, "change", move |_event| {
            state_clone.set(match_media_clone.matches());
        });

        move || {
            drop(listener);
        }
    });

    Ok(*state)
}
