use web_view::*;

fn main() {
    let html_content = "<html><body>
        <video 
            id='player'
            controls
            style='max-width:960px;'
            src='https://l3video.lwcdn.com/v-8fdb4e20-8ebb-4590-8844-dae39680d837.mp4'
            crossOrigin='anonymous'></video>
    </body></html>";
	
    web_view::builder()
        .title("CORS + CSP + crossOrigin issue")
        .content(Content::Html(html_content))
        .size(960, 480)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}