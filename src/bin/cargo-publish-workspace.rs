use anyhow::Context;
use cargo_publish_workspace::CargoPublishWSError;

fn main() -> anyhow::Result<()> {
    Err(CargoPublishWSError::TodoError).with_context(|| "Unable to run cargo-publish-workspace")
}
