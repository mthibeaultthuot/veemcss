const SKIP_SVELTE_CLASS = /[s-]............./g;

/**
 * Function to check if a given class string matches the Svelte-style class pattern.
 *
 * @param {string} classz - The class name to check.
 * @returns {boolean} - An array of matched class names or null if no match is found.
 *
 * @example
 * skipSvelteClass("s-eRIYyjEl_Oqd"); // Returns ["s-eRIYyjEl_Oqd"]
 * skipSvelteClass("other-class_name"); // Returns null
 */
export function skipSvelteClass(classz) {
  if (classz.includes("radius")) return false;
  return classz.includes("s-");
}
