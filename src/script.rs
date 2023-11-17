/*
** OpenScape 2023
** script.rs
** File description:
** script.rs
*/

use rlua::{Lua, Result};
use std::fs;

fn test_func(_ctx: rlua::Context, (arg1, arg2): (i32, f64)) -> Result<i32> {
    Ok(arg1 + (arg2 as i32))
}

pub fn run_script() -> Result<()> {
    let script = fs::read_to_string("./src/script/main.lua").expect("Failed to load lua");
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        lua_ctx.load(&script).exec()?;
        lua_ctx.globals().set("test_func", lua_ctx.create_function(test_func)?)?;

        Ok(())
    })
}
