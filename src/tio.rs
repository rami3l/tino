use std::{collections::BTreeMap, io::Write};

use flate2::{
    write::{GzDecoder, ZlibEncoder},
    Compression,
};
use regex::Regex;
use tokio::sync::OnceCell;

use crate::{
    error::{Error, Result},
    model::{Language, LanguageData},
};

#[derive(Clone, Debug)]
pub struct Client {
    /// URL to the TIO main API endpoint.
    pub api: String,
    /// URL to the TIO languages JSON.
    pub langs_json: String,

    inner: reqwest::Client,
}

impl Client {
    pub fn new(api: &str, langs: &str) -> Self {
        Self {
            api: api.to_owned(),
            langs_json: langs.to_owned(),
            inner: reqwest::Client::new(),
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new(
            "https://tio.run/cgi-bin/run/api/",
            "https://tio.run/languages.json",
        )
    }
}

#[derive(Clone, Debug)]
pub enum Payload<'a> {
    One(&'a str),
    Many(&'a [&'a str]),
}

impl Payload<'_> {
    pub fn is_empty(&self) -> bool {
        match self {
            Payload::One(s) => s.is_empty(),
            Payload::Many(v) => v.is_empty(),
        }
    }

    pub fn encode(&self, key: &str) -> Vec<u8> {
        match self {
            p if p.is_empty() => vec![],
            Payload::One(s) => format!("F{key}\0{len}\0{s}\0", len = s.len()).into(),
            Payload::Many(ss) => {
                format!("V{key}\0{len}\0{s}\0", len = ss.len(), s = ss.join("\0")).into()
            }
        }
    }
}

impl Client {
    pub async fn langs(&self) -> Result<&'static [Language]> {
        static CACHE: OnceCell<Vec<Language>> = OnceCell::const_new();

        Ok(CACHE
            .get_or_try_init(|| async {
                let resp = self.inner.get(&self.langs_json).send().await?;
                let langs = dbg!(resp).json::<BTreeMap<String, LanguageData>>().await?;
                reqwest::Result::Ok(
                    langs
                        .into_iter()
                        .map(|(k, d)| Language::new(&k, d))
                        .collect(),
                )
            })
            .await?)
    }

    pub async fn exec(
        &self,
        ExecOpts {
            code,
            lang,
            stdin,
            compiler_flags,
            cli_options,
            args,
        }: ExecOpts<'_>,
    ) -> Result<String> {
        use Payload::*;

        let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());

        for (k, p) in [
            ("lang", Many(&[lang])),
            (".code.tio", One(code)),
            (".input.tio", One(stdin)),
            ("TIO_CFLAGS", Many(compiler_flags)),
            ("TIO_OPTIONS", Many(cli_options)),
            ("args", Many(args)),
        ] {
            enc.write_all(&p.encode(k))?;
        }
        enc.write_all(b"R")?;

        let mut body = enc.finish()?;
        body.truncate(body.len() - 4);
        body.drain(..2);

        let resp = self.inner.post(&self.api).body(body).send().await?;
        if !resp.status().is_success() {
            return Err(resp.into());
        }

        let mut dec = GzDecoder::new(vec![]);
        dec.write_all(&resp.bytes().await?)?;
        let decoded = dec.finish()?;
        let txt = String::from_utf8_lossy(&decoded);
        let tok = &txt[..16];
        let txt = txt.replace(tok, "");
        if txt.lines().count() <= 1 {
            let re =
                Regex::new(r"The language '(?<name>[\w -_]+)' could not be found on the server")
                    .unwrap();
            let lang = re
                .captures(&txt)
                .map_or_else(|| "<unknown>".to_owned(), |c| c["name"].to_owned());
            return Err(Error::LanguageNotFound(lang));
        }
        Ok(txt)
    }
}

#[derive(Clone, Default, Debug)]
pub struct ExecOpts<'a> {
    code: &'a str,
    lang: &'a str,
    stdin: &'a str,
    compiler_flags: &'a [&'a str],
    cli_options: &'a [&'a str],
    args: &'a [&'a str],
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use snapbox::{file, Assert};
    use tokio::test;

    use super::*;

    const TRYCMD: &str = "TRYCMD";

    #[test]
    async fn get_langs() -> Result<()> {
        let client = Client::default();
        let langs = client.langs().await?;
        let langs = langs.iter().map(|l| &*l.name).join("\n");
        Assert::new()
            .action_env(TRYCMD)
            .eq(langs, file!["test_data/langs.txt"]);
        Ok(())
    }

    #[test]
    async fn hello_world() -> Result<()> {
        let client = Client::default();
        let output = client
            .exec(ExecOpts {
                code: r#"with Ada.Text_IO; use Ada.Text_IO; procedure Main is begin Put_Line ("Hello, World!"); end Main;"#,
                lang: "ada-gnat",
                ..ExecOpts::default()
            })
            .await?;
        Assert::new()
            .action_env(TRYCMD)
            .eq(output, file!["test_data/hello_world.txt"]);
        Ok(())
    }

    #[test]
    #[should_panic(expected = r#"LanguageNotFound("i-am-the-walrus")"#)]
    async fn no_such_lang() {
        let client = Client::default();
        let _output = client
            .exec(ExecOpts {
                code: "they-are-the-eggmen",
                lang: "i-am-the-walrus",
                ..ExecOpts::default()
            })
            .await
            .unwrap();
    }
}
