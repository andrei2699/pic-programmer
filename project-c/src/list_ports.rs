pub fn list_ports() {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.is_empty() {
                println!("no ports detected");
                return;
            }

            for p in ports {
                println!("{}", p.port_name);
            }
        }
        Err(e) => eprintln!("Error listing ports: {:?}", e),
    }
}