use common::{models::challenge::{md5::{MD5HashcashInput}, ChallengeTrait}, challenges::md5::Md5};

#[test]
fn md5_challenge() {
    let input = MD5HashcashInput {
        complexity: 16,
        message: "hello world".to_string(),
    };
    let challenge = Md5::new(input);
    let solution = challenge.solve();
    assert!(challenge.verify(&solution));
}