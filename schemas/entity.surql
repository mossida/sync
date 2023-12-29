DEFINE TABLE entity SCHEMAFULL;

DEFINE FIELD enabled ON entity TYPE bool DEFAULT true;
DEFINE FIELD available ON entity TYPE bool DEFAULT true;
DEFINE FIELD class ON entity TYPE string;
DEFINE FIELD attributes ON entity TYPE option<object>;
DEFINE FIELD state ON entity FLEXIBLE TYPE object;

DEFINE EVENT state_attributes_changed ON TABLE entity WHEN $before.state.attributes != $after.state.attributes THEN (
    UPDATE $after.id SET state.updated_at = time::now()
);