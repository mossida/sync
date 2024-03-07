DEFINE TABLE entity SCHEMAFULL
  PERMISSIONS
    FOR select, create, update, delete FULL;

DEFINE FIELD enabled ON entity TYPE bool DEFAULT true ASSERT $value != NONE;
DEFINE FIELD available ON entity TYPE bool DEFAULT true ASSERT $value != NONE;
DEFINE FIELD class ON entity TYPE string ASSERT $value != NONE;
DEFINE FIELD attributes ON entity FLEXIBLE TYPE object DEFAULT {};
DEFINE FIELD state ON entity FLEXIBLE TYPE object DEFAULT {};

DEFINE EVENT status_attributes_changed ON TABLE entity WHEN $after != NONE AND $before.attributes != $after.attributes THEN {
    UPDATE $after.id SET status.updated_at = time::now();

    CREATE event CONTENT {
        name: "status_attributes_changed",
        data: {
            from: $before.attributes,
            to: $after.attributes
        },
        source: $after.id
    };
};

DEFINE EVENT status_state_changed ON TABLE entity WHEN $after != NONE AND $before.state != $after.state THEN (
    CREATE event CONTENT {
        name: "status_changed",
        data: {
             from: $before.state,
             to: $after.state
        },
        source: $after.id
    }
);