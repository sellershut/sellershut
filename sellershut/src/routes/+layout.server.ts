import type { LayoutServerLoad } from './$types';

// export const load: LayoutServerLoad = async () => {
// return {
//   user: {
//     id: 'user_01',
//     username: 'cyril',
//     displayName: 'Cyril Figgis',
//     handle: '@cyril@sellers.hut',
//     avatarUrl: 'https://i.pravatar.cc/150?img=47',
//
//     instance: {
//       domain: 'sellers.hut',
//       name: 'sellershut',
//       url: 'https://sellers.hut',
//     },
//   },
// };
// };

// src/routes/+layout.server.ts
//import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
  return {
    user: locals.user,
    domain: locals.instance.domain,
  };
};
