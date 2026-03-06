import { concurrentIf, HAS_AI, TEST_PRODUCTION } from "../lib";
import {
  scrape,
  scrapeRaw,
  scrapeWithFailure,
  scrapeTimeout,
  idmux,
  Identity,
} from "./lib";

let identity: Identity;

beforeAll(async () => {
  identity = await idmux({
    name: "scrape-query",
    concurrency: 100,
    credits: 1000000,
  });
}, 10000 + scrapeTimeout);

describe("Query parameter", () => {
  concurrentIf(TEST_PRODUCTION || HAS_AI)(
    "returns a non-empty answer for a valid query",
    async () => {
      const response = await scrape(
        {
          url: "https://firecrawl.dev",
          query: "What is Firecrawl?",
        },
        identity,
      );

      expect(response.answer).toBeDefined();
      expect(typeof response.answer).toBe("string");
      expect(response.answer!.length).toBeGreaterThan(0);
    },
    scrapeTimeout,
  );

  concurrentIf(TEST_PRODUCTION || HAS_AI)(
    "returns both answer and markdown when formats include markdown",
    async () => {
      const response = await scrape(
        {
          url: "https://firecrawl.dev",
          formats: ["markdown"],
          query: "What is Firecrawl?",
        },
        identity,
      );

      expect(response.answer).toBeDefined();
      expect(typeof response.answer).toBe("string");
      expect(response.answer!.length).toBeGreaterThan(0);
      expect(response.markdown).toBeDefined();
      expect(typeof response.markdown).toBe("string");
    },
    scrapeTimeout,
  );

  concurrentIf(TEST_PRODUCTION || HAS_AI)(
    "does not include answer field when no query param is provided",
    async () => {
      const response = await scrape(
        {
          url: "https://firecrawl.dev",
          formats: ["markdown"],
        },
        identity,
      );

      expect(response.answer).toBeUndefined();
    },
    scrapeTimeout,
  );

  it(
    "rejects query over 1000 characters",
    async () => {
      const longQuery = "a".repeat(1001);
      const response = await scrapeWithFailure(
        {
          url: "https://firecrawl.dev",
          query: longQuery,
        } as any,
        identity,
      );

      expect(response.success).toBe(false);
      expect(response.error).toBeDefined();
    },
    scrapeTimeout,
  );
});
