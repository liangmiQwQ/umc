import { parse as parseToml } from '@iarna/toml';
import { execSync } from 'child_process';
import { readdirSync, readFileSync, statSync, writeFileSync } from 'fs';
import { join } from 'path';
import type { PackageJson } from 'type-fest';

function getJsPackagesVersions(dir: string, results: string[] = []): string[] {
  const items = readdirSync(dir);

  for (const item of items) {
    const fullPath = join(dir, item);
    const stat = statSync(fullPath);

    if (stat.isDirectory()) {
      if (item !== 'node_modules') {
        getJsPackagesVersions(fullPath, results);
      }
    } else if (item === 'package.json') {
      const packageJson: PackageJson = JSON.parse(readFileSync(fullPath, 'utf-8'));

      if (!packageJson.private && packageJson.version) {
        results.push(packageJson.version);
      }
    }
  }

  return results;
}

function getRustCratesVersions(dir: string, results: string[] = []): string[] {
  const items = readdirSync(dir);

  for (const item of items) {
    const fullPath = join(dir, item);
    const stat = statSync(fullPath);

    if (stat.isDirectory()) {
      if (item !== 'node_modules') {
        getRustCratesVersions(fullPath, results);
      }
    } else if (item === 'Cargo.toml') {
      type CargoPackage = { version: {} | string };
      type CargoToml = { package: CargoPackage; workspace: { package: CargoPackage } };

      const cargoToml = parseToml(readFileSync(fullPath, 'utf-8')) as unknown as CargoToml;

      if (cargoToml.package && typeof cargoToml.package.version === 'string') {
        results.push(cargoToml.package.version);
      } else if (
        cargoToml.workspace && cargoToml.workspace.package && typeof cargoToml.workspace.package.version === 'string'
      ) {
        results.push(cargoToml.workspace.package.version);
      }
    }
  }

  return results;
}

function main() {
  const rootDir = join(import.meta.dirname, '../');
  const versions = [...getJsPackagesVersions(rootDir), ...getRustCratesVersions(rootDir)];
  if (versions.length > 0) {
    let tag: string = 'unknown';
    try {
      execSync('git describe --tags --abbrev=0', { encoding: 'utf8' }).trim();
    } catch (e) {
      console.warn(`No prev tags gotten, use the default instead. ${e}`);
    }

    if (versions.every(e => e === versions[0]) && tag !== versions[0]) {
      console.log('Found version changed');
      console.log(`Old version: ${tag}`);
      console.log(`New version: ${versions[0]}`);
      writeFileSync(join(rootDir, 'VERSION_INFO'), versions[0], 'utf-8');
    }
  }
  process.exit(0);
}

main();
