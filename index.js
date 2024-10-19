import { SocksProxyAgent } from "socks-proxy-agent";
import { Telegraf } from "telegraf";
import { default as tio } from "tio.js";
import "dotenv/config";

const botToken = process.env.TINO_TELEGRAM_BOT_TOKEN;
if (!botToken) {
  console.error("ERROR: `TINO_TELEGRAM_BOT_TOKEN` is missing");
  process.exit(1);
}

const botNewOpts = {};
const socksProxy = process.env.SOCKS_PROXY;
if (socksProxy) {
  console.error(`Using SOCKS proxy: ${socksProxy}`);
  botNewOpts.telegram = { agent: new SocksProxyAgent(socksProxy) };
}
const bot = new Telegraf(botToken, botNewOpts);

var botWebhookOpts = null;
const botWebhookListen = process.env.TINO_TELEGRAM_BOT_WEBHOOK_LISTEN;
if (botWebhookListen) {
  const domain = botWebhookListen;
  console.error(`Setting up webhook mode at \`${domain}\``);
  const port = Number(process.env.PORT) || 443;
  console.error(`Will be listening on port \`${port}\``);
  botWebhookOpts = { domain, port };
}

/** @type {string[]} */
const langs = tio.languages;

async function showLangs(ctx) {
  await ctx.reply(
    "Please refer to https://github.com/TryItOnline/tiosetup/tree/master/languages for a list of supported languages.",
    { reply_to_message_id: ctx.message.message_id }
  );
}

bot.command("tio", async (ctx) => {
  console.error("Triggered /tio");
  await showLangs(ctx);
});

langs.forEach((lang) => {
  let cmd = "tio" + lang;
  bot.command(cmd, async (ctx) => {
    console.error(`Triggered /${cmd}`);
    const args = ctx.message.text.split(/ (.*)/s);
    if (args.length < 2) {
      await ctx.reply("Usage: /tio<lang> <code>", {
        reply_to_message_id: ctx.message.message_id,
      });
      return;
    }
    if (!langs.includes(lang)) {
      await showLangs(ctx);
      return;
    }
    const [, code] = args;
    const { output, exitCode, realTime } = await tio(code, { language: lang });
    await ctx.reply(`${output}[exit(${exitCode}) in ${realTime}s]`, {
      reply_to_message_id: ctx.message.message_id,
    });
  });
});

const botLaunchOpts = {};
if (botWebhookOpts) {
  botLaunchOpts.webhook = botWebhookOpts;
}
bot.launch(botLaunchOpts);
console.log("The bot is now running...");

process.once("SIGINT", () => bot.stop("SIGINT"));
process.once("SIGTERM", () => bot.stop("SIGTERM"));
