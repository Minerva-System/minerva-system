window.SIDEBAR_ITEMS = {"fn":[["create_session","Creates a new session for a user. Given the data for a new session, checks if the database contains that user, if the password matches and, if it does, creates a new session on the non-relational database and returns its ID as a Base64 encoded string that should be stored on a cookie."],["recover_session","Recovers a user’s session from the non-relational database, given a previously generated token. The token must be the actual ID for the session object on the non-relational database, encoded as Base64. If it was found, returns the `Session` object with the session information that was stored."],["remove_session","Removes a user session from non-relational database, given a session token. The token must be the actual ID for the session object on the non-relational database, encoded as Base64. If it was found, remove it altogether from the non-relational database."]]};