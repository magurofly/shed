function *main() {
  // (yield) で 1 行読み込み
}

const iter = main();
iter.next();
require("readline").createInterface({input: process.stdin}).on("line", l => iter.next(l));
