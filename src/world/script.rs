/*
** OpenScape 2023
** script.rs
** File description:
** script.rs
*/

use rlua::{Lua, Result};
use std::fs;
use crate::world::create_block;
use crate::world::remove_block;

const SQUARE_SIZE: i32 = 50;

pub fn run_script() -> Result<()> {
    let script = fs::read_to_string("./src/script/main.lua").expect("Failed to load lua");
    let lua = Lua::new();

    lua.context(|lua_ctx| {
        lua_ctx.globals().set(
            "create_block",
            lua_ctx.create_function_mut(|_, (x, y): (i32, i32)| {
                create_block(x * SQUARE_SIZE, y * SQUARE_SIZE);
                Ok(())
            })?,
        )?;
        lua_ctx.globals().set(
            "remove_block",
            lua_ctx.create_function_mut(|_, (x, y): (i32, i32)| {
                remove_block(x * SQUARE_SIZE, y * SQUARE_SIZE);
                Ok(())
            })?,
        )?;
        lua_ctx.load(&script).exec()?;

        Ok(())
    })
}
