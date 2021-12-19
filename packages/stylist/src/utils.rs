#[cfg(feature = "random")]
fn get_rand_str() -> String {
    use std::iter::repeat_with;

    repeat_with(fastrand::alphanumeric).take(8).collect()
}

#[cfg(any(test, not(feature = "random")))]
pub(crate) fn get_next_style_id() -> String {
    use once_cell::sync::Lazy;
    use std::sync::atomic::{AtomicU64, Ordering};

    static CTR: Lazy<AtomicU64> = Lazy::new(AtomicU64::default);

    let ctr = CTR.fetch_add(1, Ordering::SeqCst);
    format!("style-{}", ctr)
}

pub(crate) fn get_entropy() -> String {
    #[cfg(feature = "random")]
    let entropy = get_rand_str();
    #[cfg(not(feature = "random"))]
    let entropy = get_next_style_id();

    entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        // As long as everytime it yields a different id, it will be fine.
        assert_ne!(get_next_style_id(), get_next_style_id());
        assert_ne!(get_next_style_id(), get_next_style_id());
        assert_ne!(get_next_style_id(), get_next_style_id());
    }
}
