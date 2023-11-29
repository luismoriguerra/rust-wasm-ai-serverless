use spin_sdk::{
    http::{IntoResponse, Request},
    http_component, llm,
};

#[http_component]
fn hello_world(_req: Request) -> anyhow::Result<impl IntoResponse> {
    let model = llm::InferencingModel::Llama2Chat;
    let inference = llm::infer(model, "can you tell me a joke about cats?");
    Ok(http::Response::builder()
        .status(200)
        .body(format!("{:?}", inference))?)
}
