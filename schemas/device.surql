DEFINE TABLE device SCHEMAFULL;

DEFINE FIELD name ON device TYPE string;
DEFINE FIELD serial ON device TYPE string;
DEFINE FIELD model ON device TYPE string;
DEFINE FIELD manufacturer ON device TYPE string;
DEFINE FIELD sw_version ON device TYPE option<string>;
DEFINE FIELD hw_version ON device TYPE option<string>;