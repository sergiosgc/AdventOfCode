import { readFileSync } from "fs"
import * as getCallerFile from "get-caller-file"

export const readInput = (test=false) => {
  const file = getCallerFile()
    .split("/")
    .slice(0, -1)
    .concat(test ? "test.input.txt" : "input.txt")
    .join("/")

  return readFileSync(file).toString()
}
