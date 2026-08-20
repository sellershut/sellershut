import type { Component } from 'svelte';
import DiscordIcon from '$lib/components/auth/icons/DiscordIcon.svelte';
import GoogleIcon from '$lib/components/auth/icons/GoogleIcon.svelte';

export type OauthProvider = {
  id: string;
  name: string;
  icon: Component;
};

export const oauthProviders: OauthProvider[] = [
  {
    id: 'google',
    name: 'Google',
    icon: GoogleIcon,
  },
  {
    id: 'discord',
    name: 'Discord',
    icon: DiscordIcon,
  },
];
