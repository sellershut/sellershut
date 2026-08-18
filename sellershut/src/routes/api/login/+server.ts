import { BACKEND_URL } from '$env/static/private';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ url, fetch }) => {
  const provider = url.searchParams.get('provider');

  if (!provider) {
    return new Response('Missing provider', { status: 400 });
  }

  const backendUrl = new URL('/auth/login', BACKEND_URL);
  backendUrl.searchParams.set('provider', provider);

  return Response.redirect(backendUrl, 302);
  // const response = await fetch(backendUrl, {
  //   credentials: 'include',
  // });
  //
  // if (!response.ok) {
  //   return new Response('Backend request failed', {
  //     status: response.status,
  //   });
  // }
  //
  // const { authorisation_url } = await response.json();
  //
  // return Response.redirect(authorisation_url, 302);
};
