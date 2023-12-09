DEFINE TABLE controls SCHEMAFULL;

DEFINE FIELD in ON TABLE controls TYPE record<component>;
DEFINE FIELD out ON TABLE controls TYPE record<device>;

DEFINE INDEX unique_controls ON TABLE controls COLUMNS in, out UNIQUE;