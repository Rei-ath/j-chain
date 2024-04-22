use crate::bc::block_chain::Blockchain;
use crate::handler::Credentials;
use crate::mote::voter::Voter;
use crate::BLOCKCHAIN;

/// Routes incoming requests based on the provided credentials.
///
/// Processes incoming requests by delegating them to appropriate handlers based on the action
/// specified in the credentials.
///
/// # Arguments
///
/// * `c` - The credentials associated with the incoming request.
///
/// # Returns
///
/// A `String` indicating the result of processing the request.
pub fn route(c: Credentials) -> String {
    if c.action == "login" {
        let res = handle_login(c);
        res
    } else {
        "success".to_string()
    }
}

/// Handles login requests.
///
/// Validates login credentials against the blockchain and returns the result.
///
/// # Arguments
///
/// * `c` - The credentials associated with the login request.
///
/// # Returns
///
/// A `String` indicating the result of the login attempt.
fn handle_login(c: Credentials) -> String {
    let bc = BLOCKCHAIN.lock().unwrap();
    if bc.voters.contains_key(&c.username) && c.role_id == 1 {
        let voter = bc.voters.get(&c.username);
        if voter.unwrap().password == c.password {
            "success".to_string()
        } else {
            "failure".to_string()
        }
    } else {
        "failure".to_string()
    }
}

/// Registers a new candidate.
pub fn register_candidate() {}

/// Registers a new voter.
pub fn register_voter() {}

/// Casts a vote.
pub fn cast_vote() {}

/// Retrieves the count of votes.
pub fn get_vote_count() {}
