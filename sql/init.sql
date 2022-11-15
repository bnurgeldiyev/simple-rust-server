DROP TABLE IF EXISTS tbl_user;

CREATE TABLE tbl_user (
    id SERIAL  PRIMARY KEY,
    username  varchar(100) NOT NULL,
    password  varchar(150) NOT NULL,
    firstname varchar(50)  NOT NULL,
    lastname  varchar(50)  NOT NULL,
    create_ts timestamp without time zone DEFAULT current_timestamp,
    update_ts timestamp without time zone DEFAULT current_timestamp
);
