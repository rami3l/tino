// Adapted from https://github.com/zeabur/expressjs-template.

import { default as esbuild } from "esbuild";
import { default as fs } from "fs";

// dynamic-required files
const dynamicRequiredDirs = [
  // "views"
];

// static files
const staticFileDirs = [
  // "public"
];

// Remove old output
if (fs.existsSync(".zeabur/output")) {
  console.info("Removing old .zeabur/output");
  fs.rmSync(".zeabur/output", { recursive: true });
}

function getModuleEntries() {
  function getModuleEntriesRecursive(dir) {
    let entries = [];
    fs.readdirSync(dir).forEach((file) => {
      const path = `${dir}/${file}`;
      if (fs.statSync(path).isDirectory()) {
        if (file === "node_modules") return;
        entries = entries.concat(getModuleEntriesRecursive(path));
      } else if (file.endsWith(".js")) {
        entries.push(path);
      }
    });
    return entries;
  }
  return getModuleEntriesRecursive(".");
}

// build with esbuild
try {
  esbuild
    .build({
      entryPoints: getModuleEntries(),
      bundle: false,
      minify: false,
      outdir: ".zeabur/output/functions/index.func",
      platform: "node",
      target: "node20",
      plugins: [
        {
          name: "make-all-packages-external",
          setup(build) {
            let filter = /^[^./]|^\.[^./]|^\.\.[^/]/; // Must not start with "/" or "./" or "../"
            build.onResolve({ filter }, (args) => ({
              path: args.path,
              external: true,
            }));
          },
        },
      ],
    })
    .then((res) => {
      if (res.errors.length > 0) {
        console.error(res.errors);
        process.exit(1);
      }
      console.info(
        "Successfully built app.js into .zeabur/output/functions/index.func"
      );
      fs.copyFileSync(
        ".zeabur/output/functions/index.func/app.js",
        ".zeabur/output/functions/index.func/index.mjs"
      );
      fs.rmSync(".zeabur/output/functions/index.func/app.js");
    });
} catch (error) {
  console.error(error);
}

// copy node_modules into function output directory
console.info(
  "Copying node_modules into .zeabur/output/functions/index.func/node_modules"
);
fs.cpSync("node_modules", ".zeabur/output/functions/index.func/node_modules", {
  recursive: true,
  verbatimSymlinks: true,
});

// copy package.json into function output directory
console.info("Copying package.json into .zeabur/output/functions/index.func");
fs.cpSync("package.json", ".zeabur/output/functions/index.func/package.json");

// copy dynamic-required files into function output directory, so they can be required during runtime
dynamicRequiredDirs.forEach((dir) => {
  copyIfDirExists(dir, `.zeabur/output/functions/index.func/${dir}`);
});

// copy static files into function output directory, so they can be served by the web server directly
staticFileDirs.forEach((dir) => {
  copyIfDirExists(dir, `.zeabur/output/static`);
});

function copyIfDirExists(src, dest) {
  if (!fs.existsSync(src)) {
    console.warn("${src} does not exist");
    return;
  }
  if (!fs.statSync(src).isDirectory()) {
    console.warn(`${src} is not a directory`);
    return;
  }
  console.info(`Copying ${src} to ${dest}`);
  fs.cp(src, dest, { recursive: true }, (err) => {
    if (err) throw err;
  });
}
