#[derive(Clone, Default)]
pub struct Draw {
    pub(crate) inner: nannou::Draw,
}

// Note: This is inherently unsafe but required for multi-threading sketches
// We must be careful to never access a draw instance from multiple threads at the same time
unsafe impl Send for Draw {}
unsafe impl Sync for Draw {}
