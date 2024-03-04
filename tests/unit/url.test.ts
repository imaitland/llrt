//url.test.mjs
import { echo, parse } from "url";

describe("URL", () => {
  it("should echo a url", () => {
    const ec = echo("hello");
    assert.strictEqual(ec, "hello");
  });
  it("should parse a url", () => {
    const ec = parse("https://www.example.com");
    assert.equal(ec.hostname, "www.example.com");
  });
});
