use serde_json::Value;

/// Represents a voter participating in an election.
#[derive(Debug, Clone)]
pub struct Voter {
    /// The unique identifier of the voter.
    pub(crate) id: u64,
    /// The name of the voter.
    pub(crate) name: String,
    /// Indicates whether the voter has cast a vote.
    pub(crate) vote_given: bool,
    /// Additional data associated with the voter (in JSON format).
    pub(crate) data: serde_json::Value,
    /// The password associated with the voter.
    pub(crate) password: String,
}

impl Voter {
    /// Creates a new `Voter` instance with the specified ID, name, vote status, data, and password.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the voter.
    /// * `name` - The name of the voter.
    /// * `vote_given` - Indicates whether the voter has cast a vote.
    /// * `data` - Additional data associated with the voter (in JSON format).
    /// * `password` - The password associated with the voter.
    ///
    /// # Returns
    ///
    /// A new `Voter` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    ///
    /// let voter_data = json!({
    ///     "age": 30,
    ///     "city": "New York"
    /// });
    /// let voter = Voter::new(123, "Alice".to_string(), false, voter_data, "password123".to_string());
    /// ```
    pub fn new(id: u64, name: String, vote_given: bool, data: Value, password: String) -> Self {
        Voter {
            id,
            name,
            vote_given,
            data,
            password,
        }
    }
}
