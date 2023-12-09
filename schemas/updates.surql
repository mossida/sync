DEFINE TABLE updates SCHEMAFULL;

DEFINE FIELD in ON TABLE updates TYPE record<device> ASSERT $value != NONE;
DEFINE FIELD out ON TABLE updates TYPE record<entity> ASSERT $value != NONE;
DEFINE FIELD with_polling ON TABLE updates TYPE bool DEFAULT true ASSERT $value != NONE;
DEFINE FIELD polling_interval ON TABLE updates TYPE number DEFAULT 120 ASSERT $value > 0;

DEFINE INDEX unique_updates ON TABLE updates COLUMNS in, out UNIQUE;