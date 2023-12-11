DEFINE TABLE entity SCHEMAFULL;

DEFINE FIELD enabled ON entity TYPE bool DEFAULT true;
DEFINE FIELD available ON entity TYPE bool DEFAULT true;
DEFINE FIELD class ON entity TYPE string;
DEFINE FIELD attributes ON entity TYPE option<object>;
DEFINE FIELD state_id ON entity TYPE option<record<state>>;