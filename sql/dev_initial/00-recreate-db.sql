-- DEV ONLY - Brute Force DROP DB
SELECT pg_terminate_backend(pid) 
FROM pg_stat_activity 
WHERE usename = 'app_user' OR datname = 'app_db';

DROP DATABASE IF EXISTS app_db;
DROP USER IF EXISTS app_user;

CREATE USER app_user PASSWORD 'dev_only_pwd';
CREATE DATABASE app_db owner app_user ENCODING = 'UTF-8';

-- CREATE USER sa_user PASSWORD 'dev_only_pwd';
-- CREATE DATABASE post_backend_db owner sa_user ENCODING = 'UTF-8';