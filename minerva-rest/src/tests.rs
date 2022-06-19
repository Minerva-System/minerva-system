use super::launch;
use rocket::http::Status;
use rocket::local::blocking::Client;
use serde_json::json;
use serial_test::serial;
use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
};

#[derive(Debug)]
struct Microservices {
    services: Vec<(&'static str, Child)>,
}

impl Microservices {
    fn spawn_microservice(name: &str, service: &str) -> Child {
        println!("Spawning microservice {}...", name);
        // Spawn through `cargo run`
        let mut child = Command::new("cargo")
            .arg("run")
            .current_dir(format!("../{}", service))
            .stdout(Stdio::piped())
            .stdin(Stdio::piped())
            .spawn()
            .expect(&format!("Failed to create child process for {}", service));

        // Check if read by waiting for "SERVICENAME is ready to accept connections."
        // in command output
        let expected_text = format!("{} is ready to accept connections.", name);

        println!("Awaiting for {} to be ready...", name);
        'await_child: loop {
            // TODO: Add a test timeout
            if let Some(stdout) = &mut child.stdout {
                // There aren't many lines so slurp them on memory
                let lines = BufReader::new(stdout).lines().enumerate();
                for (counter, line) in lines {
                    if line.unwrap().trim() == expected_text.trim() {
                        println!(
                            "Microservice {} is ready (as per line output {})",
                            name, counter
                        );
                        break 'await_child;
                    }
                }
            }
        }
        child
    }

    fn spawn(services: Vec<&'static str>) -> Self {
        Microservices {
            services: services
                .iter()
                .map(|name| {
                    let service = match *name {
                        "TENANCY" => "minerva-tenancy",
                        "USERS" => "minerva-user",
                        "SESSION" => "minerva-session",
                        "PRODUCT" => "minerva-product",
                        "STOCK" => "minerva-stock",
                        "REPORT" => "minerva-report",
                        "CLIENT" => "minerva-client",
                        "AUDIT" => "minerva-audit",
                        "COMM" => "minerva-comm",
                        _ => panic!("Unknown service {}", name),
                    };
                    (*name, Microservices::spawn_microservice(name, service))
                })
                .collect(),
        }
    }

    fn dispose(&mut self) {
        for (svc, proc) in self.services.iter_mut() {
            proc.kill()
                .expect(&format!("Successfully kill {} microservice", svc))
        }
    }
}

/* Authentication */

#[test]
#[serial]
fn login_only() {
    let mut svc = Microservices::spawn(vec!["SESSION"]);

    println!("Launching API...");
    let client = Client::tracked(launch()).expect("Instância válida da API");

    let request = client.post("/teste/login").body(
        json! ({
            "login": "admin",
            "password": "admin"
        })
        .to_string(),
    );

    let response = request.dispatch();
    assert_eq!(response.status(), Status::Ok);

    svc.dispose();
}

#[test]
#[serial]
fn login_logout() {
    let mut svc = Microservices::spawn(vec!["SESSION"]);

    println!("Launching API...");
    let client = Client::tracked(launch()).expect("Instância válida da API");

    // Login
    let response = client
        .post("/teste/login")
        .body(
            json! ({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    // Logout
    // Reuses previous cookies
    let response = client
        .post("/logout")
        .body(
            json!({
                "login": "admin",
                "password": "admin"
            })
            .to_string(),
        )
        .dispatch();
    assert_eq!(response.status(), Status::Ok);

    svc.dispose();
}
