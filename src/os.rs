use super::Deserialize;

pub type OSFamily = String;
pub type OSMajor = String;
pub type OSMinor = String;
pub type OSPatch = String;
pub type OSPatchMinor = String;

#[derive(Clone, Debug, Deserialize)]
pub struct OS {
  pub family: OSFamily,
  pub major: Option<OSMajor>,
  pub minor: Option<OSMinor>,
  pub patch: Option<OSPatch>,
  pub patch_minor: Option<OSPatchMinor>,
}

impl Default for OS {
  fn default() -> OS {
    OS {
      family: "Other".to_string(),
      major: None,
      minor: None,
      patch: None,
      patch_minor: None,
    }
  }
}
