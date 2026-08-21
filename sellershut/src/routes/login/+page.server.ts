import { error, redirect } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';
import { toOauthCookieName } from '$lib/oauth';
import type { PageServerLoad } from './$types';

type LoginResponse = {
  authorisationUrl: string;
  state: string;
};

export const load: PageServerLoad = async ({ locals, url, fetch, cookies }) => {
  if (locals.user) {
    redirect(303, '/');
  }

  const rawProvider = url.searchParams.get('provider');

  if (rawProvider) {
    const { provider, cookieName } = toOauthCookieName(rawProvider);

    const backendUrl = new URL('/auth/login', BACKEND_URL);
    backendUrl.searchParams.set('provider', provider);
    switch (provider) {
      case 'discord':
      case 'google':
        break;
      default:
        error(400, 'Invalid login provider');
    }

    const response = await fetch(backendUrl, {
      method: 'POST',
    });

    if (!response.ok) {
      error(response.status, 'Backend request failed');
    }

    const { authorisationUrl, state } = (await response.json()) as LoginResponse;

    cookies.set(cookieName, state, {
      httpOnly: true,
      sameSite: 'lax',
      path: '/',
      maxAge: 10 * 60,
    });

    return redirect(303, authorisationUrl);
  }
};
