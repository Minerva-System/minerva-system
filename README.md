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

- Docker;
- Docker Compose;
- Rust (stable channel, version 2021, preferably using `rustc` 1.60.0 or up).

For documentation:

- `mdbook`;
- `mdbook-graphviz`.

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
