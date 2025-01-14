/*
* Copyright (C) 2019-2021 TON Labs. All Rights Reserved.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

use ever_crypto::Ed25519KeyOption;
use ton_types::Result;
     
fn gen() -> Result<()> {
    let (private, public) = Ed25519KeyOption::generate_with_json()?;
    let private = serde_json::to_value(private)?;
    println!("{:#}", serde_json::json!({
        "private": {
            "type_id": Ed25519KeyOption::KEY_TYPE,
            "pvt_key": private["pvt_key"],
        },
        "public": {
            "type_id": Ed25519KeyOption::KEY_TYPE,
            "pub_key": base64::encode(public.pub_key()?),
        },
        "keyhash": base64::encode(public.id().data()),
        "hex": {
            "secret": hex::encode(base64::decode(private["pvt_key"].as_str().unwrap())?),
            "public": hex::encode(public.pub_key()?)
        }
    }));
    Ok(())
} 

fn main() {
    gen().unwrap_or_else(|e| println!("Keypair generation error: {}", e))
}
