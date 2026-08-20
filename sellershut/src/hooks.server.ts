import type { Handle } from '@sveltejs/kit';
import { BACKEND_URL, INSTANCE_DOMAIN, INSTANCE_NAME, INSTANCE_URL } from '$env/static/private';

export const handle: Handle = async ({ event, resolve }) => {
  event.locals.instance = {
    domain: INSTANCE_DOMAIN,
    name: INSTANCE_NAME,
    url: INSTANCE_URL,
  };
  const session = event.cookies.get('auth_session');

  event.locals.user = undefined;

  if (session) {
    const response = await event.fetch(`${BACKEND_URL}/users/me`, {
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
