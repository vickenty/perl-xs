use crate::context::Context;

pub fn boot(ctx: &mut Context, _package: &'static str) {
    use std::collections::HashMap;
    let mut package_rewrites: HashMap<&'static str, &'static str> = HashMap::new();
    for package in crate::PACKAGE_REGISTRY.iter() {
        package_rewrites.insert(package.module, package.package);
    }

    for symbol in crate::SYMBOL_REGISTRY.iter() {
        println!("BOOT - FOUND {:?}", symbol);

        let mut symbol_name: String;

        if let Some(package) = symbol.package {
            symbol_name = package.to_string().replace(" :: ", "::");
        //  Hack ^^ - deprecated macro seems to be adding spaces
        } else {
            symbol_name = symbol.module.to_string();

            if let Some(package_rewrite) = package_rewrites.get(&symbol.module) {
                symbol_name = package_rewrite.to_string();
            } else {
                let mut module_name: &str = &symbol.module;
                let mut non_aliased_parts: Vec<&str> = Vec::new();

                loop {
                    let mut parts = module_name.rsplitn(2, "::");
                    if let (Some(spill), Some(module_name_part)) = (parts.next(), parts.next()) {
                        non_aliased_parts.push(spill);

                        if let Some(package_rewrite) = package_rewrites.get(module_name_part) {
                            symbol_name = package_rewrite.to_string();
                            symbol_name.push_str("::");
                            symbol_name.push_str(&non_aliased_parts.join("::"));
                            break;
                        }
                        module_name = module_name_part;
                    } else {
                        break;
                    }
                }
            }
        }

        symbol_name.push_str("::");
        symbol_name.push_str(symbol.name);

        println!("SYMBOL NAME: {}", symbol_name);
        //
        let cname = ::std::ffi::CString::new(symbol_name).unwrap();
        ctx.new_xs(&cname, symbol.ptr);
    }
}
