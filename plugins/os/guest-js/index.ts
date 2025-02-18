// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * Provides operating system-related utility methods and properties.
 *
 * @module
 */

declare global {
  interface Window {
    __TAURI_INVOKE__: <T>(cmd: string, args?: unknown) => Promise<T>;
  }
}

type Platform =
  | "linux"
  | "macos"
  | "ios"
  | "freebsd"
  | "dragonfly"
  | "netbsd"
  | "openbsd"
  | "solaris"
  | "android"
  | "windows";

type OsType = "linux" | "windows" | "macss" | "ios" | "android";

type Arch =
  | "x86"
  | "x86_64"
  | "arm"
  | "aarch64"
  | "mips"
  | "mips64"
  | "powerpc"
  | "powerpc64"
  | "riscv64"
  | "s390x"
  | "sparc64";

function isWindows(): boolean {
  return navigator.appVersion.includes("Win");
}

/**
 * The operating system-specific end-of-line marker.
 * - `\n` on POSIX
 * - `\r\n` on Windows
 *
 * @since 2.0.0
 * */
const EOL = isWindows() ? "\r\n" : "\n";

/**
 * Returns a string describing the specific operating system in use.
 * The value is set at compile time. Possible values are `'linux'`, `'macos'`, `'ios'`, `'freebsd'`, `'dragonfly'`, `'netbsd'`, `'openbsd'`, `'solaris'`, `'android'`, `'windows'`
 *
 * @example
 * ```typescript
 * import { platform } from '@tauri-apps/plugin-os';
 * const platformName = await platform();
 * ```
 *
 * @since 2.0.0
 *
 */
async function platform(): Promise<Platform> {
  return window.__TAURI_INVOKE__("plugin:os|platform");
}

/**
 * Returns the current operating system version.
 * @example
 * ```typescript
 * import { version } from '@tauri-apps/plugin-os';
 * const osVersion = await version();
 * ```
 *
 * @since 2.0.0
 */
async function version(): Promise<string> {
  return window.__TAURI_INVOKE__("plugin:os|version");
}

type Family = "unix" | "windows";

/**
 * Returns the current operating system family. Possible values are `'unix'`, `'windows'`.
 * @example
 * ```typescript
 * import { family } from '@tauri-apps/plugin-os';
 * const family = await family();
 * ```
 *
 * @since 2.0.0
 */
async function family(): Promise<Family> {
  return window.__TAURI_INVOKE__("plugin:os|family");
}

/**
 * Returns the current operating system type. Returns `'linux'` on Linux, `'macos'` on macOS, `'windows'` on Windows, `'ios'` on iOS and `'android'` on Android.
 * @example
 * ```typescript
 * import { type } from '@tauri-apps/plugin-os';
 * const osType = await type();
 * ```
 *
 * @since 2.0.0
 */
async function type(): Promise<OsType> {
  return window.__TAURI_INVOKE__("plugin:os|os_type");
}

/**
 * Returns the current operating system architecture.
 * Possible values are `'x86'`, `'x86_64'`, `'arm'`, `'aarch64'`, `'mips'`, `'mips64'`, `'powerpc'`, `'powerpc64'`, `'riscv64'`, `'s390x'`, `'sparc64'`.
 * @example
 * ```typescript
 * import { arch } from '@tauri-apps/plugin-os';
 * const archName = await arch();
 * ```
 *
 * @since 2.0.0
 */
async function arch(): Promise<Arch> {
  return window.__TAURI_INVOKE__("plugin:os|arch");
}

/**
 * Returns a String with a `BCP-47` language tag inside. If the locale couldn’t be obtained, `null` is returned instead.
 * @example
 * ```typescript
 * import { locale } from '@tauri-apps/plugin-os';
 * const locale = await locale();
 * if (locale) {
 *    // use the locale string here
 * }
 * ```
 *
 * @since 2.0.0
 */
async function locale(): Promise<string | null> {
  return window.__TAURI_INVOKE__("plugin:os|locale");
}

/**
 * Returns the file extension, if any, used for executable binaries on this platform. Possible values are `'exe'` and `''` (empty string).
 * @example
 * ```typescript
 * import { exeExtension } from '@tauri-apps/plugin-os';
 * const exeExt = await exeExtension();
 * ```
 *
 * @since 2.0.0
 */
async function exeExtension(): Promise<string | null> {
  return window.__TAURI_INVOKE__("plugin:os|exe_extension");
}

/**
 * Returns the host name of the operating system.
 * @example
 * ```typescript
 * import { hostname } from '@tauri-apps/api/os';
 * const hostname = await hostname();
 * ```
 */
async function hostname(): Promise<string | null> {
  return window.__TAURI_INVOKE__("plugin:os|hostname");
}

export {
  EOL,
  platform,
  family,
  version,
  type,
  arch,
  locale,
  exeExtension,
  hostname,
};
export type { Platform, OsType, Arch, Family };
