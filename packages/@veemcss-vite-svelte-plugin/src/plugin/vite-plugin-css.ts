import { skipSvelteClass } from "./utils/skip.js";
import {sum} from "@veemcss/parser"
import { Scanner } from "@veemcss/parser";

const REGEX = {
  // group 1 : list class
  CLASS: /(?:class:\s*"|class=")([^"]+)"/g,
  // Group 1 : Breakpoint
  // Group 2 : Class Name
  // Group 3 : Size if apply
  CLASS_INFO: /(?:(sm|md|lg|xl):)?([a-z-]+)?-(?:\[(.*?)\])?/g,
};

const BREAKPOINTS = {
  sm: "(min-width: 640px)",
  md: "(min-width: 768px)",
  lg: "(min-width: 1024px)",
  xl: "(min-width: 1280px)",
};

interface ClassInfo {
  breakpoint : string,
  className : string,
  size : string,
}

class VeemCSSPluginCore {
  definedClasses: Array<string>;
  currentClasses: Map<string, ClassInfo>;
  lastProcessedId: string;
  hasSvelteFile: boolean;

  constructor() {
    //console.log('From native', sum(40, 2))
    this.definedClasses = [];
    this.currentClasses = new Map();
    this.lastProcessedId = "";
    this.hasSvelteFile = false;
  }

  /**
   * Process a single class string and extract its information
   * @param {string} classStr - The class string to process
   * @returns {void}
   */
  processClass(classStr) {
    if (
      skipSvelteClass(classStr) ||
      this.currentClasses.has(classStr) ||
      this.definedClasses.includes(classStr)
    ) {
      return;
    }

    const classInfoResults = [...classStr.matchAll(REGEX.CLASS_INFO)];
    classInfoResults.forEach(([, breakpoint, className, size]) => {
      this.currentClasses.set(classStr, {
        breakpoint,
        className,
        size,
      });
      console.warn(`new class fffff : {  ${sum(1,32)} }`);
    });
  }

  /**
   * Find all the class from the code provide
   * @param {string} code - The code to verify class use
   * @returns {string[]}
   */
  parseCSSClasses(code) {
    const classMatches = [...code.matchAll(REGEX.CLASS)];
    return classMatches
      .map(([, classNames]) => classNames.split(" "))
      .flat()
      .filter((cls) => !this.definedClasses.includes(cls));
  }

  /**
   * Handle CSS file processing
   * @returns {void}
   */
  handleCSSFile() {
    if (!this.hasSvelteFile) return;

    this.currentClasses.forEach((_, key) => {
      this.definedClasses.push(key);
    });

    this.currentClasses.clear();
    this.hasSvelteFile = false;
  }

  /**
   * Transform the code with compile class
   ** See Vite docs
   * @param {string} code
   * @param {string} id
   * @returns {void}
   */
  transform(code, id) {
    if (id.includes("node_modules")) return;
    if (id.endsWith(".svelte")) {
      this.lastProcessedId = id;
      this.parseCSSClasses(code).forEach((currClass) => {
        this.processClass(currClass);
      });
      this.hasSvelteFile = true;
    } else if (id.endsWith(".css")) {
      this.handleCSSFile();
    } else {
      this.hasSvelteFile = false;
    }
  }

  /**
   * Create Vite plugin for css library
   * @returns {Object} Vite plugin configuration
   */
  createVitePlugin() {
    return {
      name: "vite-plugin-atomic-css",
      transform: (code, id) => this.transform(code, id),
    };
  }
}

export function VeemCSSPlugin() {
  //let scanner = new Scanner();
  //const plugin = new VeemCSSPluginCore();
  //return plugin.createVitePlugin();
  return {
    name: "vite-plugin-atomic-css",
    transform: (code, id) => {
      if (id.includes("node_modules")) return;
      if (!id.endsWith(".svelte")) return;
      let scanner = new Scanner(code);
      let output = scanner.scan();
      output.forEach((info) => {
        console.log(`${info.breakpoint}, ${info.classeName}, ${info.size}`);
      });

    }
  };
}

export { REGEX };
