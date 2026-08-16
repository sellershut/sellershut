<script lang="ts">
import { ArrowRight } from '@lucide/svelte';

import type { OAuthProvider } from '$lib/auth/providers';

let {
  provider,
}: {
  provider: OAuthProvider;
} = $props();

const Icon = provider.icon;
</script>

<a
  href={provider.href}
  class="
		group
		flex h-12
		w-full
		items-center
		gap-3
		rounded-xl
		border border-border
		bg-background
		px-4

		text-[14px]
		font-medium
		text-foreground

		transition
		duration-200

		hover:border-foreground/20
		hover:bg-muted/60

		focus-visible:outline-none
		focus-visible:ring-2
		focus-visible:ring-ring
		focus-visible:ring-offset-2
		focus-visible:ring-offset-background

		active:scale-[0.99]
	"
>
  <span
    class="
			flex size-6
			shrink-0
			items-center
			justify-center
		"
  >
    <Icon class="size-5" />
  </span>

  <span class="flex-1 text-left"> Continue with {provider.name} </span>

  <ArrowRight
    size={15}
    strokeWidth={1.8}
    class="
			text-muted-foreground
			transition-transform
			duration-200
			group-hover:translate-x-0.5
		"
  />
</a>
