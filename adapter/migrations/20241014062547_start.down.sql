DROP TRIGGER IF EXISTS books_updated_at_trigger ON books;
DROP TABLE IF EXISTS books;

DROP FUNCTION set_updated_at;
DROP TRIGGER IF EXISTS users_updated_at_trigger ON users;
DROP TABLE IF EXISTS users;
DROP TABLE IS EXISTS roles;
