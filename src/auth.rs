use crate::models::EveCharacter;
use crate::EveDatabase;

use oauth2::Config;
use oauth2::Token;
use rocket::request;
use rocket::http::Status;

pub fn create_config() -> Config {
    
    let mut config = Config::new(dotenv!("CLIENT_ID"), dotenv!("CLIENT_SECRET"), dotenv!("EVE_AUTH_URL"), dotenv!("EVE_TOKEN_URL"));

    config = config.add_scope("esi-wallet.read_character_wallet.v1");
    config = config.add_scope("esi-markets.structure_markets.v1");
    config = config.add_scope("esi-markets.read_character_orders.v1");
    println!("{}", dotenv!("EVE_REDIRECT_URL"));
    config = config.set_redirect_url(dotenv!("EVE_REDIRECT_URL"));
    config = config.set_state("1234");
    
    config
}


pub struct AuthedClient(pub restson::RestClient);

impl<'a, 'r> request::FromRequest<'a, 'r> for AuthedClient {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {

        let mut eve_character = request.guard::<EveCharacter>()?;        

        // We get an eve_character,
        let access_token = if eve_character.expiry_date > chrono::Utc::now().naive_utc() { // Access token is not yet expired
            eve_character.access_token.clone()
        } else { // Access token is expired            
            let database = request.guard::<EveDatabase>()?;

            let config = create_config();
            
            let token: Token = match config.exchange_refresh_token(eve_character.refresh_token) {
                Ok(token) => token,
                _ => return rocket::Outcome::Forward(())
            };

            eve_character.access_token = token.access_token;
            eve_character.refresh_token = token.refresh_token.unwrap();
            eve_character.expiry_date = (chrono::Utc::now() + chrono::Duration::seconds(token.expires_in.unwrap() as i64 - 60)).naive_utc();

            // Update database
            match eve_character.upsert(&database) {
                Ok(_) => eve_character.access_token.clone(),
                Err(_) => return rocket::Outcome::Failure((Status::new(500, "Something went wrong in the database."), ()))
            }
        };        
        
        let mut client = restson::RestClient::builder()
            .send_null_body(false)
            .build(dotenv!("EVE_ESI_URL")).unwrap();

        client.set_header("USER_AGENT", "Eve Noic - Gale Kishunuba ").unwrap();
        client.set_header("Authorization", &format!("Bearer {}", access_token)).unwrap();

        rocket::Outcome::Success(AuthedClient(client))
    }
}

pub struct RestClient(restson::RestClient);

impl<'a, 'r> request::FromRequest<'a, 'r> for RestClient {
    type Error = ();

    fn from_request(_: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {
        let mut client = restson::RestClient::builder()
            .send_null_body(false)
            .build(dotenv!("EVE_ESI_URL")).unwrap();

        client.set_header("USER_AGENT", "Eve Noic - Gale Kishunuba ").unwrap();

        rocket::Outcome::Success(RestClient(client))
    }
}