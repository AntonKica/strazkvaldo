ALTER TABLE enum_values
    ADD _removed BOOLEAN NULL;
ALTER TABLE one_time_activity
    ADD _removed BOOLEAN NULL;
ALTER TABLE repeated_activity
    ADD _removed BOOLEAN NULL;
ALTER TABLE app_user
    ADD _removed BOOLEAN NULL;
ALTER TABLE room
    ADD _removed BOOLEAN NULL;

UPDATE enum_values SET _removed = false;
UPDATE one_time_activity SET _removed = false;
UPDATE repeated_activity SET _removed = false;
UPDATE app_user SET _removed = false;
UPDATE room SET _removed = false;

ALTER TABLE enum_values ALTER COLUMN _removed SET NOT NULL;
ALTER TABLE one_time_activity ALTER COLUMN _removed SET NOT NULL;
ALTER TABLE repeated_activity ALTER COLUMN _removed SET NOT NULL;
ALTER TABLE app_user ALTER COLUMN _removed SET NOT NULL;
ALTER TABLE room ALTER COLUMN _removed SET NOT NULL;