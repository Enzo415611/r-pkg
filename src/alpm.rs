use std::{collections::HashSet, fs, path::PathBuf};

use alpm::{Alpm, SigLevel, TransFlag};
use iced::widget::sensor::Key;

use crate::AppState;

#[derive(Debug)]
pub struct AlpmState {
    pub alpm: Alpm,
    pub start_pkg_list: HashSet<AlpmPkg>,
    pub pkg_list: HashSet<AlpmPkg>,
    pub pkg_selected: AlpmPkg,
}

impl AlpmState {
    pub fn default() -> Self {
        Self {
            alpm: Alpm::new("/", "/var/lib/pacman").unwrap(),
            start_pkg_list: HashSet::new(),
            pkg_list: HashSet::new(),
            pkg_selected: AlpmPkg::default(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq ,Eq, Hash)]
pub struct AlpmPkg {
    pub name: String,
    pub db: Option<String>,
    pub depends: Vec<String>,
    pub desc: Option<String>,
    pub size: i64,
    pub is_installed: bool,
}

impl AppState {
    pub fn sync_alpm_dbs(&mut self) -> anyhow::Result<HashSet<AlpmPkg>> {
        self.register_dbs()?;

        let mut pkgs: HashSet<AlpmPkg> = HashSet::new();

        for db in self.alpm_state.alpm.syncdbs() {
            let mut count_db = 0;

            for pkg in db.pkgs() {
                if count_db >= 15 {
                    break;
                }
                let is_installed = self.pkg_is_installed(pkg.name());

                pkgs.insert(AlpmPkg {
                    name: pkg.name().to_string(),
                    db: pkg.db().map(|db| db.name().to_string()),
                    depends: pkg.depends().iter().map(|dep| dep.to_string()).collect(),
                    desc: pkg.desc().map(|d| d.to_string()),
                    size: pkg.size(),
                    is_installed,
                });

                count_db += 1;

                if pkgs.len() >= 100 {
                    break;
                }
            }
            if pkgs.len() >= 100 {
                break;
            }
        }
        Ok(pkgs)
    }

//     pub fn sync(&mut self) -> anyhow::Result<()> {
//         self.register_dbs()?;
//         Ok(())
//     }
    
    pub fn search_pkg_by_name(&mut self) -> HashSet<AlpmPkg> {
        let name = &self.ui_state.search_content;
        let mut pkgs: HashSet<AlpmPkg> = HashSet::new();

        for db in self.alpm_state.alpm.syncdbs() {
            if let Ok(pkg) = db.pkg(name.to_owned()) {
                let is_installed = self.pkg_is_installed(pkg.name());

                pkgs.insert(AlpmPkg {
                    name: pkg.name().to_string(),
                    db: Some(pkg.db().map(|db| db.name()).unwrap_or_default().to_string()),
                    depends: pkg.depends().iter().map(|dep| dep.to_string()).collect(),
                    desc: pkg.desc().map(|desc| desc.to_string()),
                    size: pkg.size(),
                    is_installed,
                });
            }
        }
        pkgs
    }

    pub fn register_dbs(&mut self) -> anyhow::Result<()> {
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

    pub fn pkg_is_installed(&self, pkg_name: &str) -> bool {
        let alpm = Alpm::new("/", "/var/lib/pacman").unwrap();
        let local_db = alpm.localdb();
        local_db.pkg(pkg_name).is_ok()
    }
    
    
    
    // Teste install fn
    pub fn install_pkg(&mut self) -> anyhow::Result<()> {
        let flags = TransFlag::DOWNLOAD_ONLY;
        _ = self.alpm_state.alpm.trans_init(flags)?;
        
        // add pkg
        for db in self.alpm_state.alpm.syncdbs() {
            if let Ok(pkg) = db.pkg(self.alpm_state.pkg_selected.name.as_str()) {
                self.alpm_state.alpm.trans_add_pkg(pkg);
                break;
            }
        }
        
        
        self.alpm_state.alpm.sync_sysupgrade(false);
        
        self.alpm_state.alpm.trans_prepare();
        let install = self.alpm_state.alpm.trans_add();
        println!("{:?}", install);
        self.alpm_state.alpm.trans_commit()?;
        
        
        
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
