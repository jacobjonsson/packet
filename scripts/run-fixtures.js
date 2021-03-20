const { execSync } = require("child_process");
const path = require("path");
const fs = require("fs");

const cliPath = path.join(__dirname, "..", "target", "release", "cli");
const fixturesPath = path.join(__dirname, "..", "fixtures");

const fixtures = fs
  .readdirSync(fixturesPath)
  .filter((file) => file.endsWith(".in.js"));

let results = [];

for (fixture of fixtures) {
  try {
    console.log(`Running ${fixture}...`);
    execSync(
      `${cliPath} ./fixtures/${fixture} ./fixtures/${fixture.replace(
        ".in.js",
        ".out.js"
      )}`,
      {
        stdio: "pipe",
      }
    );
    results.push({ fixture, success: true, error: null });
  } catch (error) {
    results.push({ fixture, success: false, error: error.message });
  }
}

let successfulFixtures = results.filter((res) => res.success);
let errorFixtures = results.filter((res) => !res.success);

console.log(`Success count: ${successfulFixtures.length}
Error count: ${errorFixtures.length}
`);

if (errorFixtures.length > 0) {
  console.log("Errors:");
  for ({ fixture, error } of errorFixtures) {
    console.log(`- ${fixture}`);
    console.log(`Error: ${error}`);
  }
}
