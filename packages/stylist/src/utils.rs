use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;

#[cfg(feature = "random")]
pub fn get_rand_str() -> String {
    use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

    static RNG: Lazy<Arc<Mutex<SmallRng>>> =
        Lazy::new(|| Arc::new(Mutex::new(SmallRng::from_entropy())));
    let mut rng = RNG.lock().expect("Failed to lock Rng.");

    (&mut *rng)
        .sample_iter(Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}

#[cfg(any(test, not(feature = "random")))]
pub fn get_next_style_id() -> String {
    static CTR: Lazy<Arc<Mutex<u64>>> = Lazy::new(Arc::default);
    let mut ctr = CTR.lock().expect("Failed to lock Rng.");

    *ctr += 1;
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
