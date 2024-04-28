use actix_web::web;

use super::handler::{create_qr, delete_owner, set_owner};

pub fn route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/create-qr").route(web::post().to(create_qr)));

    cfg.service(web::resource("/set-owner").route(web::post().to(set_owner)));

    cfg.service(web::resource("/delete-owner").route(web::delete().to(delete_owner)));
}
