/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export declare function sum(a: number, b: number): number
export declare class Scanner {
  constructor(code: string)
  scan(): Array<ClasseInfo>
  findClass(classes: Array<string>): Array<ClasseInfo>
}
export declare class ScannerOutput { }
export declare class ClasseInfo {
  breakpoint?: string
  classeName?: string
  size?: string
}