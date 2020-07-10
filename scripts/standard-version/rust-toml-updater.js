"use strict";

const { execFileSync } = require("child_process");

module.exports.readVersion = function (_contents) {
  const version = execFileSync("toml", ["get", "Cargo.toml", "package.version"])
    .toString()
    .trim();
  const normalized_version = version.replace(/(^"|"$)/g, "");
  return normalized_version;
};

module.exports.writeVersion = function (_contents, version) {
  return execFileSync("toml", [
    "set",
    "Cargo.toml",
    "package.version",
    version,
  ]);
};
