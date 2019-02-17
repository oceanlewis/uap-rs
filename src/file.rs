use super::*;

#[derive(Debug, Deserialize)]
pub struct RegexFile {
    pub user_agent_parsers: Vec<UserAgentParserEntry>,
    pub os_parsers: Vec<OSParserEntry>,
    pub device_parsers: Vec<DeviceParserEntry>,
}

#[derive(Debug, Deserialize)]
pub struct UserAgentParserEntry {
    pub regex: String,
    pub family_replacement: Option<String>,
    pub v1_replacement: Option<String>,
    pub v2_replacement: Option<String>,
    pub v3_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OSParserEntry {
    pub regex: String,
    pub os_replacement: Option<String>,
    pub os_v1_replacement: Option<String>,
    pub os_v2_replacement: Option<String>,
    pub os_v3_replacement: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeviceParserEntry {
    pub regex_flag: Option<String>,
    pub regex: String,
    pub device_replacement: Option<String>,
    pub brand_replacement: Option<String>,
    pub model_replacement: Option<String>,
}
