use std::str::FromStr;

use crate::core::{compile_time_fnv1a_hash_64};


// -----------------------------------------------------------------------------
// Реальные хэши строк, определенные по алгоритму FNV-1a
// -----------------------------------------------------------------------------
pub const HASH_UNKNOWN: u64 = 0;
pub const HASH_STRING:  u64 = compile_time_fnv1a_hash_64("String");
pub const HASH_BOOL:    u64 = compile_time_fnv1a_hash_64("bool");
pub const HASH_F32:     u64 = compile_time_fnv1a_hash_64("f32");
pub const HASH_U32:     u64 = compile_time_fnv1a_hash_64("u32");
pub const HASH_PIXELS:  u64 = compile_time_fnv1a_hash_64("Pixels");
pub const HASH_PADDING: u64 = compile_time_fnv1a_hash_64("Padding");
pub const HASH_LENGTH:  u64 = compile_time_fnv1a_hash_64("Length");
pub const HASH_COLOR:   u64 = compile_time_fnv1a_hash_64("Color");
pub const HASH_RADIUS:  u64 = compile_time_fnv1a_hash_64("Radius");

pub const HASH_VERTICAL:    u64 = compile_time_fnv1a_hash_64("Vertical");
pub const HASH_HORIZONTAL:  u64 = compile_time_fnv1a_hash_64("Horizontal");
pub const HASH_ALIGNMENT:   u64 = compile_time_fnv1a_hash_64("Alignment");


// -----------------------------------------------------------------------------
// Enum типов
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
    // -------------------------------------------------------------------------
    // Поиск по хэшу: u64 => PropertyType
    // -------------------------------------------------------------------------
    pub fn from_hash(hash: u64) -> Option<Self> {
        match hash {
            HASH_UNKNOWN => Some(Self::Unknown),
            HASH_STRING  => Some(Self::String),
            HASH_BOOL    => Some(Self::Bool),
            HASH_F32     => Some(Self::F32),
            HASH_U32     => Some(Self::U32),
            HASH_PIXELS  => Some(Self::Pixels),
            HASH_PADDING => Some(Self::Padding),
            HASH_LENGTH  => Some(Self::Length),
            HASH_COLOR   => Some(Self::Color),
            HASH_RADIUS  => Some(Self::Radius),

            HASH_VERTICAL   => Some(Self::Vertical),
            HASH_HORIZONTAL => Some(Self::Horizontal),
            HASH_ALIGNMENT  => Some(Self::Alignment),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Получение системного хэша по типу Enum: PropertyType => u64
    // -------------------------------------------------------------------------
    pub fn to_hash(&self) -> u64 {
        match self {
            Self::Unknown => HASH_UNKNOWN,
            Self::String  => HASH_STRING,
            Self::Bool    => HASH_BOOL,
            Self::F32     => HASH_F32,
            Self::U32     => HASH_U32,
            Self::Pixels  => HASH_PIXELS,
            Self::Padding => HASH_PADDING,
            Self::Length  => HASH_LENGTH,
            Self::Color   => HASH_COLOR,
            Self::Radius  => HASH_RADIUS,

            Self::Vertical   => HASH_VERTICAL,
            Self::Horizontal => HASH_HORIZONTAL,
            Self::Alignment  => HASH_ALIGNMENT,
        }
    }

    // -------------------------------------------------------------------------
    // Получение текстового имени: PropertyType => &'static str
    // Генерирует текстовые маркеры типов для выгрузки в Schema JSON файла
    // -------------------------------------------------------------------------
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Unknown => "Unknown",
            Self::String  => "String",
            Self::Bool    => "bool",
            Self::F32     => "f32",
            Self::U32     => "u32",
            Self::Pixels  => "Pixels",
            Self::Padding => "Padding",
            Self::Length  => "Length",
            Self::Color   => "Color",
            Self::Radius  => "Radius",

            Self::Vertical   => "Vertial",
            Self::Horizontal => "Hoirizontal",
            Self::Alignment  => "Alignment",
        }
    }
}

// -----------------------------------------------------------------------------
// Поиск по имени: &'static str => PropertyType
// Нативно вызывается внутри десериализатора PropertyRegistry при чтении JSON
// -----------------------------------------------------------------------------
impl FromStr for PropertyType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "String"     | "string"     => Ok(Self::String),
            "Bool"       | "bool"       => Ok(Self::Bool),
            "F2"         | "f32"        => Ok(Self::F32),
            "U32"        | "u32"        => Ok(Self::F32),
            "Pixels"     | "pixels"     => Ok(Self::Pixels),
            "Padding"    | "padding"    => Ok(Self::Padding),
            "Length"     | "length"     => Ok(Self::Length),
            "Color"      | "color"      => Ok(Self::Color),
            "Radius"     | "radius"     => Ok(Self::Radius),
            "Vertical"   | "vertical"   => Ok(Self::Vertical),
            "Horizontal" | "horizontal" => Ok(Self::Horizontal),
            "Alignment"  | "alignment"  => Ok(Self::Alignment),
            _ => Err(())
        }
    }
}