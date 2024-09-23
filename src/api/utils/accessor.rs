use std::path::PathBuf;

use anyhow::{anyhow, Result};
use reqwest::Url;
use serde::de::DeserializeOwned;
use zip::ZipArchive;

use crate::api::app::App;

/// An `Accessor` allows for filesystem, remote or zip file access.
pub enum Accessor {
    Local(PathBuf),
    Remote(reqwest::Url),
    ZipLocal((PathBuf, ZipArchive<std::fs::File>)),
    //ZipRemote(SomeSortOfTempFile, ZipArchive<std::fs::File>),
}

impl ToString for Accessor {
    fn to_string(&self) -> String {
        match self {
            Accessor::Local(path) => path.to_string_lossy().into_owned(),
            Accessor::Remote(url) => url.to_string(),
            Accessor::ZipLocal((path, _)) => path.to_string_lossy().into_owned(),
        }
    }
}

impl Accessor {
    pub fn from(str: &str) -> Result<Self> {
        if str.starts_with("http://") || str.starts_with("https://") {
            Ok(Self::Remote(Url::parse(str)?))
        } else if str.ends_with(".zip") || str.ends_with(".mrpack") {
            let file = std::fs::File::open(str)?;
            let archive = ZipArchive::new(file)?;
            Ok(Self::ZipLocal((PathBuf::from(str), archive)))
        } else {
            Ok(Self::Local(PathBuf::from(str)))
        }
    }

    /// Try listing a directory
    #[allow(unused)]
    pub async fn dir(&self) -> Result<Vec<String>> {
        match self {
            Accessor::ZipLocal((_, zip)) => Ok(zip.file_names().map(ToOwned::to_owned).collect()),
            Accessor::Local(path) => Ok(path
                .read_dir()?
                .filter_map(|r| r.ok())
                .map(|n| n.file_name().to_string_lossy().into_owned())
                .collect()),
            Accessor::Remote(_) => Err(anyhow!("cannot dir() Accessor::Remote")),
        }
    }

    /// Read a JSON file
    pub async fn json<T: DeserializeOwned>(&mut self, app: &App, path: &str) -> Result<T> {
        match self {
            Accessor::Local(base) => Ok(serde_json::from_reader(std::fs::File::open(
                base.join(path),
            )?)?),
            Accessor::ZipLocal((_, zip)) => Ok(serde_json::from_reader(zip.by_name(path)?)?),
            Accessor::Remote(url) => Ok(app.http_get_json(url.join(path)?).await?),
        }
    }

    /// Read a TOML file
    pub async fn toml<T: DeserializeOwned>(&mut self, app: &App, path: &str) -> Result<T> {
        match self {
            Accessor::Local(base) => Ok(toml::from_str(
                &tokio::fs::read_to_string(base.join(path)).await?,
            )?),
            Accessor::ZipLocal((_, zip)) => {
                let file = zip.by_name(path)?;

                Ok(toml::from_str(&std::io::read_to_string(file)?)?)
            },
            Accessor::Remote(url) => {
                let res = app.http_get(url.join(path)?).await?;
                let content = res.text().await?;

                Ok(toml::from_str(&content)?)
            },
        }
    }
}
