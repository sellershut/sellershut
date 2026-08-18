import { fail, redirect } from '@sveltejs/kit';
import type { Actions, PageServerLoad } from './$types';

const USERNAME_REGEX = /^[a-z0-9_]{3,30}$/;

export const load: PageServerLoad = async ({ locals }) => {
  // // Your auth middleware should already have populated locals.user.
  // if (!locals.user) {
  // 	redirect(303, '/login');
  // }
  //
  // // If the user already has a username, there is nothing to set up.
  // if (locals.user.username) {
  // 	redirect(303, '/');
  // }
};

export const actions: Actions = {
  default: async ({ request, locals }) => {
    if (!locals.user) {
      return fail(401, {
        error: 'You must be signed in to continue.',
      });
    }

    const formData = await request.formData();

    const username = String(formData.get('username') ?? '')
      .trim()
      .toLowerCase();

    if (!USERNAME_REGEX.test(username)) {
      return fail(400, {
        username,
        error: 'Username must be 3–30 characters and contain only lowercase letters, numbers, and underscores.',
      });
    }

    /*
     * Check username availability.
     *
     * Replace this with your UsersService / database call.
     */
    const existingUser = await locals.users.getByUsername(username);

    if (existingUser) {
      return fail(409, {
        username,
        error: 'That username is already taken.',
      });
    }

    /*
     * Update the authenticated user.
     *
     * Ideally your database should also have a UNIQUE constraint
     * on username so this remains race-safe.
     */
    await locals.users.setUsername(locals.user.id, username);

    redirect(303, '/');
  },
};
