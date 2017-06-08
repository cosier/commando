
pub fn service_enable(project_name: &str, service_name: &str) -> bool{
    debug!("service_enable: {} -> {}", project_name, service_name);
    true
}

pub fn service_disable(project_name: &str, service_name: &str) -> bool {
    debug!("service_disable: {} -> {}", project_name, service_name);
    true
}

pub fn service_start(project_name: &str, service_name: &str) -> bool{
    debug!("service_start: {} -> {}", project_name, service_name);
    true
}

pub fn service_stop(project_name: &str, service_name: &str) -> bool {
    debug!("service_stop: {} -> {}", project_name, service_name);
    true
}
pub fn service_restart(project_name: &str, service_name: &str) -> bool {
    debug!("service_restart: {} -> {}", project_name, service_name);
    true
}

pub fn service_logs(project_name: &str, service_name: &str) -> bool {
    debug!("service_logs: {} -> {}", project_name, service_name);
    true
}

pub fn service_env(project_name: &str, service_name: &str) -> bool {
    debug!("service_env: {} -> {}", project_name, service_name);
    true
}

pub fn service_list(project_name: &str) -> bool {
    debug!("service_list: {}", project_name);
    true
}
