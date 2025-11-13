#[macro_export]
macro_rules! define_app_error {
    (
        $(#[$enum_meta:meta])*
        pub enum $enum_name:ident {
            $($variants:tt)*
        }

        response: {
            delegates: [$($delegate_variant:ident),*],
            custom: {
                $($custom_match_arms:tt)*
            }
        }
    ) => {
        $(#[$enum_meta])*
        pub enum $enum_name {
            $($variants)*
        }

        impl actix_web::ResponseError for $enum_name {
            fn error_response(&self) -> actix_web::HttpResponse {
                match self {
                    $(
                        Self::$delegate_variant(inner) => inner.error_response(),
                    )*
                    $($custom_match_arms)*
                }
            }
        }
    };
}
