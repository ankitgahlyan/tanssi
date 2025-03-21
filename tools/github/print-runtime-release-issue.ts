import yargs from "yargs";

async function main() {
  const argv = yargs(process.argv.slice(2))
    .usage("Usage: npm run ts-node github/generate-gh-issue-runtime-release.ts [args]")
    .version("1.0.0")
    .options({
      from: {
        type: "string",
        describe: "previous runtime version",
        required: true,
      },
      to: {
        type: "string",
        describe: "next runtime version",
        required: true,
      },
      client: {
        type: "string",
        describe: "current client version",
        required: true,
      },
    })
    .demandOption(["from", "to", "client"])
    .help().argv;

  const previousVersion = argv.from;
  const newVersion = argv.to;
  const lastClientVersion = argv.client;

  const commonTemplate =
    `
## Release
- [ ] Check all proxy types.
- [ ] Re-run all extrinsics/hooks benchmarks.
- [ ] Branch from master and create branch \`perm-runtime-${newVersion}\`.
- [ ] Tag \`perm-${newVersion}\` with runtime-${newVersion} and push to github
- [ ] Tag \`perm-runtime-${newVersion}\` with runtime-${newVersion}-templates and push to github
- [ ] Tag \`perm-runtime-${newVersion}\` with runtime-${newVersion}-starlight and push to github
- [ ] NOTE: if this is a hotfix to one of the runtimes, branch from runtime-${previousVersion} version
and create perm-runtime-${newVersion}-templates, perm-runtime-${newVersion}-starlight or perm-runtime-${newVersion}-tanssi
depending on whether the hotfix is for templates, starlight or tanssi-parachain. Then tag accordingly
- [ ] Start the github action Publish Runtime Draft
with runtime-${previousVersion} => runtime-${newVersion}
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
    `-f from=runtime-${previousVersion} -f to=runtime-${newVersion} -f chains=tanssi-only\`
- [ ] Start the github action Publish Runtime Draft
with runtime-${previousVersion} => runtime-${newVersion}
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
    `-f from=runtime-${previousVersion} -f to=runtime-${newVersion} -f chains=templates-only\`
- [ ] Start the github action Publish Runtime Draft
  with runtime-${previousVersion} => runtime-${newVersion}
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
    `-f from=runtime-${previousVersion} -f to=runtime-${newVersion} -f chains=dancelight-only\`
- [ ] Review the generated Draft and clean a bit the messages if needed (keep it draft)
- [ ] Upgrade typescript API: Start the github action "Upgrade typescript API"
- [ ] Upgrade stagenet-dancebox
- [ ] Upgrade stagelight
- [ ] When everything is ok, publish the draft release
  `;

  // Detect if it's a major release or hotfix
  if (newVersion.endsWith("00")) {
    const template =
      `
## Requirements
- [ ] To be manually edited (add pending PRs)

## Pre-Release
- [ ] Check that proxy types are adapted to extrinsics changes (
  read all PR descriptions with B7-runtimenoteworthy)
- [ ] Re-run all extrinsics/hooks benchmarks.

${commonTemplate}

## Post Release
- [ ] Publish the docker runtime image (trigger the github action "Publish Docker runtime tanssi")
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
      `-f from=runtime-${previousVersion} -f to=runtime-${newVersion}\`
- [ ] Publish the docker runtime image (trigger the github action "Publish Docker runtime containers")
  - \`gh workflow run "Publish Runtime Draft" -r 'master' ` +
      `-f from=runtime-${previousVersion}-templates -f to=runtime-${newVersion}-templates\`
- [ ] Create a PR that increment spec version (like #1051) in both containers, tanssi and starlight runtimes
    `;
    console.log(template);
  } else {
    const template = `
## Requirements
- [ ] To be manually edited (add pending PRs)

## Pre-Release
- [ ] Bump spec version to ${newVersion}

${commonTemplate}

## Post Release
- [ ] Publish the docker runtime image (trigger the github action "Publish Docker runtime")
    `;
    console.log(template);
  }
}

main();