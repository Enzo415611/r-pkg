use std::{fs, path::PathBuf};

use alpm::SigLevel;

use crate::AppState;

#[derive(Debug, Clone, Default)]
pub struct AlpmPkg {
    pub name: String,
    pub db: Option<String>,
    pub depends: Vec<String>,
    pub desc: Option<String>,
    pub size: i64,
}

impl AppState {
    pub fn sync_alpm_dbs(&mut self) -> anyhow::Result<Vec<AlpmPkg>> {
        self.register_dbs()?;
        let mut pkgs: Vec<AlpmPkg> = vec![];

        for db in self.alpm.syncdbs() {
            for pkg in db.pkgs() {
                pkgs.push(AlpmPkg {
                    name: pkg.name().to_string(),
                    db: pkg.db().map(|db| db.name().to_string()),
                    depends: pkg.depends().iter().map(|dep| dep.to_string()).collect(),
                    desc: pkg.desc().map(|d| d.to_string()),
                    size: pkg.size(),
                });
            }
            if pkgs.iter().len() >= 100 {
                break;
            }
        }
        Ok(pkgs)
    }

    // fn search_pkg(&self) {}

    // fn get_all_dbs(&self) -> anyhow::Result<Vec<String>> {
    //     let mut dbs: Vec<String> = vec![];
    //     for db in self.alpm.syncdbs() {
    //         dbs.push(db.name().to_string());
    //     }

    //     Ok(dbs)
    // }

    fn register_dbs(&mut self) -> anyhow::Result<()> {
        let dbs_path = get_alpm_dbs();

        if let Ok(dbs_path) = dbs_path {
            for db in dbs_path {
                self.alpm.register_syncdb_mut(db, SigLevel::USE_DEFAULT)?;
            }
        }

        Ok(())
    }
}

// /var/lib/pacman/sync
fn get_alpm_dbs() -> anyhow::Result<Vec<String>> {
    let path = PathBuf::from("/var/lib/pacman/sync");
    let mut dbs: Vec<String> = vec![];
    if let Ok(dir) = fs::read_dir(path) {
        for entry in dir {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("db") {
                if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                    dbs.push(name.to_string());
                }
            }
        }
    }

    Ok(dbs)
}
