import { error, fail, redirect } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';

const USERNAME_REGEX = /^[a-z0-9_]{3,30}$/;

import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = ({ locals }) => {
  if (locals.user) {
    redirect(303, '/');
  }
};

export const actions = {
  default: async ({ request, cookies, fetch }) => {
    const onboardingToken = cookies.get('auth_onboarding');

    if (!onboardingToken) {
      error(401, 'No pending onboarding');
    }

    const form = await request.formData();
    const username = form.get('username');

    if (typeof username !== 'string') {
      error(400, 'Username required');
    }

    if (!USERNAME_REGEX.test(username)) {
      return fail(400, {
        username,
        error: 'Username must be 3–30 characters and contain only lowercase letters, numbers, and underscores.',
      });
    }

    const response = await fetch(`${BACKEND_URL}/auth/onboard`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify({
        onboarding_token: onboardingToken,
        username,
      }),
    });

    if (!response.ok) {
      console.log(response);
      error(response.status, 'Onboarding failed');
    }

    const result = await response.json();
    console.log(result);

    cookies.delete('auth_onboarding', {
      path: '/',
    });

    cookies.set('auth_session', result.session_token, {
      path: '/',
      httpOnly: true,
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 30,
    });

    redirect(303, '/');
  },
} satisfies Actions;
