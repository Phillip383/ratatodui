mod db_service;

use db_service::{db_connect, db_crud};

fn main() {
    let _ = db_connect::connect();
}
