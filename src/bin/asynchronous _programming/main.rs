// // 典型的なスレッドを使ったアプリケーションでは、2つのWeb ページを同時にダウンロードしたい時、
// // 次のように 2 つのスレッドに作業を分散する

// use std::thread;

// fn get_two_sites() {
//     // 2つのtheadを作成
//     let thread_one = thread::spawn(|| download ("https::www.foo.com"));
//     let thread_two = thread::spawn(|| download ("https::www.bar.com"));
//     // 両方のthreadが完了するまで待つ
//     thread_one.join().expect("thread one panicked");
//     thread_two.join().expect("thread two panicked");
// }

// // async/.await記法を使って、先ほどのコードを書き換える
// // 非同期アプリケーションはスレッド実装よりも高速で、使うリソースも少なくなる可能性がある
// async fn get_two_sites_async() {

//     let future_one = download_async("https::www.foo.com");
//     let future_two = download_async("https::www.bar.com");

//     join!(future_one,future_two);
// }


// use futures::executor::block_on;

// async fn hellow_world(){
//     println!("hello,world");
// }

// fn main(){
//     let future = hello_world();
//     block_on(future);
// }

use {
hyper::{
    Body, Client, Request, Response, Server, Uri,
    service::service_fn,
    rt::run,
},
futures::{
    compat::Future01CompatExt,
    future::{FutureExt, TryFutureExt},
},
std::net::SocketAddr,
};
async fn serve_req(_req:Request<Body>) ->Result<Response<Body>, hyper::Error> {
    Ok(Response::new(Body::from("hello,world")))
}
async fn run_server(add:SocketAddr) {
    println!("listening on http://{}",addr);

    let serve_future = Server::bind(&addr).serve(|| service_fn(|req| serve_req(req).boxed().compat()));
    if let Err(e) = serve_future.compat().await {
        eprintln!("server error: {}",e);
    }
}

fn main(){
    let addr = SocketAddr::from(([127,0,0,1],3000));

    let futures_03_future = run_server(addr);
    let futures_01_future = futures_03_future.unit_error().boxed.compat();

    run(futures_01_future);
}