import * as libhttp from "node:http";
import * as libpg from "pg";

/**
 * ```
 * $ node --version
 * v22.19.0
 *
 * $ tsc --version
 * Version 5.9.2
 * ```
 *
 * ```
 * $ node main.mts
 * ```
 */
async function main() {
  const client: libpg.Client = new libpg.Client(
    "postgres://postgres:postgres@127.0.0.1:5432/postgres?connect_timeout=1",
  );
  await client.connect();
  info("Connected to database");

  const server: libhttp.Server = libhttp.createServer(post_one);
  server.listen(
    8080,
    "127.0.0.1",
    () => info("Listening: %s", JSON.stringify(server.address())),
  );
}
main();

function info(fmt: string, ...args: unknown[]) {
  console.log("[%s] INFO - " + fmt, new Date().toISOString(), ...args);
}

const ROUTE_BASE_PATH_POST_ONE = "/api/books/v1/genre/" as const;

function router(inbound: libhttp.IncomingMessage, outbound: libhttp.ServerResponse) {
  if (inbound.method === "POST" && inbound.url?.startsWith(ROUTE_BASE_PATH_POST_ONE)) {
    return post_one(inbound, outbound);
  } else {
    outbound.statusCode = 404;
    outbound.end();
    return;
  }
}

async function post_one(inbound: libhttp.IncomingMessage, outbound: libhttp.ServerResponse) {
  /*
   * Extract the single expected path parameter "genre".
   */
  const inbound_url_path: string = inbound.url as FuckTypeScript;
  const url_parsed: URL = new URL(inbound_url_path, "http://never.internal");
  const url_path: string = url_parsed.pathname.substring(ROUTE_BASE_PATH_POST_ONE.length);
  const path_params_raw: string[] = url_path.split("/");
  const path_params_decoded: string[] = path_params_raw.map((n) => decodeURIComponent(n));
  if (path_params_decoded.length !== 1) {
    outbound.statusCode = 400;
    outbound.end();
    return;
  }
  const [genre_raw]: [string] = path_params_decoded as [string];

  /*
   * Validate the path parameter "genre".
   */
  let genre: "history" | "horror" | "scifi" | "scitech" | "other";
  switch (genre_raw) {
    case "history":
    case "horror":
    case "other":
    case "scifi":
    case "scitech": {
      genre = genre_raw;
      break;
    }
    default: {
      outbound.statusCode = 400;
      outbound.end();
      return;
    }
  }

  /*
   * Check headers.
   */
  const mime_type = inbound.headers["content-type"];
  if (mime_type !== "application/json") {
    outbound.statusCode = 400;
    outbound.end();
    return;
  }

  /*
   * Capture request payload.
   */
  const request_payload: Buffer = await new Promise<Buffer>((resolve) => {
    const buf: Buffer[] = [];
    inbound.on("data", (b: Buffer) => buf.push(b));
    inbound.on("end", () => resolve(Buffer.concat(buf)));
  });
  if (!request_payload.byteLength) {
    outbound.statusCode = 400;
    outbound.end();
    return;
  }
  const request_payload_decoded: string = request_payload.toString();

  /*
   * Deserialize JSON payload.
   */
  let json: unknown;
  try {
    json = JSON.parse(request_payload_decoded);
  } catch (_) {
    outbound.statusCode = 400;
    outbound.end();
    return;
  }

  /*
   * Validate JSON payload.
   */
  let json_validated: { title: string, page_count: number };
  if (
    typeof json !== "object"
    || json === null
    || !("title" in json)
    || !("page_count" in json)
    || !Number.isInteger(json.page_count)
    || typeof json.page_count !== "number"
    || json.page_count < 0
    || typeof json.title !== "string"
  ) {
    outbound.statusCode = 422;
    outbound.end();
    return;
  } else {
    json_validated = json as FuckTypeScript;
  }
  if (Object.entries(json_validated).length !== 2) {
    outbound.statusCode = 422;
    outbound.end();
    return;
  }

  /*
   * Request is now considered OK. Here, we could do the actual INSERT into the
   * connected PostgreSQL instance... Omitting for brevity!
   */
  info("INSERT: %s", JSON.stringify(json_validated, null, 2));

  outbound.statusCode = 204;
  outbound.end();
  return;
}

type FuckTypeScript = any;
