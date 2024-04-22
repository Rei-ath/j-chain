use serde_json::json;

use crate::{
    bc::block_chain::Blockchain, handler::Credentials, mote::{vote::Vote, voter::Voter}, vs::route
};



pub fn test_2(){
    let mut bc = Blockchain::new(3);
    // println!("{:?}",bc.candidates);
    let data = json!({
        "age":0
    });
    let voter = Voter {
        id: 1,
        name: "rei".to_string(),
        data,
        vote_given: false,
        password:"idkidkidk".to_string()
    };
    let c = Credentials{
        action:"login".to_string(),
        username:"rei".to_string(),
        password:"idkidkidk".to_string(),
        role_id:1
    };
    bc.register_user(voter.name.clone(), voter.clone());
    bc.registered_users.insert("rei".to_owned());
    // bc.add_registered_user(voter.name.clone());
    let res = route(c);
    println!("{:?}",bc)
}

// pub fn test() {
//     let mut blockchain = Blockchain::new(2);
//     let voter1 = Voter {
//         id: 1,
//         name: "rei1".to_string(),
//         data: json!({"age":12}),
//     };
//     let voter2 = Voter {
//         id: 1,
//         name: "rei2".to_string(),
//         data: json!({"age":441}),
//     };
//     let voter3 = Voter {
//         id: 1,
//         name: "rei3".to_string(),
//         data: json!({"age":312}),
//     };
//     // // Simulate voting
//     let vote1 = Vote {
//         candidate_name: String::from("Candidate A"),
//         voter: voter1,
//         signature: String::from("signature1"),
//     };
//     let vote2 = Vote {
//         candidate_name: String::from("Candidate B"),
//         voter: voter2,
//         signature: String::from("signature2"),
//     };
//     let _vote3 = Vote {
//         candidate_name: String::from("Candidate A"),
//         voter: voter3,
//         signature: String::from("signature3"),
//     };
//     blockchain.mine_block("next block".to_string(), vote1);
//     blockchain.mine_block("next 212".to_string(), vote2);
//     println!("{:?}", blockchain);
//     blockchain.display_votes();
// }
