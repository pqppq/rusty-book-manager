INSERT INTO
    roles (name)
VALUES
    ('Admin'),
    ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
    users (name, email, password_hash, role_id)
SELECT
    'Eleazar Fig',
    'eleazar.fig@example.com',
    '$2b$12$dPyskX8.SWL6WCl9XZPK6OEk6gKiNz5KOt3IZcfYYGuVkj06wGBp6',
    role_id
FROM
    roles
WHERE
    name LIKE 'Admin';
