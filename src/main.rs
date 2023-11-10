use warp::Filter;
mod backend;
use dotenv::dotenv;
use backend::handle_connect;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Definir una ruta para manejar las solicitudes POST a '/connect'
    let connect_route = warp::path!("connect")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handle_connect);

    // Combinar todas las rutas
    let routes = connect_route.clone(); // <--- Agrega esto

    // Puerto en el que se ejecutará la aplicación
    let port = 3030;

    println!("Aplicación corriendo en http://127.0.0.1:{}", port);

    // Iniciar el servidor
    warp::serve(routes.clone())
        .run(([127, 0, 0, 1], port))
        .await;
}
