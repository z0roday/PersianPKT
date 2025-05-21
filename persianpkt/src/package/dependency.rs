use serde::{Serialize, Deserialize};
use semver::{Version, VersionReq};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDependency {
    pub name: String,
    pub version_req: Option<String>,
    pub is_optional: bool,
}

impl PackageDependency {
    pub fn new(name: String) -> Self {
        Self {
            name,
            version_req: None,
            is_optional: false,
        }
    }

    pub fn with_version_req(name: String, version_req: String) -> Self {
        Self {
            name,
            version_req: Some(version_req),
            is_optional: false,
        }
    }

    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }

    pub fn satisfies(&self, version: &str) -> bool {
        if let Some(req_str) = &self.version_req {
            if let (Ok(version_parsed), Ok(req_parsed)) = (Version::parse(version), VersionReq::parse(req_str)) {
                req_parsed.matches(&version_parsed)
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn from_string(dep_str: &str) -> Option<Self> {
        let parts: Vec<&str> = dep_str.split_whitespace().collect();
        if parts.is_empty() {
            return None;
        }

        let name = parts[0].to_string();
        let mut dependency = Self::new(name);

        if parts.len() > 1 {
            let version_str = parts[1..].join(" ");
            dependency.version_req = Some(version_str);
        }

        Some(dependency)
    }

    pub fn to_string(&self) -> String {
        if let Some(version_req) = &self.version_req {
            format!("{} ({})", self.name, version_req)
        } else {
            self.name.clone()
        }
    }
} 