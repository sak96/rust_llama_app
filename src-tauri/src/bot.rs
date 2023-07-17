use std::convert::Infallible;

use llm::{models::Llama, InferenceFeedback, InferenceSession, KnownModel};

const CHARACTER_NAME: &str = "### Assistant";
const USER_NAME: &str = "### User";
const PERSONA: &str = r#"
A chat between a human ("User") and an AI assistant ("Assistant").
The assistant gives helpful, detailed, and polite answers to the human's questions.
### Assistant: How may I help you?
### User:
"#;

pub struct ChatBot {
    model: Llama,
    session: InferenceSession,
}

impl ChatBot {
    pub fn new(path: &std::path::PathBuf) -> Self {
        let model = llm::load::<Llama>(
            path,
            llm::TokenizerSource::Embedded,
            Default::default(),
            llm::load_progress_callback_stdout,
        )
        .unwrap_or_else(|err| panic!("Failed to load model from {path:?}: {err}"));
        let session = Self::make_session(&model);
        Self { model, session }
    }

    fn make_session(model: &Llama) -> InferenceSession {
        let mut session = model.start_session(Default::default());
        session
            .feed_prompt(
                model,
                &mut Default::default(),
                PERSONA,
                &mut Default::default(),
                llm::feed_prompt_callback(|resp| match resp {
                    llm::InferenceResponse::PromptToken(t)
                    | llm::InferenceResponse::InferredToken(t) => {
                        Ok::<llm::InferenceFeedback, Infallible>(llm::InferenceFeedback::Continue)
                    }
                    _ => Ok(llm::InferenceFeedback::Continue),
                }),
            )
            .expect("Failed to ingest initial prompt.");
        session
    }

    fn inference_callback<'a>(
        stop_sequence: String,
        buf: &'a mut String,
        out_str: &'a mut String,
    ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
        move |resp| match resp {
            llm::InferenceResponse::InferredToken(t) => {
                let mut reverse_buf = buf.clone();
                reverse_buf.push_str(t.as_str());
                if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                    buf.clear();
                    return Ok(InferenceFeedback::Halt);
                } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                    buf.push_str(t.as_str());
                    return Ok(InferenceFeedback::Continue);
                }
                if buf.is_empty() {
                    out_str.push_str(&t);
                } else {
                    out_str.push_str(&reverse_buf);
                }
                Ok(InferenceFeedback::Continue)
            }
            llm::InferenceResponse::EotToken => Ok(InferenceFeedback::Halt),
            _ => Ok(InferenceFeedback::Continue),
        }
    }

    pub fn get_reply(&mut self, message: &str) -> String {
        let mut res = String::new();
        let mut buf = String::new();
        let mut rng = rand::thread_rng();
        self.session
            .infer(
                &self.model,
                &mut rng,
                &llm::InferenceRequest {
                    prompt: format!("{message}\n{CHARACTER_NAME}: ").as_str().into(),
                    parameters: &llm::InferenceParameters::default(),
                    play_back_previous_tokens: false,
                    maximum_token_count: None,
                },
                &mut Default::default(),
                Self::inference_callback(String::from(USER_NAME), &mut buf, &mut res),
            )
            .unwrap_or_else(|e| panic!("{e}"));
        res
    }
}
