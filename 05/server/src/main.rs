use std::sync::Arc;

use axum::{
    Router, serve,
    routing::get,
    response::{
        IntoResponse, Html
    }
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Iniciando server...");

        let cors = CorsLayer::new()
        .allow_origin(Any) 
        .allow_methods(Any) 
        .allow_headers(Any); 

    let app: Router = Router::new()
        .route("/", get(root))
        .route("/api/alunos", get(get_alunos_table))
        .layer(cors)
        ;    

    let listener = TcpListener::bind("127.0.0.1:30000").await?;

    println!("Server iniciado!");

    serve(listener, app.into_make_service()).await?;

    Ok(())
}

static alunos_mock: std::sync::LazyLock<Arc<[Aluno]>> = 
std::sync::LazyLock::new(|| {
    Arc::new([
        Aluno { nome: String::from("Marquin"), curso: String::from("ES"), ira: 9.34},
        Aluno { nome: String::from("Henriquius"), curso: String::from("ES"), ira: 9.1111},
        Aluno { nome: String::from("Bito"), curso: String::from("SI"), ira: 8.999},
        Aluno { nome: String::from("Irineu"), curso: String::from("CC"), ira: 2.02},
    ])
});

#[derive(Debug)]
struct Aluno {
    nome: String,
    curso: String,
    ira: f64,
}

async fn root() -> impl IntoResponse {
    Html("<p>Nada pra ver aqui...</p>")
}

async fn get_alunos_table() -> impl IntoResponse {
    let mut html_content = String::from(
        "
        <h2>Tabela de Alunos</h2>
        <table>
            <tr>
                <th>Nome</th>
                <th>Curso</th>
                <th>IRA</th>
            </tr>",
    );

    for aluno in alunos_mock.iter() {
        html_content.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{:.2}</td>
            </tr>",
            aluno.nome, aluno.curso, aluno.ira
        ));
    }

    html_content.push_str("</table>");

    Html(html_content)
}