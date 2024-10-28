# tino

Run code snippets right in your Telegram chat window.

> _Alice:_ \
> /tioruby[^lang] puts "Hi from Ruby :)"

> _Tino:_ \
> Hi from Ruby :) \
> \
> Real time: 0.131 s \
> User time: 0.076 s \
> Sys. time: 0.026 s \
> CPU share: 77.55 % \
> Exit code: 0

[^lang]: You can replace `ruby` with any other language supported by https://tio.run.

## Getting Started

To host your own instance of tino, you will simply need to:

1. Deploy this one-shot bot in webhook mode on a serverless hosting platform (e.g. Zeabur);

2. Set the `TINO_TELEGRAM_BOT_TOKEN` environment variable to your own token;

3. Set the `HOST` environment variable to your own webhook `<url>`
(e.g. `https://<your-public-domain>.zeabur.app` for Zeabur);

4. Register the webhook URL at Telegram by accessing the following link in your browser:

   ```
   https://api.telegram.org/bot<token>/setWebhook?url=<url>
   ```
