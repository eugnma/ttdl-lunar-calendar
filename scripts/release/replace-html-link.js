"use strict";

const fs = require("fs");
const process = require("process");

function main() {
  const myArgs = process.argv.slice(2);
  const fromToDictionary = {};
  myArgs.forEach((arg) => {
    const array = arg.split("->");
    const oldUrl = array[0].trim();
    const newUrl = array[1].trim();
    fromToDictionary[oldUrl] = newUrl;
  });
  readAllText(process.stdin, (data) => {
    const output = replaceHtmlLink(data, fromToDictionary);
    process.stdout.write(output);
  });
}

function replaceHtmlLink(html, fromToDictionary) {
  const reLinkStartTag = /<a [^>]*?>/gi;
  const reHrefAttr = / href=(['"])(.*?)\1/i;
  return html.replace(reLinkStartTag, (linkStartTag) => {
    const match = linkStartTag.match(reHrefAttr);
    const url = match ? match[2] : null;
    return Object.prototype.hasOwnProperty.call(fromToDictionary, url)
      ? linkStartTag.replace(reHrefAttr, ` href=$1${fromToDictionary[url]}$1`)
      : linkStartTag;
  });
}

function readAllText(readableStream, callback) {
  let data = "";
  readableStream.on("data", (chunk) => (data += chunk));
  readableStream.on("end", () => callback(data));
}

main();
