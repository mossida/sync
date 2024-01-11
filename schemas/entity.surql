DEFINE TABLE entity SCHEMAFULL
  PERMISSIONS
    FOR select, create, update, delete FULL;

DEFINE FIELD enabled ON entity TYPE bool DEFAULT true ASSERT $value != NONE;
DEFINE FIELD available ON entity TYPE bool DEFAULT true ASSERT $value != NONE;
DEFINE FIELD class ON entity TYPE string ASSERT $value != NONE;
DEFINE FIELD attributes ON entity FLEXIBLE TYPE object DEFAULT {};
DEFINE FIELD status ON entity FLEXIBLE TYPE object DEFAULT {};

DEFINE EVENT status_attributes_changed ON TABLE entity WHEN $after != NONE AND $before.status.attributes != $after.status.attributes THEN {
    UPDATE $after.id SET status.updated_at = time::now();

    CREATE event CONTENT {
        name: "status_attributes_changed",
        data: {
            from: $before.status.attributes,
            to: $after.status.attributes
        },
        source: $after.id
    };
};

DEFINE EVENT status_state_changed ON TABLE entity WHEN $after != NONE AND $before.status.state != $after.status.state THEN (
    CREATE event CONTENT {
        name: "status_changed",
        data: {
             from: $before.status.state,
             to: $after.status.state
        },
        source: $after.id
    }
);