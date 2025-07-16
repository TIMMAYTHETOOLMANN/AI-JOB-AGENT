use autonomous_job_agent::config::Config;

fn main() {
    let config = Config::load("AgentConfig.toml").unwrap();
    println!("Company targets: {:?}", config.company_targets);
    println!("Manufacturing companies: {:?}", config.manufacturing_companies);
    println!("Technology companies: {:?}", config.technology_companies);
    println!("All companies: {:?}", config.get_all_target_companies());
}
