/**
 * Gets a function from a CheerpJ 3 library object.
 * Intended to be used in Rust - JS users can simply do this:
 * ```ts
 * lib.org.lwjgl.DefaultSysImplementation.getJNIVersion() // returns a Java function
 * ```
 * @param {string} method The full method name, e.g. `org.lwjgl.DefaultSysImplementation.getJNIVersion`
 * @param {Object} lib The CheerpJ 3 library object.
 * @returns {function | undefined}
 */
export function getMethodFromCJLib(method, lib) {
  const parts = method.split(".");
  let currentLib = lib;
  for (const part of parts) {
    currentLib = currentLib[part];
    if (!currentLib) {
      return undefined;
    }
    console.log(currentLib);
  }
  return currentLib;
}

export function consoleTable(obj) {
  console.table(obj);
}
