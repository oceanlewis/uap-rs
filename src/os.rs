use super::Deserialize;

pub type OSFamily = String;
pub type OSMajor = String;
pub type OSMinor = String;
pub type OSPatch = String;
pub type OSPatchMinor = String;

#[derive(Debug, Deserialize)]
pub struct OS {
  family: OSFamily,
  major: Option<OSMajor>,
  minor: Option<OSMinor>,
  patch: Option<OSPatch>,
  patch_minor: Option<OSPatchMinor>,
}
