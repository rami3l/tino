# tino

Run code snippets right in your Telegram chat window.

> _Alice:_ \
> /tioruby[^lang] puts "Hi from Ruby :)"

> _Tino:_ \
> Hi from Ruby :) \
> [exit(0) in 0.26s]

[^lang]: You can replace `ruby` with any other language supported by https://tio.run.

## Getting Started

To host your own instance of tino, you will simply need to:

1. Deploy this one-shot bot in webhook mode on a serverless hosting platform (e.g. Zeabur);

2. Set the `TINO_TELEGRAM_BOT_TOKEN` environment variable to your own token;

3. Register the webhook URL at Telegram by accessing the following link in your browser:

   ```
   https://api.telegram.org/bot<token>/setWebhook?url=<url>
   ```
