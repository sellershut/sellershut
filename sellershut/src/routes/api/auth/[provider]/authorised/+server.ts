import { error, redirect } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';
import type { RequestHandler } from './$types';

type Session =
  | {
      kind: 'onboardingRequired';
      onboardingToken: string;
    }
  | {
      kind: 'authenticated';
      sessionToken: string;
    };

export const GET: RequestHandler = async ({ cookies, url, fetch, params }) => {
  const provider = params.provider;
  const cookieName = `oauth_state_${provider}`;

  const providerError = url.searchParams.get('error');
  const expectedState = cookies.get(cookieName);

  const code = url.searchParams.get('code');
  const state = url.searchParams.get('state');

  if (providerError) {
    cookies.delete(`oauth_state_${provider}`, {
      path: '/',
    });

    redirect(303, '/login?error=oauth');
  }

  if (!code || !state || !expectedState || state !== expectedState) {
    error(400, 'Invalid OAuth state');
  }

  cookies.delete(`oauth_state_${provider}`, {
    path: '/',
  });

  const response = await fetch(`${BACKEND_URL}/auth/${provider}/authorised`, {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify({
      code,
      state,
    }),
  });

  if (!response.ok) {
    error(response.status, 'OAuth authentication failed');
  }

  const result = (await response.json()) as Session;

  switch (result.kind) {
    case 'authenticated':
      cookies.set('auth_session', result.sessionToken, {
        path: '/',
        httpOnly: true,
        sameSite: 'lax',
        maxAge: 60 * 60 * 24 * 30,
      });

      redirect(303, '/');
      break;

    case 'onboardingRequired':
      cookies.set('auth_onboarding', result.onboardingToken, {
        path: '/',
        httpOnly: true,
        sameSite: 'lax',
        maxAge: 15 * 60,
      });

      redirect(303, '/setup/profile');
      break;

    default:
      error(500, 'Unexpected OAuth response');
  }
};
