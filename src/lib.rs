#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc_plugin;

use rustc_plugin::Registry;
use syntax::ext::base::*;
use syntax::codemap::*;
use syntax::ast::*;

pub fn get<T>(n: &str) -> Option<T> {
	None
}

pub struct StaticGroupItemDecorator { x: bool }

impl MultiItemDecorator for StaticGroupItemDecorator {
	fn expand(&self, ecx: &mut ExtCtxt, sp: Span, meta_item: &MetaItem, item: &Annotatable, push: &mut FnMut(Annotatable)) {
		match item {
			&Annotatable::Item(ref really_item) => {
				match really_item.node {
					ItemKind::Fn(ref decl, ref style, ref constness, ref abi, ref generics, ref block) => {
						// trace!("{}", decl);
						// ...? Add something here.
					}
					_ => {
						ecx.span_err(sp, "static_group is only permissiable on functions for now");
					}
				}
			}
			_ => {
				ecx.span_err(sp, "static_group is only permissiable on functions for now");
			}
		}
	}
}

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
	use syntax::parse::token::intern;
	use syntax::ext::base;

	let static_group_item_decorator = Box::new(StaticGroupItemDecorator { x: true });

	reg.register_syntax_extension(intern("static_group"), SyntaxExtension::MultiDecorator(static_group_item_decorator));
}
