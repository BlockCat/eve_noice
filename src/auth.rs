use oauth2::Config;


pub fn create_config() -> Config {
    let mut config = Config::new(env!("CLIENT_ID"), env!("CLIENT_SECRET"), env!("EVE_AUTH_URL"), env!("EVE_TOKEN_URL"));

    config = config.add_scope("esi-wallet.read_character_wallet.v1");
    config = config.add_scope("esi-markets.structure_markets.v1");
    config = config.add_scope("esi-markets.read_character_orders.v1");
    config = config.set_redirect_url(env!("EVE_REDIRECT_URL"));
    config = config.set_state("1234");
    
    config
}
