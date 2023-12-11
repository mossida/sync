REMOVE FIELD state ON TABLE entity;
UPDATE entity PATCH [{ "op": "remove", "path": "state" }];