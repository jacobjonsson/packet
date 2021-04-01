const path = require("path");
const fs = require("fs");
const { execSync } = require("child_process");

const packet = path.join(__dirname, "..", "target", "release", "cli");
const test262Dir = path.join(__dirname, "..", "github", "test262");

const ignorePassFiles = [
    "fbcd793ec7c82779.js", // HTML Comment
    "1270d541e0fd6af8.js", // HTML Comment
    "e03ae54743348d7d.js", // HTML Comment
    "d3ac25ddc7ba9779.js", // HTML Comment
    "b15ab152f8531a9f.js", // HTML Comment
    "946bee37652a31fa.js", // HTML Comment
    "9f0d8eb6f7ab8180.js", // HTML Comment
    "8ec6a55806087669.js", // HTML Comment
    "5d5b9de6d9b95f3e.js", // HTML Comment
    "5a2a8e992fa4fe37.js", // HTML Comment
    "4f5419fe648c691b.js", // HTML Comment
    "4ae32442eef8a4e0.js", // HTML Comment
    "0b6dfcd5427a43a6.js", // Legacy octal literal
    "f1534392279bddbf.js", // Legacy octal literal
    "d2af344779cc1f26.js", // Legacy octal literal
    "84b2a5d834daee2f.js", // Legacy octal literal
    "c06df922631aeabc.js", // Function declaration after if
    "a4d62a651f69d815.js", // Function declaration after if
    "59ae0289778b80cd.js", // Function declaration after if
    "52aeec7b8da212a2.js", // Function declaration after if
    "3dabeca76119d501.js", // Function declaration after if
];

const ignorePassExplicitFiles = [];

const ignoreFailFiles = [];

function findFiles() {
    let pass = fs
        .readdirSync(test262Dir + "/pass")
        .filter((file) => !ignorePassFiles.includes(file))
        .map((file) => `${test262Dir}/pass/${file}`);

    let passExplicit = fs
        .readdirSync(test262Dir + "/pass-explicit")
        .filter((file) => !ignorePassExplicitFiles.includes(file))
        .map((file) => `${test262Dir}/pass-explicit/${file}`);

    let fail = fs
        .readdirSync(test262Dir + "/fail")
        .filter((file) => !ignoreFailFiles.includes(file))
        .map((file) => `${test262Dir}/fail/${file}`);

    return { pass, passExplicit, fail };
}

function main() {
    const files = findFiles();
    let shouldHavePassed = 0;

    console.log("> Running pass files");
    for (file of files.pass) {
        try {
            execSync(`${packet} ${file}`, {
                stdio: "pipe",
            });
        } catch (err) {
            console.error(`${file} failed to pass`);
            console.log(err.stderr.toString());
            shouldHavePassed += 1;
        }
    }

    console.log("\n--- Summary ---");
    console.log(`[Pass] ${shouldHavePassed} / ${files.pass.length}`);
}

main();
