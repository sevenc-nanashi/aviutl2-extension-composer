import { Version } from "./ipc.ts";

const pattern =
  /^(?<major>[0-9]+)\.(?<minor>[0-9]+)(?:\.(?<patch>[0-9]+))?(?:-(?<prerelease>[0-9A-Za-z.-]+))?$/;

export function compareVersions(v1: Version, v2: Version): number {
  if (v1.version_number != null && v2.version_number != null) {
    return Math.sign(v1.version_number - v2.version_number);
  }
  if (v1.version_number !== null) {
    return 1;
  }
  if (v2.version_number !== null) {
    return -1;
  }

  const m1 = v1.version.match(pattern);
  const m2 = v2.version.match(pattern);
  if (!m1 || !m2 || !m1.groups || !m2.groups) {
    throw new Error("Invalid version format");
  }

  const major1 = parseInt(m1.groups.major, 10);
  const major2 = parseInt(m2.groups.major, 10);
  if (major1 !== major2) {
    return Math.sign(major1 - major2);
  }

  const minor1 = parseInt(m1.groups.minor, 10);
  const minor2 = parseInt(m2.groups.minor, 10);
  if (minor1 !== minor2) {
    return Math.sign(minor1 - minor2);
  }

  const patch1 = m1.groups.patch ? parseInt(m1.groups.patch, 10) : 0;
  const patch2 = m2.groups.patch ? parseInt(m2.groups.patch, 10) : 0;
  if (patch1 !== patch2) {
    return Math.sign(patch1 - patch2);
  }

  const pre1 = m1.groups.prerelease;
  const pre2 = m2.groups.prerelease;
  if (pre1 === undefined && pre2 === undefined) {
    return 0;
  }
  if (pre1 === undefined) {
    return 1;
  }
  if (pre2 === undefined) {
    return -1;
  }
  if (pre1 === pre2) {
    return 0;
  }
  return [pre1, pre2].toSorted()[0] === pre1 ? -1 : 1;
}
