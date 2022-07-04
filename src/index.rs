use crate::ast;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Index(HashMap<String, Item>);

pub fn index(ast: &[ast::Item]) -> Index {
    let mut map = HashMap::new();

    for item in ast {
        match item {
            ast::Item::Function { name, params, return_ty, body: _ } => {
                let params = params.iter().map(|(name, ty)| (name.clone(), lower_ty(ty))).collect();
                let return_ty = lower_ty(return_ty);
                map.insert(name.clone(), Item::Function { params, return_ty });
            }

            ast::Item::Struct { name, fields } => {
                let fields = fields.iter().map(|(name, ty)| (name.clone(), lower_ty(ty))).collect();
                map.insert(name.clone(), Item::Struct { fields });
            }
        }
    }

    Index(map)
}

fn lower_ty(ty: &ast::Ty) -> Ty {
    match ty {
        ast::Ty::Void => Ty::Void,
        ast::Ty::Named(name) => Ty::Named(name.clone()),
        ast::Ty::Pointer(ty) => Ty::Pointer(Box::new(lower_ty(ty))),
    }
}

#[derive(Debug)]
pub enum Item {
    Function { params: Vec<(String, Ty)>, return_ty: Ty },
    Struct { fields: Vec<(String, Ty)> },
}

#[derive(Debug)]
pub enum Ty {
    Void,
    Named(String),
    Pointer(Box<Ty>),
}