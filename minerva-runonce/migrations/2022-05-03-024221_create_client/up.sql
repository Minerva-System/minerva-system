CREATE TABLE CLIENT (
	ID       SERIAL     PRIMARY KEY,
	TYPE     SMALLINT   NOT NULL      DEFAULT 0,  -- Física, Jurídica
	NAME     VARCHAR    NOT NULL,
	DOCUMENT VARCHAR    NOT NULL,                 -- CPF ou CNPJ
	ACTIVE   BOOLEAN    NOT NULL      DEFAULT 'T' -- Desconsiderar cliente
)

