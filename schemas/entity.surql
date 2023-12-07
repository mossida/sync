DEFINE TABLE entity SCHEMALESS;

DEFINE FIELD state ON entity TYPE string;
DEFINE FIELD enabled ON entity TYPE bool DEFAULT true;
DEFINE FIELD available ON entity TYPE bool DEFAULT true;
DEFINE FIELD class ON entity TYPE string;
DEFINE FIELD attributes ON entity TYPE object;

-- DEFINE EVENT state_changed ON TABLE entity WHEN $before.state != $after.state THEN ();