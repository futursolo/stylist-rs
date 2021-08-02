use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

static RNG: Lazy<Arc<Mutex<SmallRng>>> =
    Lazy::new(|| Arc::new(Mutex::new(SmallRng::from_entropy())));

pub(crate) fn get_rand_str() -> String {
    let mut rng = RNG.lock().expect("Failed to lock Rng.");

    (&mut *rng)
        .sample_iter(Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
