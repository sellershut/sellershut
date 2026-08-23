import { error, fail, redirect } from '@sveltejs/kit';
import { z } from 'zod';
import { BACKEND_URL } from '$env/static/private';
import { profileSchema } from '$lib/types/schemas/profile';

import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = ({ locals, cookies }) => {
  const onboardingToken = cookies.get('auth_onboarding');

  if (locals.user || !onboardingToken) {
    redirect(303, '/');
  }
};

type OnboardingResponse = {
  sessionToken: string;
};

const avatarSchema = z
  .instanceof(File)
  .refine((file) => file.size <= 5 * 1024 * 1024, 'Image must be smaller than 5 MB')
  .refine((file) => ['image/jpeg', 'image/png', 'image/webp'].includes(file.type), 'Image must be a JPG, PNG, or WebP');

export type Profile = z.infer<typeof profileSchema>;

export const actions = {
  default: async ({ request, cookies, fetch }) => {
    const onboardingToken = cookies.get('auth_onboarding');

    if (!onboardingToken) {
      error(401, 'No pending onboarding');
    }

    const form = await request.formData();

    const username = form.get('username')?.toString() ?? '';
    const displayName = form.get('display_name')?.toString() ?? '';
    const description = form.get('description')?.toString() ?? '';

    const picture = form.get('picture');
    const result = profileSchema.safeParse({
      username,
      displayName: displayName || undefined,
      description: description || undefined,
    });

    let avatarResult: z.ZodSafeParseResult<File> | undefined;

    if (picture instanceof File && picture.size > 0) {
      avatarResult = avatarSchema.safeParse(picture);
    }

    if (!result.success || avatarResult?.success === false) {
      const errors: Record<string, string[]> = {};
      if (!result.success) {
        for (const issue of result.error.issues) {
          const field = issue.path[0];

          if (typeof field !== 'string') {
            continue;
          }

          errors[field] ??= [];
          errors[field].push(issue.message);
        }
      }

      if (avatarResult?.success === false) {
        errors.picture = avatarResult.error.issues.map((issue) => issue.message);
      }

      return fail(400, {
        errors,
        values: {
          username,
          displayName: displayName,
          description,
        },
      });
    }

    const {
      username: validatedUsername,
      displayName: validatedDisplayName,
      description: validatedDescription,
    } = result.data;

    const _profilePicture = picture instanceof File && picture.size > 0 ? picture : null;

    const response = await fetch(`${BACKEND_URL}/auth/onboard`, {
      method: 'POST',
      headers: {
        'content-type': 'application/json',
      },
      body: JSON.stringify({
        onboardingToken,
        username: validatedUsername,
        displayName: validatedDisplayName,
        description: validatedDescription,
      }),
    });

    if (!response.ok) {
      error(response.status, 'Onboarding failed');
    }

    const { sessionToken } = (await response.json()) as OnboardingResponse;

    cookies.delete('auth_onboarding', {
      path: '/',
    });

    cookies.set('auth_session', sessionToken, {
      path: '/',
      httpOnly: true,
      sameSite: 'lax',
      maxAge: 60 * 60 * 24 * 30,
    });

    redirect(303, '/');
  },
} satisfies Actions;
