use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select};

use crate::util::SelectItem;

use super::ServerType;

impl ServerType {
    pub fn select_jar_interactive() -> Result<Self> {
        let items = vec![
            SelectItem(0, "Vanilla    - No patches".into()),
            SelectItem(1, "PaperMC    - Spigot fork, most popular".into()),
            SelectItem(2, "Purpur     - Paper fork".into()),
            SelectItem(3, "BuildTools - Spigot or CraftBukkit".into()),
        ];

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which server software to use?")
            .default(0)
            .items(&items)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Vanilla {},
            1 => Self::Paper {},
            2 => Self::Purpur {
                build: "latest".to_owned(),
            },
            3 => {
                let items = vec![
                    SelectItem(
                        Self::BuildTools {
                            args: vec![],
                            software: "spigot".into(),
                        },
                        "Spigot".into(),
                    ),
                    SelectItem(
                        Self::BuildTools {
                            args: vec![],
                            software: "craftbukkit".into(),
                        },
                        "CraftBukkit".into(),
                    ),
                ];

                let idx = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Which one?")
                    .default(0)
                    .items(&items)
                    .interact()?;

                items[idx].0.clone()
            },
            _ => unreachable!(),
        })
    }

    pub fn select_modded_jar_interactive() -> Result<Self> {
        let items = [
            (0, "Quilt (fabric compatible)"),
            (1, "Fabric"),
            (2, "NeoForge (forge compatible)"),
            (3, "Forge"),
        ];

        let items_str: Vec<String> = items.iter().map(|v| v.1.to_owned()).collect();

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which mod loader to use?")
            .default(0)
            .items(&items_str)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Quilt {
                loader: "latest".to_owned(),
                installer: "latest".to_owned(),
            },
            1 => Self::Fabric {
                loader: "latest".to_owned(),
                installer: "latest".to_owned(),
            },
            2 => Self::NeoForge {
                loader: "latest".to_owned(),
            },
            3 => Self::Forge {
                loader: "latest".to_owned(),
            },
            _ => unreachable!(),
        })
    }

    pub fn select_proxy_jar_interactive() -> Result<Self> {
        let items = [
            (0, "Velocity   - Modern, high perf. proxy by PaperMC"),
            (1, "Waterfall  - BungeeCord fork by PaperMC"),
            (2, "Bungeecord - By md5 (Spigot)"),
        ];

        let items_str: Vec<String> = items.iter().map(|v| v.1.to_owned()).collect();

        let jar_type = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Which proxy software to use?")
            .default(0)
            .items(&items_str)
            .interact()?;

        Ok(match jar_type {
            0 => Self::Velocity {},
            1 => Self::Waterfall {},
            2 => Self::BungeeCord {},
            _ => unreachable!(),
        })
    }
}
