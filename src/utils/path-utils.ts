/**
 * Expands ~ to home directory in paths
 * @param path Path to expand
 * @returns Expanded path
 */
export function expandPath(path: string): string {
  if (path.startsWith("~/") || path.startsWith("~\\")) {
    const home = Deno.env.get("HOME") || Deno.env.get("USERPROFILE");
    if (!home) {
      throw new Error("Could not determine home directory");
    }
    return path.replace(/^~/, home);
  }
  return path;
}

/**
 * Normalizes path separators for the current platform
 * @param path Path to normalize
 * @returns Normalized path
 */
export function normalizePath(path: string): string {
  if (Deno.build.os === "windows") {
    return path.replace(/\//g, "\\");
  }
  return path.replace(/\\/g, "/");
}

/**
 * Gets the directory name from a path
 * @param path File path
 * @returns Directory path
 */
export function dirname(path: string): string {
  const normalized = normalizePath(path);
  const separator = Deno.build.os === "windows" ? "\\" : "/";
  const lastSep = normalized.lastIndexOf(separator);
  if (lastSep === -1) return ".";
  return normalized.slice(0, lastSep) || separator;
}
