const path = require("path");
const fs = require("fs");
const { execSync } = require("child_process");

const packet = path.join(__dirname, "..", "target", "release", "cli");
const test262Dir = path.join(__dirname, "..", "github", "test262");

const debugPass = false;
const debugPassExplicit = false;
const debugFail = false;

function findFiles() {
    let pass = fs
        .readdirSync(test262Dir + "/pass")
        .map((file) => `${test262Dir}/pass/${file}`);
    let passExplicit = fs
        .readdirSync(test262Dir + "/pass-explicit")
        .map((file) => `${test262Dir}/pass-explicit/${file}`);
    let fail = fs
        .readdirSync(test262Dir + "/fail")
        .map((file) => `${test262Dir}/fail/${file}`);

    return { pass, passExplicit, fail };
}

function main() {
    const files = findFiles();
    let shouldHavePassed = 0;
    let shouldHavePassedExplicit = 0;
    let shouldHaveFailed = 0;

    console.log("> Running pass files");
    for (file of files.pass) {
        try {
            if (debugPass) {
                console.log(`\n----- ${file} -----`);
            }
            execSync(`${packet} ${file}`, {
                stdio: debugPass ? "inherit" : "ignore",
            });
        } catch {
            shouldHavePassed += 1;
        }
    }

    console.log("> Running pass explicit files");
    for (file of files.passExplicit) {
        try {
            if (debugPassExplicit) {
                console.log(`\n----- ${file} -----`);
            }
            execSync(`${packet} ${file}`, {
                stdio: debugPassExplicit ? "inherit" : "ignore",
            });
        } catch {
            shouldHavePassedExplicit += 1;
        }
    }

    console.log("> Running fail files");
    for (file of files.fail) {
        try {
            if (debugFail) {
                console.log(`\n----- ${file} -----`);
            }
            execSync(`${packet} ${file}`, {
                stdio: debugFail ? "inherit" : "ignore",
            });
            shouldHaveFailed += 1;
        } catch {}
    }

    console.log("\n--- Summary ---");
    console.log(`[Pass] ${shouldHavePassed} / ${files.pass.length}`);
    console.log(
        `[Pass explicit] ${shouldHavePassedExplicit} / ${files.passExplicit.length}`
    );
    console.log(`[Fail] ${shouldHaveFailed} / ${files.fail.length}`);
}

main();
