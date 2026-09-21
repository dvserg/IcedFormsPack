use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;

// -----------------------------------------------------------------------------
// Типы данных для реестра Schema-полей
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyType {
    Unknown,
    String,
    Bool,
    F32,
    U32,
    Pixels,
    Padding,
    Length,
    Color,
    Radius,
    Vertical,
    Horizontal,
    Alignment,
}

impl PropertyType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::String => "String",
            Self::Bool => "bool",
            Self::F32 => "f32",
            Self::U32 => "u32",
            Self::Pixels => "Pixels",
            Self::Padding => "Padding",
            Self::Length => "Length",
            Self::Color => "Color",
            Self::Radius => "Radius",
            Self::Vertical => "Vertical",
            Self::Horizontal => "Horizontal",
            Self::Alignment => "Alignment",
        }
    }
}

impl FromStr for PropertyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "String" | "string" => Ok(Self::String),
            "Bool" | "bool" => Ok(Self::Bool),
            "F32" | "f32" => Ok(Self::F32),
            "U32" | "u32" => Ok(Self::U32),
            "Pixels" | "pixels" => Ok(Self::Pixels),
            "Padding" | "padding" => Ok(Self::Padding),
            "Length" | "length" => Ok(Self::Length),
            "Color" | "color" => Ok(Self::Color),
            "Radius" | "radius" => Ok(Self::Radius),
            "Vertical" | "vertical" => Ok(Self::Vertical),
            "Horizontal" | "horizontal" => Ok(Self::Horizontal),
            "Alignment" | "alignment" => Ok(Self::Alignment),
            _ => Ok(Self::Unknown),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyMetadata {
    pub name: String,
    pub raw_type_name: String,
    pub prop_type: PropertyType,
}

#[derive(Debug, Clone, Default)]
pub struct PropertyRegistry {
    data: BTreeMap<String, PropertyMetadata>,
}

impl PropertyRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, prop_name: impl Into<String>, prop_type: PropertyType) {
        let name = prop_name.into();
        let prop_type_name = prop_type.as_str().to_string();
        self.data.insert(
            name.clone(),
            PropertyMetadata {
                name,
                raw_type_name: prop_type_name,
                prop_type,
            },
        );
    }

    pub fn get(&self, prop_name: &str) -> Option<&PropertyMetadata> {
        self.data.get(prop_name)
    }

    pub fn get_type(&self, prop_name: &str) -> PropertyType {
        self.data
            .get(prop_name)
            .map(|meta| meta.prop_type)
            .unwrap_or(PropertyType::Unknown)
    }

    pub fn contains(&self, prop_name: &str) -> bool {
        self.data.contains_key(prop_name)
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn validate_value_type(&self, prop_name: &str, value: &serde_json::Value) -> bool {
        match self.get_type(prop_name) {
            PropertyType::Bool => value.is_boolean(),
            PropertyType::F32 | PropertyType::U32 | PropertyType::Pixels => value.is_number() || value.is_string(),
            PropertyType::Padding | PropertyType::Length | PropertyType::Radius => value.is_object(),
            PropertyType::String | PropertyType::Color => value.is_string(),
            PropertyType::Vertical => value.as_str().map(|s| matches!(s.trim(), "Top" | "top" | "Start" | "start" | "Center" | "center" | "Middle" | "middle" | "Bottom" | "bottom" | "End" | "end")).unwrap_or(false),
            PropertyType::Horizontal => value.as_str().map(|s| matches!(s.trim(), "Left" | "left" | "Start" | "start" | "Center" | "center" | "Middle" | "middle" | "Right" | "right" | "End" | "end")).unwrap_or(false),
            PropertyType::Alignment => value.is_string(),
            PropertyType::Unknown => true,
        }
    }
}

impl<'de> Deserialize<'de> for PropertyRegistry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw_map = BTreeMap::<String, String>::deserialize(deserializer)?;
        let mut registry = Self::new();

        for (prop_name, type_name) in raw_map {
            let prop_type = PropertyType::from_str(&type_name).unwrap_or(PropertyType::String);
            registry.register(prop_name, prop_type);
        }

        Ok(registry)
    }
}

impl Serialize for PropertyRegistry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = BTreeMap::new();
        for (name, meta) in &self.data {
            map.insert(name.clone(), meta.raw_type_name.clone());
        }
        map.serialize(serializer)
    }
}

// -----------------------------------------------------------------------------
// Структуры иерархии виджетов и их свойств
// -----------------------------------------------------------------------------
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetMeta {
    pub id: String,
    pub local_index: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "serde_json::Value", into = "serde_json::Value")]
pub struct WidgetProperties {
    #[serde(flatten)]
    pub fields: BTreeMap<String, serde_json::Value>,
}

impl From<serde_json::Value> for WidgetProperties {
    fn from(value: serde_json::Value) -> Self {
        let map = value
            .as_object()
            .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        Self { fields: map }
    }
}

impl From<WidgetProperties> for serde_json::Value {
    fn from(value: WidgetProperties) -> Self {
        let mut obj = serde_json::Map::new();
        for (key, val) in value.fields {
            obj.insert(key, val);
        }
        serde_json::Value::Object(obj)
    }
}

impl WidgetProperties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.fields.get(key).and_then(|value| value.as_str())
    }

    pub fn get_f32(&self, key: &str) -> f32 {
        let Some(value) = self.fields.get(key) else {
            return 0.0;
        };

        match value {
            serde_json::Value::Number(number) => number.as_f64().unwrap_or(0.0) as f32,
            serde_json::Value::String(s) => s.trim().parse::<f32>().unwrap_or(0.0),
            _ => 0.0,
        }
    }

    pub fn get_bool(&self, key: &str) -> bool {
        self.fields.get(key).and_then(|value| value.as_bool()).unwrap_or(false)
    }

    pub fn set_parent(&mut self, parent_id: &str) {
        self.fields.insert("parent".to_string(), serde_json::Value::String(parent_id.to_string()));
    }

    pub fn parent_id(&self) -> &str {
        self.get_str("parent").unwrap_or("root")
    }

    
    pub fn _prepare_generator_codes(&mut self, registry: &PropertyRegistry) {
        let keys: Vec<String> = self
            .fields
            .keys()
            .filter(|key| *key != "parent")
            .cloned()
            .collect();

        for key in keys {
            let value = self.fields[&key].clone();
            let prop_type = registry.get_type(&key);

            if let Some(str_value) = value.as_str() {
                let normalized = str_value.trim();
                match prop_type {
                    PropertyType::Color if normalized == "transparent" => {
                        self.fields.insert(format!("{}_code", key), serde_json::Value::String("iced::Color::TRANSPARENT".to_string()));
                    }
                    PropertyType::Color if normalized.starts_with('#') => {
                        let digits = normalized.trim_start_matches('#');
                        if digits.len() == 6 {
                            if let (Ok(r), Ok(g), Ok(b)) = (
                                u8::from_str_radix(&digits[0..2], 16),
                                u8::from_str_radix(&digits[2..4], 16),
                                u8::from_str_radix(&digits[4..6], 16),
                            ) {
                                let r = r as f32 / 255.0;
                                let g = g as f32 / 255.0;
                                let b = b as f32 / 255.0;
                                self.fields.insert(
                                    format!("{}_code", key),
                                    serde_json::Value::String(format!("iced::Color::from_rgb({:.3}, {:.3}, {:.3})", r, g, b)),
                                );
                                continue;
                            }
                        }
                        self.fields.insert(format!("{}_code", key), serde_json::Value::String("iced::Color::BLACK".to_string()));
                    }
                    PropertyType::F32 | PropertyType::U32 | PropertyType::Pixels if !normalized.is_empty() => {
                        if let Ok(number) = normalized.parse::<f64>() {
                            self.fields.insert(format!("{}_code", key), serde_json::Value::String(format!("{:.1}", number)));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetNode {
    #[serde(rename = "type")]
    pub widget_type: String,
    pub meta: WidgetMeta,
    #[serde(default)]
    pub properties: WidgetProperties,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct CadProject {
    pub field_counter: usize,
    #[serde(rename = "types_registry")]
    pub property_registry: PropertyRegistry,
    pub widgets: IndexMap<String, WidgetNode>,
    pub widgets_order: Vec<String>,
}

impl CadProject {
    pub fn new() -> Self {
        Self {
            field_counter: 0,
            property_registry: PropertyRegistry::new(),
            widgets: IndexMap::new(),
            widgets_order: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct WidgetMappingConfig {
    pub aliases: HashMap<String, String>,
    pub templates: HashMap<String, String>,
}

impl WidgetMappingConfig {
    pub fn load_from_file(path: &str) -> Self {
        let path = std::path::Path::new(path);

        if !path.exists() {
            return Self::default();
        }

        match std::fs::read_to_string(path) {
            Ok(content) => {
                let mut config = Self::default();
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        continue;
                    }
                    if let Some((key, value)) = trimmed.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();
                        if key.starts_with("alias.") {
                            config.aliases.insert(key.trim_start_matches("alias.").to_string(), value.to_string());
                        } else if key.starts_with("template.") {
                            config.templates.insert(key.trim_start_matches("template.").to_string(), value.to_string());
                        }
                    }
                }
                config
            }
            Err(_) => Self::default(),
        }
    }
}
