export function getMethodFromCJLib(method, lib) {
  const parts = method.split(".");
  let currentLib = lib;
  for (const part of parts) {
    currentLib = currentLib[part];
  }
  return currentLib;
}
