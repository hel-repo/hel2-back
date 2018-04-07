ALTER TABLE users DROP COLUMN IF EXISTS registered;

ALTER TABLE packages DROP COLUMN IF EXISTS downloads,
                     DROP COLUMN IF EXISTS created,
                     DROP COLUMN IF EXISTS updated;

DROP TABLE IF EXISTS likes;

ALTER TABLE versions DROP COLUMN IF EXISTS created;

ALTER TABLE maintainers DROP CONSTRAINT maintainers_package_fk;
ALTER TABLE maintainers ADD CONSTRAINT maintains_package_fk FOREIGN KEY (package) REFERENCES packages ON DELETE CASCADE;