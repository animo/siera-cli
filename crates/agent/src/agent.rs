/// Generic Aries cloudagent structure
#[derive(Debug)]
pub struct Agent<A> {
    /// Default cloud agent structure
    pub agent: A,

    /// version string, e.g. 0.7.3
    pub version: String,
}

impl<A> Agent<A> {
    /// Create a new instance of an agent
    pub fn new(agent: A, version: String) -> Self {
        Self { agent, version }
    }
}
