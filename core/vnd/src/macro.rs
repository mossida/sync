#[macro_export]
macro_rules! implement {
    ($($module:ident => $enum:ident),*) => {
        use serde::{Deserialize, Serialize};
        use err::{Result, Error};

        // Generate use statements for each module and enum

        // Define the enum
        #[derive(Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum Vendors {
            $($enum),*
        }

        // Define the implementation function
        pub async fn implement(v: Vendors, config: serde_json::Value) -> Result<(), Error> {
            match v {
                $(
                    Vendors::$enum => {
                        crate::vendors::$module::$enum::build(config).await?;
                    }
                ),*
            };

            Ok(())
        }
    };
}
