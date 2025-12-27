/**
 * Cloudflare Pages Function - Health Proxy
 */

const BACKEND_URL = "https://unacceptable-dennie-axur-tool-8f97679f.koyeb.app";

export async function onRequest(context) {
    const { request } = context;
    const url = new URL(request.url);
    const targetUrl = `${BACKEND_URL}/health${url.search}`;

    const proxyRequest = new Request(targetUrl, {
        method: request.method,
        headers: request.headers,
    });

    return await fetch(proxyRequest);
}
