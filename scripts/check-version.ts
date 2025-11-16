import { parse as parseToml } from '@iarna/toml';
import { execSync } from 'child_process';
import { readdirSync, readFileSync, statSync, writeFileSync } from 'fs';
import { join } from 'path';
import type { PackageJson } from 'type-fest';

interface Result {
  origin: string;
  value: string;
}

function getJsPackagesVersions(dir: string, results: Result[] = []): Result[] {
  const items = readdirSync(dir);

  for (const item of items) {
    const fullPath = join(dir, item);
    const stat = statSync(fullPath);

    if (stat.isDirectory()) {
      if (item !== 'node_modules' && item !== 'target') {
        getJsPackagesVersions(fullPath, results);
      }
    } else if (item === 'package.json') {
      const packageJson: PackageJson = JSON.parse(readFileSync(fullPath, 'utf-8'));

      if (!packageJson.private && packageJson.version) {
        results.push({ origin: fullPath, value: packageJson.version });
      }
    }
  }

  return results;
}

function getRustCratesVersions(dir: string, results: Result[] = []): Result[] {
  const items = readdirSync(dir);

  for (const item of items) {
    const fullPath = join(dir, item);
    const stat = statSync(fullPath);

    if (stat.isDirectory()) {
      if (item !== 'node_modules' && item !== 'target') {
        getRustCratesVersions(fullPath, results);
      }
    } else if (item === 'Cargo.toml') {
      type CargoPackage = { version: {} | string };
      type CargoToml = { package: CargoPackage; workspace: { package: CargoPackage } };

      const cargoToml = parseToml(readFileSync(fullPath, 'utf-8')) as unknown as CargoToml;

      if (cargoToml.package && typeof cargoToml.package.version === 'string') {
        results.push({ origin: fullPath, value: cargoToml.package.version });
      } else if (
        cargoToml.workspace && cargoToml.workspace.package && typeof cargoToml.workspace.package.version === 'string'
      ) {
        results.push({ origin: fullPath, value: cargoToml.workspace.package.version });
      }
    }
  }

  return results;
}

function main() {
  const rootDir = join(import.meta.dirname, '../');
  const versions: Result[] = [...getJsPackagesVersions(rootDir), ...getRustCratesVersions(rootDir)];

  if (versions.length > 0) {
    let tag: string = 'unknown';
    try {
      tag = execSync('git describe --tags --abbrev=0', { encoding: 'utf8' }).trim();
    } catch (e) {
      console.warn(`No prev tags gotten, use the default instead. ${e}`);
    }

    const flatVersions = versions.map(e => e.value);
    if (flatVersions.every(e => e === flatVersions[0])) {
      const newVersion = `v${flatVersions[0]}`;
      if (tag !== newVersion) {
        console.log('Found version changed');
        console.log(`Old version: ${tag}`);
        console.log(`New version: ${newVersion}`);
        writeFileSync(join(rootDir, 'VERSION_INFO'), newVersion, 'utf-8');
      }
    } else {
      console.warn(`The versions in packageJson and CargoToml is not the same, fix needed`);
      console.dir(versions, { depth: null, colors: true });
    }
  }
  process.exit(0);
}

main();
