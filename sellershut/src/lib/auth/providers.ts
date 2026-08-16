import type { Component } from 'svelte';
import DiscordIcon from '$lib/components/auth/icons/DiscordIcon.svelte';
import GoogleIcon from '$lib/components/auth/icons/GoogleIcon.svelte';

export type OAuthProvider = {
  id: string;
  name: string;
  href: string;
  icon: Component;
};

export const oauthProviders: OAuthProvider[] = [
  {
    id: 'google',
    name: 'Google',
    href: '/auth/oauth/google',
    icon: GoogleIcon,
  },
  {
    id: 'discord',
    name: 'Discord',
    href: '/auth/oauth/discord',
    icon: DiscordIcon,
  },
];
