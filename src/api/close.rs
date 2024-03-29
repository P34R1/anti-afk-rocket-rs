#[get("/shutdown")]
pub fn shutdown(shutdown: rocket::Shutdown) -> &'static str {
    shutdown.notify();
    "Successfully Shutting Down, Please Close The Current Tab."
}
