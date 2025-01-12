use std::{borrow::Cow, sync::LazyLock};

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::Me,
    update_listeners::{self, webhooks, UpdateListener},
};
use tino::{Client, ExecOpts};
use tokio::net::TcpListener;
use tracing::info;

/// The command line options to be collected.
#[derive(Debug, Parser)]
#[clap(
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
    about = clap::crate_description!(),
)]
#[allow(clippy::struct_excessive_bools)]
pub struct Tino {
    /// The Telegram bot token.
    #[clap(long, env = "TINO_TELEGRAM_BOT_TOKEN")]
    token: Option<String>,

    /// The host url when using the webhook listener.
    /// Leave empty to use the long-polling mode.
    #[clap(long, env = "HOST")]
    host: Option<url::Url>,

    /// The port number when using the webhook listener.
    #[clap(long, env = "PORT", default_value = "443")]
    port: u16,
}

impl Tino {
    async fn dispatch(self) -> Result<()> {
        static LANGS: [&str; 681] = include!("test_data/langs.rs");
        static LANGS_REPLACED: LazyLock<[String; 681]> =
            LazyLock::new(|| LANGS.map(|s| s.replace("-", "_")));

        let handler = move |bot: Bot, msg: Message, me: Me| {
            let client = Client::default();
            async move {
                if let Some(rest) = msg.text().and_then(|t| t.strip_prefix("/tio")) {
                    let help_str = r#"Usage: /tio<lang> <code>
e.g. /tiopython3 print("Hello, World!")

Please refer to https://github.com/TryItOnline/tryitonline/tree/master/wrappers for the list of supported languages."#;

                    let mut resp = Cow::from(help_str);
                    if let Some((mut lang, code)) = rest.split_once(' ') {
                        if let Some(stripped) = lang.strip_suffix(&format!("@{}", me.username())) {
                            lang = stripped;
                        }
                        info!("triggered `/tio{lang}`");
                        resp = if let Some(idx) = LANGS_REPLACED.iter().position(|l| l == lang) {
                            client
                                .exec(ExecOpts {
                                    code,
                                    lang: LANGS[idx],
                                    ..ExecOpts::default()
                                })
                                .await
                                .unwrap_or_else(|e| format!("ERROR: {e:#?}"))
                        } else {
                            format!(
                                "Unknown language `{lang}` :[\n(Hint: Telegram doesn't support command names with `-`, maybe try `_` instead?)\n\n"
                            ) + &resp
                        }
                        .into();
                    };
                    bot.send_message(msg.chat.id, resp)
                        .disable_notification(true)
                        .await?;
                }
                Ok(())
            }
        };

        let bot = Bot::new(self.token.with_context(|| anyhow!("bot token not found"))?);
        if let Some(host) = self.host {
            let addr = ([0, 0, 0, 0], self.port).into();
            let opts = webhooks::Options::new(addr, host);
            let (mut listener, _stop_flag, app) = webhooks::axum_no_setup(opts);
            let stop_token = listener.stop_token();

            tokio::spawn(async move {
                let tcp_listener = TcpListener::bind(addr)
                    .await
                    .inspect_err(|_| stop_token.stop())
                    .expect("couldn't bind to the address");
                axum::serve(tcp_listener, app)
                    .await
                    .inspect_err(|_| stop_token.stop())
                    .expect("axum server error");
            });

            teloxide::repl_with_listener(bot, handler, listener).await;
        } else {
            let listener = update_listeners::polling_default(bot.clone()).await;
            teloxide::repl_with_listener(bot, handler, listener).await;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Read the `.env` file. We don't care if it exists or not:
    // finally it's the environment variable that matters.
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    Tino::parse().dispatch().await
}
