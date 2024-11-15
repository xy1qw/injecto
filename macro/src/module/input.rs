use syn::{
    braced,
    parse::{Parse, ParseStream},
    Attribute, Ident, ItemFn, Result, Visibility,
};

#[derive(Default)]
pub struct FunctionAttributes {
    pub is_singleton: bool,
    pub is_injectable: bool,
}

pub struct ModuleInput {
    pub vis: Visibility,
    pub name: Ident,
    pub functions: Vec<(ItemFn, FunctionAttributes)>,
}

impl Parse for ModuleInput {
    fn parse(input: ParseStream) -> Result<Self> {
        // Парсим видимость и имя модуля
        let vis: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        // Парсим содержимое в фигурных скобках
        let content;
        braced!(content in input);

        // Парсим функции и их атрибуты
        let mut functions = Vec::new();
        while !content.is_empty() {
            let func: ItemFn = content.parse()?;
            let attrs = parse_function_attributes(&func.attrs);
            functions.push((func, attrs));
        }

        Ok(ModuleInput {
            vis,
            name,
            functions,
        })
    }
}

fn parse_function_attributes(attrs: &[Attribute]) -> FunctionAttributes {
    let mut attributes = FunctionAttributes::default();
    for attr in attrs {
        if attr.path().is_ident("singleton") {
            attributes.is_singleton = true;
        } else if attr.path().is_ident("injectable") {
            attributes.is_injectable = true;
        }
    }
    attributes
}
