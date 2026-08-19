import { redirect } from '@sveltejs/kit';
import { BACKEND_URL } from '$env/static/private';
import type { RequestHandler } from './$types';

export const POST: RequestHandler = async ({ cookies, fetch }) => {
  const session = cookies.get('auth_session');
  if (session) {
    await fetch(`${BACKEND_URL}/auth/logout`, {
      method: 'POST',
    });
  }
  cookies.delete('auth_session', {
    path: '/',
  });
  return redirect(303, '/');
};
