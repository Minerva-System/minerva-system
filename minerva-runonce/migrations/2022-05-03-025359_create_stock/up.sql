CREATE TABLE STOCK (
	PRODUCT_ID  SERIAL         PRIMARY KEY,
	AMOUNT      NUMERIC(12,3)  NOT NULL,
	COST        NUMERIC(13,4)  NOT NULL     DEFAULT 0.0000,

	CONSTRAINT FK_PRODUCT
	FOREIGN KEY(PRODUCT_ID) REFERENCES PRODUCT(ID)
)
