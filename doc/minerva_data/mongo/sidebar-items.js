window.SIDEBAR_ITEMS = {"fn":[["build_client_string","Generates a string to connect to MongoDB, given a server endpoint (e.g. `localhost:27017`). This function assumes that MongoDB was configured with the credentials of a user `root` with password `mongo`."],["make_client","Creates a client connection to the MongoDB service, and panics if the connection could not be established."],["try_make_client","Attempts to generate a single client for the MongoDB sevice, without error checks. This could be used to perform connections and evaluate if they could actually be estalished."]],"static":[["SESSION_DURATION","Default user session duration for the `session` collection, in seconds. Calculates to one week by default. Minimum value is one minute, as per limitations of MongoDB related to expiration checks."]]};