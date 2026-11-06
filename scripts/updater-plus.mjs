import { copyFile, mkdir, readdir, readFile, writeFile } from 'node:fs/promises'
import path from 'node:path'

const [input, output] = process.argv.slice(2)
const version = process.env.RELEASE_VERSION
const repository = process.env.GITHUB_REPOSITORY
if (!input || !output || !/^\d+\.\d+\.\d+-Plus$/.test(version ?? '')) {
  throw new Error(
    'Usage: RELEASE_VERSION=X.Y.Z-Plus node scripts/updater.mjs <bundles> <output>',
  )
}
if (!/^[\w.-]+\/[\w.-]+$/.test(repository ?? '')) {
  throw new Error('GITHUB_REPOSITORY must be owner/repository')
}

async function filesIn(directory) {
  const files = []
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const file = path.join(directory, entry.name)
    if (entry.isDirectory()) files.push(...(await filesIn(file)))
    else if (entry.isFile()) files.push(file)
  }
  return files
}

const files = await filesIn(input)
const platforms = {}
const packages = []
for (const [extension, targets] of [
  ['.exe', ['windows-x86_64', 'windows-x86_64-nsis']],
  ['.deb', ['linux-x86_64', 'linux-x86_64-deb']],
  ['.rpm', ['linux-x86_64-rpm']],
]) {
  const matches = files.filter((file) => file.endsWith(extension))
  if (matches.length !== 1)
    throw new Error(
      `Expected one ${extension} installer, found ${matches.length}`,
    )
  const file = matches[0]
  const signature = (await readFile(`${file}.sig`, 'utf8')).trim()
  if (!signature) throw new Error(`Empty signature for ${file}`)
  const name = path.basename(file)
  const entry = {
    signature,
    url: `https://github.com/${repository}/releases/download/v${version}/${encodeURIComponent(name)}`,
  }
  for (const target of targets) platforms[target] = entry
  packages.push(file, `${file}.sig`)
}

await mkdir(output, { recursive: true })
for (const file of packages)
  await copyFile(file, path.join(output, path.basename(file)))
await writeFile(
  path.join(output, 'latest.json'),
  JSON.stringify(
    {
      version,
      notes: `Clash Verge Rev ${version} for Windows and Linux AMD64.`,
      pub_date: new Date().toISOString(),
      platforms,
    },
    null,
    2,
  ) + '\n',
)
console.log(`Prepared ${packages.length / 2} signed installers and latest.json`)
