import { redirect } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';
import type { RequestHandler } from './$types';

type LoginResponse = {
  authorisationUrl: string;
  state: string;
};

export const GET: RequestHandler = async ({ cookies, url, fetch }) => {
  const provider = url.searchParams.get('provider');

  if (!provider) {
    return new Response('Missing provider', { status: 400 });
  }

  const backendUrl = new URL('/auth/login', BACKEND_URL);
  backendUrl.searchParams.set('provider', provider);

  const response = await fetch(backendUrl, {
    method: 'POST',
  });

  if (!response.ok) {
    return new Response('Backend request failed', {
      status: response.status,
    });
  }

  const { authorisationUrl, state } = (await response.json()) as LoginResponse;
  const cookieName = `oauth_state_${provider}`;

  cookies.set(cookieName, state, {
    httpOnly: true,
    sameSite: 'lax',
    path: '/',
    maxAge: 10 * 60,
  });

  return redirect(303, authorisationUrl);
};
