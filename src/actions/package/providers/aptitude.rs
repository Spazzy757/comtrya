use super::PackageProvider;
use crate::atoms::command::Exec;
use crate::{actions::package::PackageVariant, atoms::Atom};
use serde::{Deserialize, Serialize};
use tracing::warn;
use which::which;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Aptitude {}

impl Aptitude {
    fn env(&self) -> Vec<(String, String)> {
        vec![(
            String::from("DEBIAN_FRONTEND"),
            String::from("noninteractive"),
        )]
    }
}

impl PackageProvider for Aptitude {
    fn name(&self) -> &str {
        "Aptitude"
    }

    fn available(&self) -> bool {
        match which("apt-add-repository") {
            Ok(_) => true,
            Err(_) => {
                warn!(message = "apt-add-repository not available");
                false
            }
        }
    }

    fn bootstrap(&self) -> Vec<Box<dyn Atom>> {
        vec![Box::new(Exec {
            command: String::from("apt"),
            arguments: vec![
                String::from("install"),
                String::from("--yes"),
                String::from("software-properties-common"),
                String::from("gpg"),
            ],
            environment: self.env(),
            privileged: true,
            ..Default::default()
        })]
    }

    fn has_repository(&self, _package: &PackageVariant) -> bool {
        false
    }

    fn add_repository(&self, package: &PackageVariant) -> Vec<Box<dyn Atom>> {
        vec![
            Box::new(Exec {
                command: String::from("apt-add-repository"),
                arguments: vec![String::from("-y"), package.repository.clone().unwrap()],
                environment: self.env(),
                privileged: true,
                ..Default::default()
            }),
            Box::new(Exec {
                command: String::from("apt"),
                arguments: vec![String::from("update")],
                environment: self.env(),
                privileged: true,
                ..Default::default()
            }),
        ]
    }

    fn query(&self, package: &PackageVariant) -> Vec<String> {
        package.packages()
    }

    fn install(&self, package: &PackageVariant) -> Vec<Box<dyn Atom>> {
        vec![Box::new(Exec {
            command: String::from("apt"),
            arguments: vec![String::from("install"), String::from("--yes")]
                .into_iter()
                .chain(package.extra_args.clone())
                .chain(package.packages())
                .collect(),
            environment: self.env(),
            privileged: true,
            ..Default::default()
        })]
    }
}
