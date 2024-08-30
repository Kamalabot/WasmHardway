use wasm_bindgen::prelude::*;
use web_sys::{console, Document, HtmlInputElement, HtmlButtonElement, Window};
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};
use candle_core::{Device, Tensor};
// use candle_nn::VarBuilder;
use hf_hub::{api::sync::Api, Repo, RepoType};
use tokenizers::{PaddingParams, Tokenizer};
use anyhow::{Error as E, Result};


#[wasm_bindgen]
pub fn build_model_and_tokenizer(prompt: String) -> Vec<u32> {
    // let device = Device::Cpu;
    let default_model = "sentence-transformers/all-MiniLM-L6-v2".to_string();
    let default_revision = "refs/pr/21".to_string();
    // good way to check if arg is present or not
    // let (model_id, revision) = match (model_id.to_owned(), revision.to_owned()) {
    //     (Some(model_id), Some(revision)) => (model_id, revision),
    //     (Some(model_id), None) => (model_id, "main".to_string()),
    //     (None, Some(revision)) => (default_model, revision),
    //     (None, None) => (default_model, default_revision),
    // };

    let repo = Repo::with_revision(default_model, 
        RepoType::Model, default_revision);

    let (config_filename, tokenizer_filename, weights_filename) = {
        let api = Api::new().unwrap();
        let api = api.repo(repo);
        let config = api.get("config.json").unwrap();
        let tokenizer = api.get("tokenizer.json").unwrap();
        let weights = api.get("pytorch_model.bin").unwrap();
        (config, tokenizer, weights)
    };

    let config = std::fs::read_to_string(config_filename).unwrap();
    let mut config: Config = serde_json::from_str(&config).unwrap();
    let mut tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg).unwrap();

    // let vb = VarBuilder::from_pth(&weights_filename,
    //     DTYPE, &device).unwrap();
    // config.hidden_act = HiddenAct::GeluApproximate;
    
    // let model = BertModel::load(vb, &config)?;

    let tokenizer = tokenizer
        .with_padding(None)
        .with_truncation(None)
        .map_err(E::msg).unwrap();

    let tokens = tokenizer
        .encode(prompt, true)
        .map_err(E::msg)
        .unwrap()
        .get_ids()
        .to_vec();

    // let token_ids = Tensor::new(&tokens[..], &device)?.unsqueeze(0)?;
    // let token_type_ids = token_ids.zeros_like()?; 

    tokens
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window: Window = web_sys::window().unwrap();
    let document: Document = window.document().unwrap();

    // Set up input field
    let input_field = document.create_element("input")?.dyn_into::<HtmlInputElement>()?;
    input_field.set_id("inputText");
    document.body().unwrap().append_child(&input_field)?;

    // Set up button
    let button = document.create_element("button")?.dyn_into::<HtmlButtonElement>()?;
    button.set_inner_html("Run Tokenizer");
    document.body().unwrap().append_child(&button)?;

    // Set up result display
    let result_span = document.create_element("span")?;
    result_span.set_id("result");
    document.body().unwrap().append_child(&result_span)?;

    let closure = Closure::wrap(Box::new(move || {
        let input_value_element = document
            .get_element_by_id("inputText")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();

        let input_text = input_value_element.value();
        let result = load_and_run_model(&input_text);

        let result_element = document.get_element_by_id("result").unwrap();
        result_element.set_inner_html(&result);
    }) as Box<dyn FnMut()>);

    button.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    Ok(())
}
