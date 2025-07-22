use clap::*;

//
// Version
//

/// Clap command to print version.
#[derive(Args, Clone, Debug)]
pub struct Version;

impl Version {
    /// Run command.
    pub fn run<ParserT>(&self)
    where
        ParserT: Parser,
    {
        print!("{}", ParserT::command().render_version());

        println!("build-timestamp: {}", build_info::format!("{}", $.timestamp));
        println!("build-target-triple: {}", build_info::format!("{}", $.target.triple));
        println!("build-target-family: {}", build_info::format!("{}", $.target.family));
        println!("build-target-os: {}", build_info::format!("{}", $.target.os));
        println!("build-target-cpu: {}", build_info::format!("{}", $.target.cpu));
        println!("build-profile: {}", build_info::format!("{}", $.profile));
        println!("build-optimization: {}", build_info::format!("{}", $.optimization_level));

        println!("rustc-version: {}", build_info::format!("{}", $.compiler.version));
    }
}
