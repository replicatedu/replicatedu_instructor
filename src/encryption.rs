use sodiumoxide::crypto::box_;
use hex;


// pub fn gen_keypair()->(String, String){
// 	let (ourpk, oursk) = box_::gen_keypair();
// 	//dbg!(hex::encode(&ourpk));
// 	//println!("{}",hex::encode(&oursk[..]));
// 	let ret_str = String::from(hex::encode(oursk[..]).clone());
// 	(hex::encode(ourpk), ret_str)
// }

// pub fn load_keypair(sk: &str)-> box_::SecretKey {

// }

//pub fn 

#[cfg(test)]
mod tests {
    use super::*;
	use sodiumoxide::crypto::box_;

    #[test]
    fn encrypt_decrypt() {
        let (ourpk, oursk) = box_::gen_keypair();
		
        // normally theirpk is sent by the other party
		let (theirpk, theirsk) = box_::gen_keypair();
		let nonce = box_::gen_nonce();
		let plaintext = b"some data";
		let ciphertext = box_::seal(plaintext, &nonce, &theirpk, &oursk);
		let their_plaintext = box_::open(&ciphertext, &nonce, &ourpk, &theirsk).unwrap();
		assert!(plaintext == &their_plaintext[..]);

    }
}
