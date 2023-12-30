DEFINE TABLE event SCHEMAFULL;

DEFINE FIELD name ON event TYPE string VALUE string::lowercase($value);
DEFINE FIELD data ON event FLEXIBLE TYPE object;
DEFINE FIELD source ON event TYPE record;
DEFINE FIELD occurred_at ON event TYPE datetime DEFAULT time::now();