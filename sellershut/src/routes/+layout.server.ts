import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
  return {
    user: locals.user,
    domain: locals.instance.domain,
  };
};
