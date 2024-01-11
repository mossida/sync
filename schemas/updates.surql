DEFINE TABLE updates SCHEMAFULL;

DEFINE FIELD in ON TABLE updates TYPE record<device>;
DEFINE FIELD out ON TABLE updates TYPE record<entity>;

DEFINE INDEX unique_updates ON TABLE updates COLUMNS in, out UNIQUE;