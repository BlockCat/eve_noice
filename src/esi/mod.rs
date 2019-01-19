macro_rules! esi {
    ($function_name:ident, $path:tt, $ret:ty) => {
        fn $function_name() -> $ret {
            panic!()
        }
    };
}

esi!(get_wallet, "/characters/{character_id}/wallet/", f32);
