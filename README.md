# Minerva System

Source code for the Minerva System, a bare minimum study on ERP solutions
using microservices and a multi-tenant architecture.

## Description

The Minerva System is a prototype of an ERP with bare minimum systems running,
but with room for improvement. It is not build as a serious competitor to any
other system **whatsoever**, instead being aimed as a case study for the
possible technological applications of bleeding edge technology for such a
use case.

As an overview, the Minerva System is an attempt at a microservice architecture
for a bare minimum ERP, using the best that Rust, Docker, Kubernetes, Redis and
Flutter have to offer.

The project is mostly focused on studies on microservices and deployment. From
the back-end standpoint, it has a REST gateway from which a client can interface
with the service, which is an array of microservices communicating and interfacing
with a PostgreSQL database; from the front-end standpoint, it is mostly a study
on using Flutter and Neumorphic Design.

Furthermore, for isolation purposes, it is also thought of as a multi-tenant
application that can work with multiple databases, one for each client, all of them
allocated within a PostgreSQL instance.

Since this is quite a complex application, do not expect it to be easy to deploy,
and do not expect it to be fully documented (though I'm trying my best to do so).
Also, **do not use it for commercial purposes**, unless explicitly given permission.
Think of this system as a big reference for some strategies on the given technologies,
though there are no guarantees that those strategies are necessarily correct.

## Dependencies

If you plan on running it over Docker Compose:

- Docker;
- Docker Compose.

If you plan on helping on development:

- Rust (`rustc` 1.60.0 or newer, supporting Rust Edition 2021);
- `libpq5` or higher (to connect to PostgreSQL 14);
- Docker (to run services that are not necessarily a direct part of the Minerva System).

For documentation:

- `mdbook`;
- `mdbook-graphviz`.

### Running some requests

There are some exported collections for Postman under `docs/postman/`. One would
want to import and use them to a new workspace on that platform.

## Documentation

For detailed documentation, please refer to the `docs/` folder. In there, most
planned details on the system's design can be found. Just install mdBook and
the mdBook Graphviz exporter through Cargo, then `cd` into the folder and
run:

```bash
mdbook serve --open
```

## Licensing

This project is licensed under the GPLv3 license. For more information, see
the LICENSE file, or refer to [Choose a License.com](https:///choosealicense.com).

