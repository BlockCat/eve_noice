-- Your SQL goes here
CREATE TABLE eve_characters (
  id int NOT NULL,
  name VARCHAR(128) UNIQUE NOT NULL,
  access_token VARCHAR(128) UNIQUE NOT NULL,
  refresh_token VARCHAR(128) UNIQUE NOT NULL,
  expiry_date DATETIME NOT NULL,  
  PRIMARY KEY (id)
);