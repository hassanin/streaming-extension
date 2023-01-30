CREATE TABLE IF NOT EXISTS test123(
    id integer,
    name text
);

CREATE TABLE IF NOT EXISTS status(
    result text,
    time_stamp TIMESTAMP NOT NULL DEFAULT now()
);
-- Triggers
CREATE OR REPLACE FUNCTION insert_into_enriched()
RETURNS trigger LANGUAGE plpgsql
AS $$
BEGIN
    INSERT INTO status(result) SELECT hello_do_stuff(NEW.id,NEW.name);
    RETURN NULL;
END $$;

CREATE TRIGGER test123_trigger
AFTER INSERT
ON test123 for each ROW
EXECUTE FUNCTION insert_into_enriched();
