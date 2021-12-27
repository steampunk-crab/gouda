extern crate web_view;

use deno_core::op_sync;
use deno_core::JsRuntime;
use web_view::*;

pub fn launch_app(title: &str, url: &str) -> bool {
    let x = "";

    web_view::builder()
        .title(title)
        .content(Content::Url(url))
        .size(640, 480)
        // .frameless(true)
        .debug(true)
        .user_data("")
        .invoke_handler(|webview, arg| {
            match arg {
                "exit" => webview.exit(),
                _ => (),
            }
            Ok(())
        })
        .run()
        .unwrap();

    return false;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::launch_app;

        const URL: &str = "https://ipfs.io/ipfs/QmQPeNsJPyVWPFDVHb77w8G42Fvo15z4bG2X8D2GhfbSXc/readme";

        let result = launch_app("Gouda App", URL);

        assert_eq!(result, false);
    }
}
