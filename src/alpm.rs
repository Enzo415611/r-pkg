use std::{fs, path::PathBuf};

use alpm::{Alpm, SigLevel};

use crate::AppState;

#[derive(Debug)]
pub struct AlpmState {
    pub alpm: Alpm,
    pub start_pkg_list: Vec<AlpmPkg>,
    pub pkg_list: Vec<AlpmPkg>,
    pub pkg_selected: AlpmPkg,
}

impl AlpmState {
    pub fn default() -> Self {
        Self {
            alpm: Alpm::new("/", "/var/lib/pacman").unwrap(),
            start_pkg_list: vec![],
            pkg_list: vec![],
            pkg_selected: AlpmPkg::default(),
        }
    }
}

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

        for db in self.alpm_state.alpm.syncdbs() {
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

    pub fn search_pkg_by_name(&self) -> Vec<AlpmPkg> {
        let name = &self.ui_state.search_content;
        let mut pkgs: Vec<AlpmPkg> = Vec::new();

        for db in self.alpm_state.alpm.syncdbs() {
            if let Ok(pkg) = db.pkg(name.to_owned()) {
                pkgs.push(AlpmPkg {
                    name: pkg.name().to_string(),
                    db: Some(pkg.db().map(|db| db.name()).unwrap_or_default().to_string()),
                    depends: pkg.depends().iter().map(|dep| dep.to_string()).collect(),
                    desc: pkg.desc().map(|desc| desc.to_string()),
                    size: pkg.size(),
                    ..Default::default()
                });
            }
        }
        pkgs
    }

    // fn get_all_dbs_name(&self) -> anyhow::Result<Vec<String>> {
    //     let mut dbs: Vec<String> = vec![];
    //     for db in self.alpm.syncdbs() {
    //         dbs.push(db.name().to_string());
    //     }

    //     Ok(dbs)
    // }

    fn register_dbs(&mut self) -> anyhow::Result<()> {
        let dbs_path = get_alpm_dbs_path();

        if let Ok(dbs_path) = dbs_path {
            for db in dbs_path {
                self.alpm_state
                    .alpm
                    .register_syncdb_mut(db, SigLevel::USE_DEFAULT)?;
            }
        }

        Ok(())
    }
}

// /var/lib/pacman/sync
fn get_alpm_dbs_path() -> anyhow::Result<Vec<String>> {
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
