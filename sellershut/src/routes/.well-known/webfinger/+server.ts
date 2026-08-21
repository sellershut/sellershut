import { BACKEND_URL } from '$env/static/private';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ url, fetch }) => {
  const resource = url.searchParams.get('resource');

  if (!resource) {
    return new Response('Missing resource', { status: 400 });
  }

  const { body, status, headers } = await fetch(
    `${BACKEND_URL}/.well-known/webfinger?resource=${encodeURIComponent(resource)}`,
  );

  return new Response(body, {
    status,
    headers,
  });
};
