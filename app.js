import { SocksProxyAgent } from "socks-proxy-agent";
import { Bot, webhookCallback } from "grammy";
import tio from "tio.js";
import "dotenv/config";
import express from "express";

const app = express();
app.use(express.json());

const bot = getBot();
app.use(webhookCallback(bot, "express"));

function getBot() {
  const botToken = process.env.TINO_TELEGRAM_BOT_TOKEN;
  if (!botToken) {
    console.error("ERROR: `TINO_TELEGRAM_BOT_TOKEN` is missing");
    throw new Error("telegram bot token not found");
  }

  const botNewOpts = {};
  const socksProxy = process.env.SOCKS_PROXY;
  if (socksProxy) {
    console.error(`Using SOCKS proxy: ${socksProxy}`);
    botNewOpts.client = {
      baseFetchConfig: {
        agent: new SocksProxyAgent(socksProxy),
        compress: true,
      },
    };
  }
  const bot = new Bot(botToken, botNewOpts);

  /** @type {string[]} */
  const langs = tio.languages;

  /**
   * @param {CommandContext<Context>} ctx
   * @param {string} txt
   */
  async function replyToMsg(ctx, txt) {
    await ctx.reply(txt, {
      reply_parameters: { message_id: ctx.msg.message_id },
    });
  }

  /** @param {CommandContext<Context>} ctx */
  async function showHelp(ctx) {
    await replyToMsg(
      ctx,
      `Usage: /tio<lang> <code>
e.g. /tiopython3 print("Hello, World!")

Please refer to https://github.com/TryItOnline/tryitonline/tree/master/wrappers for the list of supported languages.`
    );
  }

  bot.command("tio", async (ctx) => {
    console.error("Triggered /tio");
    await showHelp(ctx);
  });

  for (const lang of langs) {
    let cmd = "tio" + lang;
    bot.command(cmd, async (ctx) => {
      console.error(`Triggered /${cmd}`);
      const args = ctx.message.text.split(/ (.*)/s);
      if (args.length < 2 || !langs.includes(lang)) {
        await showHelp(ctx);
        return;
      }
      const [, code] = args;
      const { output, exitCode, realTime } = await tio(code, {
        language: lang,
      });
      await replyToMsg(ctx, `${output}[exit(${exitCode}) in ${realTime}s]`);
    });
  }

  return bot;
}

export default app;
