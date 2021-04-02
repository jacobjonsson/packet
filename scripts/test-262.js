const path = require("path");
const fs = require("fs");
const { execSync } = require("child_process");

const packet = path.join(__dirname, "..", "target", "release", "cli");
const test262Dir = path.join(__dirname, "..", "github", "test262");

const ignorePassFiles = [
    // HTML Comments
    "fbcd793ec7c82779.js",
    "1270d541e0fd6af8.js",
    "e03ae54743348d7d.js",
    "d3ac25ddc7ba9779.js",
    "b15ab152f8531a9f.js",
    "946bee37652a31fa.js",
    "9f0d8eb6f7ab8180.js",
    "8ec6a55806087669.js",
    "5d5b9de6d9b95f3e.js",
    "5a2a8e992fa4fe37.js",
    "4f5419fe648c691b.js",
    "4ae32442eef8a4e0.js",
    "c532e126a986c1d4.js",
    "ba00173ff473e7da.js",

    // Legacy octal literal
    "0b6dfcd5427a43a6.js",
    "f1534392279bddbf.js",
    "d2af344779cc1f26.js",
    "84b2a5d834daee2f.js",

    // Function declaration after if
    "c06df922631aeabc.js",
    "a4d62a651f69d815.js",
    "59ae0289778b80cd.js",
    "52aeec7b8da212a2.js",
    "3dabeca76119d501.js",
    "1c1e2a43fe5515b6.js",

    // Non-prohibited usage of let (strict mode)
    "ffaf5b9d3140465b.js",
    "f2e41488e95243a8.js",
    "f0fbbdabdaca2146.js",
    "f0d9a7a2f5d42210.js",
    "ee4e8fa6257d810a.js",
    "df696c501125c86f.js",
    "c8565124aee75c69.js",
    "c442dc81201e2b55.js",
    "b8c98b5cd38f2bd9.js",
    "a1594a4d0c0ee99a.js",
    "9fe1d41db318afba.js",
    "9aa93e1e417ce8e3.js",
    "818ea8eaeef8b3da.js",
    "660f5a175a2d46ac.js",
    "65401ed8dc152370.js",
    "63c92209eb77315a.js",
    "5ecbbdc097bee212.js",
    "56e2ba90e05f5659.js",
    "5654d4106d7025c2.js",
    "2ef5ba0343d739dc.js",
    "14199f22a45c7e30.js",

    // Non-ascii formatted code
    "f7f611e6fdb5b9fc.js",
    "f5b89028dfa29f27.js",
    "eaee2c64dfc46b6a.js",
    "e18f80fa1699a0fd.js",
    "dc3afa2f13259ae0.js",
    "dafb7abe5b9b44f5.js",
    "d09117cbf60bd18f.js",
    "c85fbdb8c97e0534.js",
    "c7e5fba8bf3854cd.js",
    "b644c9f68254c85d.js",
    "afffb6d317e53b92.js",
    "5b146261dda66d63.js",
    "4e997dab642864ca.js",
    "465b79616fdc9794.js",
    "431ecef8c85d4d24.js",
    "0d6e503c739611e2.js",
    "05d5195f0d388d98.js",
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
