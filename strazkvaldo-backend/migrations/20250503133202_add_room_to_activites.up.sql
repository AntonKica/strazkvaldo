ALTER TABLE one_time_activity
    ADD room_code CHAR(9) NULL REFERENCES room (code);
ALTER TABLE repeated_activity
    ADD room_code CHAR(9) NULL REFERENCES room (code);

UPDATE one_time_activity SET room_code = (SELECT code FROM room LIMIT 1);
UPDATE repeated_activity SET room_code = (SELECT code FROM room LIMIT 1);

ALTER TABLE one_time_activity ALTER COLUMN room_code SET NOT NULL;
ALTER TABLE repeated_activity ALTER COLUMN room_code SET NOT NULL;
