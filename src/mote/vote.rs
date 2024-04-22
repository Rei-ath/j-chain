use crate::mote::voter::Voter;

/// Represents a vote cast by a voter for a candidate.
#[derive(Debug, Clone)]
pub(crate) struct Vote {
    /// The voter who cast the vote.
    pub(crate) voter: Voter,
    /// The name of the candidate voted for.
    pub(crate) candidate_name: String,
    /// The signature associated with the vote.
    pub(crate) signature: String,
}

impl Vote {
    /// Creates a new `Vote` instance with the specified voter, candidate name, and signature.
    ///
    /// # Arguments
    ///
    /// * `voter` - The voter who cast the vote.
    /// * `candidate_name` - The name of the candidate voted for.
    /// * `signature` - The signature associated with the vote.
    ///
    /// # Returns
    ///
    /// A new `Vote` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// let voter = Voter::new("Alice".to_string(), 123);
    /// let vote = Vote::new(voter, "Bob".to_string(), "abc123".to_string());
    /// ```
    pub fn new(voter: Voter, candidate_name: String, signature: String) -> Self {
        Vote {
            voter,
            candidate_name,
            signature,
        }
    }
}

