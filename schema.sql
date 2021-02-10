-- Double newline mandatory between queries
CREATE TABLE IF NOT EXISTS todos(
	id SERIAL PRIMARY KEY,
	title TEXT,
	description TEXT,
	tsv TSVECTOR,
	completed BOOLEAN
);

DROP TRIGGER IF EXISTS tsvectorupdate ON todos;

CREATE TRIGGER tsvectorupdate BEFORE INSERT OR UPDATE
ON todos FOR EACH ROW EXECUTE PROCEDURE
tsvector_update_trigger(tsv, 'pg_catalog.english', title, description);