ALTER TABLE users ALTER COLUMN password SET DATA TYPE BYTEA USING decode(password, 'hex'),
                  ALTER COLUMN salt SET DATA TYPE BYTEA USING decode(salt, 'hex');