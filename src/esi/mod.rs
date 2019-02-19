

#[macro_export]
macro_rules! restpath {
    ($s:ident, $path:expr, [$a:ident: $t:ty]) => {
        impl RestPath<$t> for $s {
            fn get_path($a: $t) -> Result<String, restson::Error> {
                // Create esi_path first, then prepends /latest to it, so that it works with restson                
                Ok(format!("/latest{}", format!($path, $a = $a)))
            }
        }
    };
    ($s:ident, $path:expr, [$($a:ident: $t:ty), *]) => {
        impl RestPath<($($t), *)> for $s {
            fn get_path(($($a), *) : ($($t), *)) -> Result<String, restson::Error> {
                // Create esi_path first, then prepends /latest to it, so that it works with restson                
                Ok(format!("/latest{}", format!($path, $($a = $a), *)))
            }
        }
    };
}

#[macro_use] mod wallet;

pub use self::wallet::*;