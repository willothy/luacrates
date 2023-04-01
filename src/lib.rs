use nvim_utils::prelude::*;

const CARGO_TOML: &str = "Cargo.toml";

#[mlua::lua_module]
pub fn crates(lua: &Lua) -> LuaResult<LuaTable> {
    ModuleBuilder::new(lua)
        .with_fn("list", |lua: &Lua, _: ()| {
            let current_dir = std::env::current_dir().map_err(|e| LuaError::external(e))?;

            // Find Cargo.toml in or above current dir
            let cargo_toml = current_dir
                .ancestors()
                .find(|p| p.join(CARGO_TOML).exists())
                .ok_or_else(|| LuaError::RuntimeError("Could not find Cargo.toml".to_owned()))?
                .join(CARGO_TOML);

            let data = std::fs::read_to_string(cargo_toml).map_err(|e| LuaError::external(e))?;

            let toml = toml::from_str::<toml::Value>(&data).map_err(|e| LuaError::external(e))?;

            let deps = toml
                .get("dependencies")
                .ok_or_else(|| LuaError::RuntimeError("Could not find dependencies".to_owned()))?;

            let deps = deps
                .as_table()
                .ok_or_else(|| LuaError::RuntimeError("Dependencies is not a table".to_owned()))?;

            let crates = lua.create_table()?;
            for (k, v) in deps.iter() {
                let version = v
                    .get("version")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_owned();
                crates.set(k.to_owned(), version)?;
            }
            Ok(crates)
        })?
        .build()
}
