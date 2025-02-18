use crate::util::output::Output;
use crate::util::{node_rpc, print_command_response};
use crate::CommandGlobalOpts;
use clap::Args;
use core::fmt::Write;
use ockam::Context;
use ockam_api::nodes::models::identity::{LongIdentityResponse, ShortIdentityResponse};
use ockam_identity::change_history::IdentityChangeHistory;

#[derive(Clone, Debug, Args)]
pub struct ShowCommand {
    #[arg()]
    name: String,
    #[arg(short, long)]
    full: bool,
}

impl ShowCommand {
    pub fn run(self, options: CommandGlobalOpts) {
        node_rpc(run_impl, (options, self))
    }
}

async fn run_impl(_: Context, (opts, cmd): (CommandGlobalOpts, ShowCommand)) -> crate::Result<()> {
    let state = opts.state.identities.get(&cmd.name)?;
    if cmd.full {
        let identity = state.config.change_history.export()?;
        let response = LongIdentityResponse::new(identity);
        print_command_response(response, &opts.global_args.output_format)?;
    } else {
        let response = ShortIdentityResponse::new(state.config.identifier.to_string());
        print_command_response(response, &opts.global_args.output_format)?;
    }
    Ok(())
}

impl Output for LongIdentityResponse<'_> {
    fn output(&self) -> anyhow::Result<String> {
        let mut w = String::new();
        let id: IdentityChangeHistory = serde_bare::from_slice(self.identity.0.as_ref())?;
        write!(w, "{}", id)?;
        Ok(w)
    }
}

impl Output for ShortIdentityResponse<'_> {
    fn output(&self) -> anyhow::Result<String> {
        let mut w = String::new();
        write!(w, "{}", self.identity_id)?;
        Ok(w)
    }
}
