CREATE TABLE ADDRESS (
	ID         SERIAL    PRIMARY KEY,
	CLIENT_ID  SERIAL    NOT NULL,
	TYPE       SMALLINT  NOT NULL,  -- Casa, trabalho...
	LOCATION   VARCHAR   NOT NULL,  -- Logradouro
	NUMBER     VARCHAR   NOT NULL,
	COMPLEMENT VARCHAR,
	DISTRICT   VARCHAR   NOT NULL, -- Bairro
	STATE      CHAR(2)   NOT NULL,
	CITY       VARCHAR   NOT NULL,
	COUNTRY    CHAR(2)   NOT NULL  DEFAULT 'BR',

	CONSTRAINT FK_CLIENT
	FOREIGN KEY(CLIENT_ID) REFERENCES CLIENT(ID)
)
