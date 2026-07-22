use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibFunctionDoc {
    pub module: String,
    pub name: String,
    pub signature: String,
    pub description: String,
    pub params: Vec<StdlibParamDoc>,
    pub returns: String,
    pub return_doc: Option<StdlibReturnDoc>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibParamDoc {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub(in crate::stdlib::metadata) struct StdlibFunctionDetails {
    pub(in crate::stdlib::metadata) key: StdlibFunctionKey,
    pub(in crate::stdlib::metadata) return_type: &'static str,
    pub(in crate::stdlib::metadata) description: String,
    pub(in crate::stdlib::metadata) params: Vec<StdlibParamDoc>,
    pub(in crate::stdlib::metadata) returns: String,
    pub(in crate::stdlib::metadata) examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibReturnDoc {
    pub type_name: String,
    pub description: String,
    pub fields: Vec<StdlibFieldDoc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibFieldDoc {
    pub name: String,
    pub type_name: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct StdlibFunctionKey {
    pub module: &'static str,
    pub name: &'static str,
}

impl StdlibFunctionKey {
    pub(crate) const fn new(module: &'static str, name: &'static str) -> Self {
        Self { module, name }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StdlibFunctionRegistration {
    pub module: &'static str,
    pub name: &'static str,
    pub signature: &'static str,
}

impl StdlibFunctionRegistration {
    pub const fn new(module: &'static str, name: &'static str, signature: &'static str) -> Self {
        Self {
            module,
            name,
            signature,
        }
    }

    pub(in crate::stdlib::metadata) const fn key(self) -> StdlibFunctionKey {
        StdlibFunctionKey::new(self.module, self.name)
    }

    pub fn parameter_names(self) -> Vec<String> {
        let Some(open) = self.signature.find('(') else {
            return Vec::new();
        };
        let Some(close) = self.signature.rfind(')') else {
            return Vec::new();
        };
        if close <= open {
            return Vec::new();
        }

        let params = self.signature[open + 1..close].trim();
        if params.is_empty() || params == "..." {
            return Vec::new();
        }

        params
            .split(',')
            .filter_map(|param| {
                let name = param.split(':').next()?.trim();
                (!name.is_empty()).then(|| name.to_string())
            })
            .collect()
    }
}
