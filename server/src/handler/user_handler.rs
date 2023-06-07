use baodatui_common::message::{requests::AddUserRequest, responses::AddUserResponse};

use super::global_handler_map;

async fn test() {
    global_handler_map().register_handler("command", async move |req: AddUserRequest| {
        Ok(AddUserResponse::default())
    });
}
