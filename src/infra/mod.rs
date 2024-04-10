use std::sync::{Arc, Mutex};

use rand_chacha::ChaCha8Rng;

pub mod db;
pub mod services;

pub type Random = Arc<Mutex<ChaCha8Rng>>;

