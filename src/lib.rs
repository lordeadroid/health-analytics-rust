use worker::*;

struct Routes;

impl Routes {
  pub const HOME: &'static str = "/";
  pub const GET_DATA: &'static str = "/get-data";
}

fn get_methods(url: Url) -> Result<Response> {
  match url.path() {
    Routes::HOME => Response::ok("This is a home Method "),
    Routes::GET_DATA => Response::ok("This getting all data"),
    _ => Response::error("Not Found", 404),
  }
}

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
  console_error_panic_hook::set_once();

  let method = req.method();
  let url = req.url()?;

  match method {
    Method::Get => get_methods(url),
    _ => Response::ok("This method is not supported"),
  }
}
