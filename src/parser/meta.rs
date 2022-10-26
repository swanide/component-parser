#[derive(Debug, Serialize)]
pub enum ComponentType {
    Component,
    Page,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Serialize)]
pub struct DataMeta {
    pub name: String,
    pub comment: Option<String>,
    pub loc: Location,
    pub children: Option<Vec<DataMeta>>,
}

impl DataMeta {
    pub fn new(name: String) -> Self {
        let mut instance = Self::default();
        instance.name = name;
        instance
    }

    fn default() -> Self {
        DataMeta {
            name: String::from(""),
            comment: Option::None,
            loc: Location::default(),
            children: Option::None,
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

#[allow(dead_code)]
impl Location {
    pub fn default() -> Self {
        Location {
            start: Position { line: 0, column: 0 },
            end: Position { line: 0, column: 0 },
        }
    }

    pub fn from(start: [usize; 2], end: [usize; 2]) -> Self {
        Location {
            start: Position {
                line: start[0],
                column: start[1],
            },
            end: Position {
                line: end[0],
                column: end[1],
            },
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PropertyMeta {
    pub name: String,
    pub r#type: String,
    pub value: Option<PropertyValue>,
    pub comment: Option<String>,
    pub loc: Location,
}

impl PropertyMeta {
    pub fn new(name: String) -> Self {
        let mut instance = Self::default();
        instance.name = name;
        instance
    }

    fn default() -> Self {
        PropertyMeta {
            name: String::from(""),
            r#type: String::from(""),
            value: Option::None,
            comment: Option::None,
            loc: Location::default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct MethodMeta {
    pub name: String,
    pub comment: Option<String>,
    pub loc: Location,
}

impl MethodMeta {
    pub fn new(name: String) -> Self {
        let mut instance = Self::default();
        instance.name = name;
        instance
    }

    fn default() -> Self {
        MethodMeta {
            name: String::from(""),
            comment: Option::None,
            loc: Location::default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct EventMeta {
    pub name: String,
    pub comment: Option<String>,
    pub loc: Location,
}

impl EventMeta {
    pub fn new(name: String) -> Self {
        let mut instance = Self::default();
        instance.name = name;
        instance
    }

    fn default() -> Self {
        EventMeta {
            name: String::from(""),
            comment: Option::None,
            loc: Location::default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ComponentMeta {
    pub r#type: ComponentType,
    pub data: Vec<DataMeta>,
    pub properties: Vec<PropertyMeta>,
    pub methods: Vec<MethodMeta>,
    pub events: Option<Vec<EventMeta>>,
}

impl ComponentMeta {
    pub fn new(r#type: ComponentType) -> Self {
        ComponentMeta {
            r#type: r#type,
            data: vec![],
            properties: vec![],
            methods: vec![],
            events: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CssClassMeta {
    pub name: String,
    pub loc: Location,
}

impl CssClassMeta {
    pub fn new(name: String) -> Self {
        let mut instance = Self::default();
        instance.name = name;
        instance
    }

    fn default() -> Self {
        CssClassMeta {
            name: String::from(""),
            loc: Location::default(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CssMeta {
    pub classes: Vec<CssClassMeta>,
    pub imports: Option<Vec<String>>,
}

impl CssMeta {
    pub fn new(classes: Vec<CssClassMeta>) -> Self {
        CssMeta {
            classes,
            imports: None,
        }
    }
}
