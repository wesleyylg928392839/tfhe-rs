use tfhe::boolean::prelude::*;
fn main() {
    // generate client/server keys
    let (client_key, server_key) = tfhe::boolean::gen_keys();
    // encrypt two booleans
    let ctxt1 = client_key.encrypt(true);
    let ctxt2 = client_key.encrypt(false);
    // homomorphic AND
    let ctxt_res = server_key.and(&ctxt1, &ctxt2);
    // decrypt result
    let res: bool = client_key.decrypt(&ctxt_res);
    println!("true AND false = {}", res);
}
