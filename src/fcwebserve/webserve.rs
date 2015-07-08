use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use fccore::Core;
use std::thread;
use std::sync::{Arc, Mutex,MutexGuard};
use fccore::motors::MotorID;

const TAG : &'static str = "webserve";

fn unknown() -> IronResult<Response> {
    Ok(Response::with((status::NotFound, "unknown command")))
}

fn generate_motor_info(core: &MutexGuard<Core>) -> String {

    let motor1_power = core.motors().motor(MotorID::FrontLeft).current_power();
    let motor2_power = core.motors().motor(MotorID::FrontRight).current_power();
    let motor3_power = core.motors().motor(MotorID::BackLeft).current_power();
    let motor4_power = core.motors().motor(MotorID::BackRight).current_power();

    format!("MOTOR FL: {}<br/>MOTOR FR: {}<br/>MOTOR BL: {}<br/>MOTOR BR: {}<br/>",
        motor1_power, motor2_power, motor3_power, motor4_power)
}

fn status_report(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving status request");
    
    //Generate header
    let boiler_start = format!("<html><head><title>Status</title><body>");
    let header = "<b>STATUS PAGE</b><br/>";
    
    //Generate alive data
    let status_portion = format!("ALIVE: {}<br/>", core.alive);
    
    //Generate accelerometer and gyroscope data
    let (acc_x, acc_y, acc_z) = core.sensors.acc;
    let (gyr_x, gyr_y, gyr_z) = core.sensors.gyro;
    let acc_portion = format!("ACC: ({}, {}, {})<br/>GYR: ({}, {}, {})<br/>", acc_x, acc_y, acc_z, gyr_x, gyr_y, gyr_z);
    
    let motor_portion = generate_motor_info(&core);
    
    //Generate armed data
    let arm_portion = format!("ARM_SAFETY: {}<br/>ARM_COMMAND: {}<br/>FULLY ARMED: {}<br/>", core.armed_switch(), core.armed_cmd(), core.armed());

    //Generate footer
    let boiler_end = format!("</body></html>");
    
    //Generate HTML mime type to send
    let html_content_type : Mime = "text/html".parse::<Mime>().unwrap();
    
    Ok(Response::with((html_content_type, status::Ok, format!("{}{}{}{}{}{}{}", boiler_start, header, status_portion, acc_portion, motor_portion, arm_portion, boiler_end))))
}

fn motor_test(core_ref: &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    
    core.set_motor_power(MotorID::FrontLeft, 25);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 50);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 75);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 100);
    thread::sleep_ms(1000);
    
    core.set_motor_power(MotorID::FrontLeft, 0);
    thread::sleep_ms(0);

    Ok(Response::with((status::Ok, "ok")))
}

fn get_log(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let core = core_ref.lock().unwrap();
    Ok(Response::with((status::Ok, core.log().to_string())))
}

fn get_config(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "serving get config request");
    Ok(Response::with((status::Ok, core.config().to_string())))
}

fn arm_core(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.set_armed_command(true);
    Ok(Response::with((status::Ok, "ok")))
}

fn kill_core(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "arm core network request");
    core.alive = false;
    Ok(Response::with((status::Ok, "ok")))
}

fn disarm_core(core_ref : &Arc<Mutex<Core>>) -> IronResult<Response> {
    let mut core = core_ref.lock().unwrap();
    core.log_mut().add(TAG, "disarm core network request");
    core.set_armed_command(false);
    Ok(Response::with((status::Ok, "ok")))
}

fn page_handler(req : &mut Request, core : &Arc<Mutex<Core>>) -> IronResult<Response> {    	
    
    let mut full_req_path = String::new();
  
    for item in &req.url.path {
        full_req_path = full_req_path + "/" + item;
    }
  
    core.lock().unwrap().log_mut().add(TAG, &format!("Request: {}", full_req_path));
    
    if req.url.path.len() != 0 {
        let base_cmd : &str = &req.url.path[0].clone();
        match base_cmd {
         "arm" => arm_core(core),
         "disarm" => disarm_core(core),
         "log" => get_log(core),
         "kill" => kill_core(core),
         "config" => get_config(core),
         "motor_test" => motor_test(core),
         "status" | _ => status_report(core)
        }
    } else {
        unknown()
    }
}

pub fn spawn(core : &Arc<Mutex<Core>>) {
    let webserve_core = core.clone();
    thread::spawn(move || {
        let webserve_addr_str : &str = &format!("localhost:{}", webserve_core.lock().unwrap().config().fc_webserve_port);
        webserve_core.lock().unwrap().log_mut().add(TAG, &format!("Starting webserve on {}", webserve_addr_str));
        Iron::new(move |req: &mut Request| {
            page_handler(req, &webserve_core)
        }).http(webserve_addr_str).unwrap();
    });
}
