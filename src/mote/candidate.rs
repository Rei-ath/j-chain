/// Represents a candidate participating in an election.
pub struct Candidate {
    /// The name of the candidate.
    pub(crate) name: String,
    /// The unique identifier of the candidate.
    pub(crate) id: u64,
    /// The political party affiliation of the candidate.
    pub(crate) party: String,
}

impl Candidate {
    /// Creates a new `Candidate` instance with the specified name, ID, and party affiliation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the candidate.
    /// * `id` - The unique identifier of the candidate.
    /// * `party` - The political party affiliation of the candidate.
    ///
    /// # Returns
    ///
    /// A new `Candidate` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let candidate = Candidate::new("John Doe".to_string(), 1, "Independent".to_string());
    /// ```
    pub fn new(name: String, id: u64, party: String) -> Self {
        Candidate { name, id, party }
    }
}
