use rand::distributions::Alphanumeric;
use rand::Rng;

pub(crate) fn get_rand_str() -> String {
    // We use rand instead of fastrand used in the client side as it generates more robust random
    // numbers than fastrand as it seeds the random number generator with system entropy and the
    // procedural macro does not subject to any size constriant.
    //
    // It's highly likely that some of your dependencies already depends on rand in some way.

    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect()
}
