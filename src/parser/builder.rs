use crate::{Error, UserAgentParser};

pub struct UserAgentParserBuilder {
    pub(super) device: bool,
    pub(super) os: bool,
    pub(super) user_agent: bool,
    pub(super) unicode: bool,
}

impl UserAgentParserBuilder {
    pub(super) fn new() -> Self {
        UserAgentParserBuilder {
            device: true,
            os: true,
            user_agent: true,
            unicode: true,
        }
    }

    /// Enable or disable unicode support. This is enabled by default.
    /// Unicode regexes are much more complex and take up more memory.
    /// Most uaparser implementation do not support unicode, so disabling
    /// this is generally safe to do.
    pub fn with_unicode_support(mut self, enabled: bool) -> Self {
        self.unicode = enabled;
        return self;
    }

    /// Enable or disable device parsing. This is enabled by default.
    /// Because all regexes are compiled up front, disabling this will
    /// save a decent amount of memory.
    pub fn with_device(mut self, enabled: bool) -> Self {
        self.device = enabled;
        return self;
    }

    /// Enable or disable os parsing. This is enabled by default.
    /// Because all regexes are compiled up front, disabling this will
    /// save a decent amount of memory.
    pub fn with_os(mut self, enabled: bool) -> Self {
        self.os = enabled;
        return self;
    }

    /// Enable or disable user agent parsing. This is enabled by default.
    /// Because all regexes are compiled up front, disabling this will
    /// save a decent amount of memory.
    pub fn with_user_agent(mut self, enabled: bool) -> Self {
        self.user_agent = enabled;
        return self;
    }

    pub fn build_from_yaml(self, path: &str) -> Result<UserAgentParser, Error> {
        UserAgentParser::_build_from_yaml(path, self)
    }
    /// Attempts to construct a `UserAgentParser` from a slice of raw bytes. The
    /// intention with providing this function is to allow using the
    /// `include_bytes!` macro to compile the `regexes.yaml` file into the
    /// the library by a consuming application.
    ///
    /// ```rust
    /// # use uaparser::*;
    /// let regexes = include_bytes!("../../src/core/regexes.yaml");
    /// let parser = UserAgentParser::builder().build_from_bytes(regexes);
    /// ```
    pub fn build_from_bytes(self, bytes: &[u8]) -> Result<UserAgentParser, Error> {
        UserAgentParser::_build_from_bytes(bytes, self)
    }
}
