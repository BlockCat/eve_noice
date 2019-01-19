use crate::models::EveCharacter;
use crate::EveDatabase;

use rocket::http::{Cookies, Cookie};
use oauth2::Config;
use oauth2::Token;
use chrono::DateTime;
use rocket::request;
use rocket::http::Status;

pub fn create_config() -> Config {
    
    let mut config = Config::new(dotenv!("CLIENT_ID"), dotenv!("CLIENT_SECRET"), dotenv!("EVE_AUTH_URL"), dotenv!("EVE_TOKEN_URL"));

    config = config.add_scope("esi-wallet.read_character_wallet.v1");
    config = config.add_scope("esi-markets.structure_markets.v1");
    config = config.add_scope("esi-markets.read_character_orders.v1");
    config = config.set_redirect_url(dotenv!("EVE_REDIRECT_URL"));
    config = config.set_state("1234");
    
    config
}

pub struct AuthedClient {
    access_token: String,
    client: reqwest::Client
}

impl<'a, 'r> request::FromRequest<'a, 'r> for AuthedClient {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<Self, Self::Error> {

        let mut eve_character = request.guard::<EveCharacter>()?;        

        // We get an eve_character,
        let access_token = if eve_character.expiry_date > chrono::Utc::now().naive_utc() { // Access token is not yet expired
            eve_character.access_token
        } else { // Access token is expired            
            let database = request.guard::<EveDatabase>()?;
            let mut cookies = request.guard::<Cookies>().unwrap();

            let config = create_config();
            
            let token: Token = match config.exchange_refresh_token(eve_character.refresh_token) {
                Ok(token) => token,
                _ => return rocket::Outcome::Forward(())
            };

            eve_character.access_token = token.access_token;
            eve_character.refresh_token = token.refresh_token.unwrap();
            eve_character.expiry_date = (chrono::Utc::now() + chrono::Duration::seconds(token.expires_in.unwrap() as i64 - 60)).naive_utc();

            let cookie_access = Cookie::build("key", eve_character.access_token.clone())
                .path("/")
                //.secure(true)
                .finish();

            cookies.add(cookie_access);

            // Update database
            match eve_character.insert(&database) {
                Ok(_) => eve_character.access_token,
                Err(_) => return rocket::Outcome::Failure((Status::new(500, "Something went wrong in the database."), ()))
            }
        };
        
        // Check if
        let mut header = reqwest::header::HeaderMap::new();
        header.insert(reqwest::header::USER_AGENT, "Gale Kishunuba".parse().unwrap());
        
        let client = reqwest::ClientBuilder::new()
            .default_headers(header)
            .build().unwrap();

        rocket::Outcome::Success(AuthedClient{ access_token, client })
    }
}