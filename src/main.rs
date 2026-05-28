mod db_service;

use db_service::connection;

fn main() {
    let _ = connection::connect();
}
