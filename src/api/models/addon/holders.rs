use serde::{Deserialize, Serialize};

use super::{Addon, AddonTarget, AddonType};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddonListFile {
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addons: Vec<Addon>,

    // backwards compatability
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub mods: Vec<AddonType>,
    #[serde(default = "Vec::new")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub plugins: Vec<AddonType>,
}

impl AddonListFile {
    pub fn flatten(self) -> Vec<Addon> {
        [
            self.addons,
            self.mods
                .into_iter()
                .map(|addon_type| Addon {
                    environment: None,
                    addon_type,
                    target: AddonTarget::Mods,
                })
                .collect(),
            self.plugins
                .into_iter()
                .map(|addon_type| Addon {
                    environment: None,
                    addon_type,
                    target: AddonTarget::Plugins,
                })
                .collect(),
        ]
        .concat()
    }
}
