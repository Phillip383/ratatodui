mod db_service;

fn main() {
    let _ = db_service::connection::connect();
}
