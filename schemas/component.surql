DEFINE TABLE component SCHEMAFULL;

DEFINE FIELD reference ON component TYPE string;
DEFINE FIELD configuration ON component FLEXIBLE TYPE object;
DEFINE FIELD priority ON component TYPE int DEFAULT 1 ASSERT $value >= 1 && $value <= 4;