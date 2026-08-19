// See https://svelte.dev/docs/kit/types#app.d.ts

import type { User } from '$lib/types/user';

// for information about these interfaces
declare global {
  namespace App {
    // interface Error {}
    interface Locals {
      user?: User;
      instance: {
        domain: string;
        name: string;
        url: string;
      };
    }
    // interface PageData {}
    // interface PageState {}
    // interface Platform {}
  }
}
