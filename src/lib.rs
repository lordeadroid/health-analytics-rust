use worker::*;

struct Routes;

impl Routes {
  pub const HOME: &'static str = "/";
  pub const GET_DATA: &'static str = "/get-data";
}

fn get_data_handler(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
  Response::ok("This getting all data")
}

#[event(fetch)]
pub async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
  console_error_panic_hook::set_once();

  let router = Router::new();

  router
    .get(Routes::HOME, |_, _| Response::ok("This is a home Method"))
    .get(Routes::GET_DATA, get_data_handler)
    .run(req, env)
    .await
}
