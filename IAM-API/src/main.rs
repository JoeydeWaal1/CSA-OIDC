use jwks_client::error::Error;
use jwks_client::keyset::KeyStore;

#[tokio::main]
async fn main() {
    let jkws_url = "https://dev-uzo17fcir336ze1h.us.auth0.com/.well-known/jwks.json";
    let key_set = KeyStore::new_from(jkws_url).await.unwrap();

    // ...

    let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjVHMHhJaDJHeldJZDhGTWl5TTJqdSJ9.eyJnaXZlbl9uYW1lIjoiSm9leSIsImZhbWlseV9uYW1lIjoiZGUgV2FhbCIsIm5pY2tuYW1lIjoiZGV3YWFsam9leSIsIm5hbWUiOiJKb2V5IGRlIFdhYWwiLCJwaWN0dXJlIjoiaHR0cHM6Ly9saDMuZ29vZ2xldXNlcmNvbnRlbnQuY29tL2EvQUNnOG9jSy1aTTRzVFItdmZmOU15U3BIc3RBVmtDYU05RG9XaVRNWVRISHBzbk9KZVFVPXM5Ni1jIiwibG9jYWxlIjoibmwiLCJ1cGRhdGVkX2F0IjoiMjAyMy0xMS0wNlQwOTo0MzoyMC4zMDZaIiwiZW1haWwiOiJkZXdhYWxqb2V5QGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJpc3MiOiJodHRwczovL2Rldi11em8xN2ZjaXIzMzZ6ZTFoLnVzLmF1dGgwLmNvbS8iLCJhdWQiOiJJS3pocjNrVFE2QXRuSU1BMFBYSnB0b3JYRWNXNWg4TyIsImlhdCI6MTY5OTI2NjgzOCwiZXhwIjoxNjk5MzAyODM4LCJzdWIiOiJnb29nbGUtb2F1dGgyfDExNzk5MjUxNjU1NjQ5Njk5Mjk3NyIsInNpZCI6IkN6R1FHYVhPSkpXX2dmMER1ZzdhekFNUExVZ3NRNUdvIn0.XiIZF2nqaCamUcQi0ac5-0dsIfpYZV9bqKzQYpfQT2zt639hBOROVQK1vTqlskgDS9XMT-B46lzlNb5y9LKSfSt1EqNaCeuMzIa6YiJjSD_q8T4QaWSL58mBwjLbBksQ8LnUWQO1SyFBwpoC2LXYyuuHGYvpkEQWGliJHz8_NAepJQXWEo7nXuXucnjmqVF3X5jPYJtJwkUzYjPVqCEfmg27kpPBKbAuOqf4TVuUVbcHhalX_u9mJD_Y5i2CCrWL210LRMpXYawYJIU2UI7FijEd0tkYH87uOADgUk5HZyRw42o6tE2JAwdohiBqOF__hKcrR8oxhGB0o5_paGiLFg";
    match key_set.verify(token){
        Ok(jwt) => {
            dbg!(jwt);
            //println!("name={}", jwt.payload().get_str("name").unwrap());
        }
        Err(Error { msg, typ: _ }) => {
            eprintln!("Could not verify token. Reason: {}", msg);
        }
    }
}
