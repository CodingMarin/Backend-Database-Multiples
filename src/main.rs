// proyecto/src/main.rs
use warp::Filter;

mod backend;
use backend::{handle_connect, DatabaseConnection};

#[tokio::main]
async fn main() {
    // Definir una ruta para manejar las solicitudes POST a '/connect'
    let connect_route = warp::path!("connect")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_connect);

    // Combinar todas las rutas
    let routes = connect_route;

    // Puerto en el que se ejecutará la aplicación
    let port = 3030;

    // Iniciar el servidor
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;

    println!("Aplicación corriendo en http://127.0.0.1:{}", port);
}
