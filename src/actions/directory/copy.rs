use std::fs::create_dir_all;

use super::DirectoryAction;
use crate::actions::{Action, ActionError, ActionResult};
use crate::manifest::Manifest;
use fs_extra::dir::CopyOptions;
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirectoryCopy {
    pub from: String,
    pub to: String,
}

impl DirectoryCopy {}

impl DirectoryAction for DirectoryCopy {}

impl Action for DirectoryCopy {
    fn run(&self, manifest: &Manifest, _context: &Context) -> Result<ActionResult, ActionError> {
        let absolute_path = manifest
            .root_dir
            .clone()
            .unwrap()
            .join("files")
            .join(&self.from);

        match create_dir_all(&self.to) {
            Ok(_) => (),
            Err(_) => {
                return Err(ActionError {
                    message: String::from("Failed to create directory"),
                });
            }
        }

        match fs_extra::dir::copy(
            &absolute_path,
            &self.to,
            &CopyOptions {
                overwrite: true,
                content_only: true,
                ..Default::default()
            },
        ) {
            Ok(_) => Ok(ActionResult {
                message: String::from("Copied"),
            }),
            Err(e) => Err(ActionError {
                message: e.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions::Actions;
    use crate::manifest;
    use crate::Action;

    #[test]
    fn it_can_be_deserialized() {
        let yaml = r#"
- action: directory.copy
  from: a
  to: b
"#;

        let mut actions: Vec<Actions> = serde_yaml::from_str(yaml).unwrap();

        match actions.pop() {
            Some(Actions::DirectoryCopy(dir_copy)) => {
                assert_eq!("a", dir_copy.from);
                assert_eq!("b", dir_copy.to);
            }
            _ => {
                panic!("DirectoryCopy didn't deserialize to the correct type");
            }
        };
    }

    #[test]
    fn it_can_copy_a_directory() {
        let manifest_dir = std::env::current_dir()
            .unwrap()
            .join("examples")
            .join("directory")
            .join("copy");

        let manifest = manifest::Manifest {
            name: Some(String::from("copy")),
            actions: vec![],
            dag_index: None,
            depends: vec![],
            root_dir: Some(manifest_dir.clone()),
        };

        let to = std::env::temp_dir().join("test-case");

        let directory_copy = super::DirectoryCopy {
            from: String::from("mydir"),
            to: String::from(to.to_str().unwrap()),
        };

        directory_copy
            .run(&manifest, &tera::Context::new())
            .unwrap();

        assert_eq!(true, to.is_dir());
        assert_eq!(true, to.join("file-a").exists());
        assert_eq!(true, to.join("file-b").exists());
    }
}
