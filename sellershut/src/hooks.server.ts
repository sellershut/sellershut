import { BACKEND_URL } from '$env/static/private';

export const handle: Handle = async ({ event, resolve }) => {
  const session = event.cookies.get('auth_session');

  event.locals.user = null;

  if (session) {
    const response = await event.fetch(`${BACKEND_URL}/internal/auth/me`, {
      headers: {
        authorization: `Bearer ${session}`,
      },
    });

    if (response.ok) {
      event.locals.user = await response.json();
    } else if (response.status === 401) {
      event.cookies.delete('auth_session', {
        path: '/',
      });
    }
  }

  return resolve(event);
};
