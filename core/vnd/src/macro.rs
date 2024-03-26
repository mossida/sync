#[macro_export]
macro_rules! implement {
    ($($module:ident => $enum:ident),*) => {
        // Define the enum
        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        #[serde(rename_all = "snake_case")]
        pub enum Vendors {
            Any,
            $($enum),*
        }

        // Define the implementation function
        pub async fn implement(v: &Vendors, config: serde_json::Value) -> err::Result<(), err::Error> {
            match v {
                $(
                    Vendors::$enum => {
                        $crate::vendors::$module::$enum::new(config)?.build().await?;
                    }
                ),*
                _ => {}
            };

            Ok(())
        }
    };
}
