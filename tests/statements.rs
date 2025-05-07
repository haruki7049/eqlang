use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn example_statements() -> anyhow::Result<()> {
    let mut cmd = Command::cargo_bin("eqlang")?;

    // !!?!?!?? <- Plus
    // !??!?!!? <- Nine
    // !????!!? <- Eight
    cmd.arg("!!?!?!??!??!?!!?!????!!?");
    cmd.assert().success();

    Ok(())
}
