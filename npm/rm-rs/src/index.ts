#!/usr/bin/env node

//#region LICENSE
/*! Adapted from https://github.com/orhun/git-cliff
 *
 * The MIT License (MIT)
 * Copyright (c) 2021-2024 Orhun Parmaksız
 * Copyright (c) 2021-2024 git-cliff contributors
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
//#endregion

import { spawnSync } from "child_process";

/**
 * Returns the executable path which is located inside `node_modules`
 * The naming convention is rm-rs-${os}-${arch}
 * If the platform is `win32` or `cygwin`, executable will include a `.exe` extension.
 * @see https://nodejs.org/api/os.html#osarch
 * @see https://nodejs.org/api/os.html#osplatform
 * @example "x/xx/node_modules/rm-rs-darwin-arm64"
 */
function getExePath() {
  const name = "rm-rs";
  const arch = process.arch;
  let os: string = process.platform;
  let extension = "";
  if (["win32", "cygwin"].includes(os)) {
    os = "windows";
    extension = ".exe";
  }

  try {
    // Since the binary will be located inside `node_modules`, we can simply call `require.resolve`
    return require.resolve(`${name}-${os}-${arch}/bin/${name}${extension}`);
  } catch (e) {
    throw new Error(
      `Couldn't find application binary inside node_modules for ${os}-${arch}`
    );
  }
}

function main() {
  const args = process.argv.slice(2);
  console.log(getExePath());
  const processResult = spawnSync(getExePath(), args, { stdio: "inherit" });

  // Returning an error if the status is null, as that means the process was killed by a signal
  process.exit(processResult.status ?? 1);
}

main();
