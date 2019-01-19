use crate::auth::AuthedClient;
use reqwest::header;

fn get_hyper_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(header::USER_AGENT, "Gale Kishunuba".parse().unwrap());

    reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build().unwrap()
}

macro_rules! get_esi {
    ($function_name:ident, $path:tt, $ret:ty, $(($n:ident: $t:ty)),*) => {
        fn $function_name($($n: $t), *) -> $ret {
            let esi_url = dotenv!("EVE_ESI_URL");
            let esi_path = format!($path, $($n = $n), *);
            let esi_path = format!("{}{}", esi_url, esi_path);
            
            println!("{}", esi_path);
            panic!()
        }
    };
}

macro_rules! get_esiauth {
    ($function_name:ident, $path:tt, $ret:ty, $(($n:ident: $t:ty)),*) => {
        fn $function_name(client: AuthedClient, $($n: $t), *) -> $ret {
            let esi_url = dotenv!("EVE_ESI_URL");
            let esi_path = format!($path, $($n = $n), *);
            let esi_path = format!("{}{}", esi_url, esi_path);
            
            println!("{}", esi_path);
            panic!()
        }
    };
}

get_esiauth!(get_wallet, "/characters/{character_id}/", f32, (character_id: i32));
get_esi!(get_alliance, "/alliances/{alliance_id}/", f32, (alliance_id: i32));
