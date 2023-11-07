use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()>{
    let mut fx_key = [0u8; 64]; // 512 bits = 64 bytes
    rand::thread_rng().fill_bytes(&mut fx_key);

    println!("Generated Key:\n {fx_key:?} \n");

    let b_64 = base64_url::encode(&fx_key);
    println!("Base64 encoded Key:\n {b_64:?}");

    Ok(())
}

         