DEFINE TABLE state SCHEMAFULL;

DEFINE FIELD state ON state TYPE string ASSERT $value != null;
DEFINE FIELD attributes ON state FLEXIBLE TYPE object DEFAULT {};
DEFINE FIELD updated_at ON state TYPE datetime DEFAULT time::now();
DEFINE FIELD entity_id ON state TYPE record<entity>;

-- Set last state to the entity
DEFINE EVENT state_created ON TABLE state WHEN $event = "CREATE" THEN (
    UPDATE $after.entity_id SET state_id = $after.id
);

DEFINE EVENT attributes_changed ON TABLE state WHEN $before.attributes != $after.attributes THEN (
    UPDATE $after.id SET updated_at = time::now()
);