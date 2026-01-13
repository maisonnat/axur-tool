/**
 * Cloudflare Pages Function - API Proxy
 * 
 * Proxies ALL HTTP methods (GET, POST, PUT, DELETE) to Leapcell backend.
 * This is needed because _redirects only works for GET requests.
 */

const BACKEND_URL = "https://axur-backend-844146909418.us-central1.run.app";

export async function onRequest(context) {
  const { request, params } = context;

  // Debug log
  console.log(`[Proxy] ${request.method} ${request.url}`);

  // Reconstruct the target URL
  const path = params.path ? `/${params.path.join("/")}` : "";
  const url = new URL(request.url);
  const targetUrl = `${BACKEND_URL}/api${path}${url.search}`;

  // Clone the request with the new URL
  const proxyRequest = new Request(targetUrl, {
    method: request.method,
    headers: request.headers,
    body: request.method !== "GET" && request.method !== "HEAD"
      ? await request.arrayBuffer()
      : undefined,
    redirect: "follow",
  });

  // Forward the request to the backend
  const response = await fetch(proxyRequest);

  // Clone response and add CORS headers for the proxy
  const newHeaders = new Headers(response.headers);

  // Preserve Set-Cookie headers from backend
  // (Cloudflare Functions handle this automatically)

  return new Response(response.body, {
    status: response.status,
    statusText: response.statusText,
    headers: newHeaders,
  });
}
