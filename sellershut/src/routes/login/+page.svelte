<script lang="ts">
import { ShieldCheck } from '@lucide/svelte';
import { page } from '$app/state';
import { oauthProviders } from '$lib/auth/providers';
import OAuthButton from '$lib/components/auth/OAuthButton.svelte';
import SellershutIcon from '$lib/components/icons/SellershutIcon.svelte';

const error = $derived(page.url.searchParams.get('error'));
</script>

<svelte:head>
  <title>Sign in · Relay</title>

  <meta name="description" content="Sign in to your marketplace account.">
</svelte:head>

<main
  class="
		flex
		min-h-[calc(100dvh-3rem)]
		items-center
		justify-center
		px-5
		py-16
		sm:px-6
	"
>
  <section class="w-full max-w-[400px]">
    <!-- Brand -->
    <div class="mb-9 text-center">
      <div
        class="
					mx-auto
					flex size-11
					items-center
					justify-center
                    text-primary
				"
      >
        <SellershutIcon class="scale-125" />
      </div>

      <h1
        class="
					mt-5
					font-title
					text-[28px]
					font-semibold
					tracking-[-0.04em]
					text-foreground
				"
      >
        Sign in to sellershut
      </h1>

      <p
        class="
					mx-auto
					mt-2
					max-w-xs
					text-[13px]
					leading-5
					text-muted-foreground
				"
      >
        Continue to your marketplace account using one of your existing accounts.
      </p>
    </div>

    {#if error}
      <div
        role="alert"
        class="
					mb-4
					rounded-xl
					border
					border-primary/20
					bg-primary/5
					px-4
					py-3
					text-[12px]
					leading-5
					text-foreground
				"
      >
        We couldn't sign you in. Please try again.
      </div>
    {/if}

    <!-- OAuth providers -->
    <div class="flex flex-col gap-2.5">
      {#each oauthProviders as provider (provider.id)}
        <OAuthButton {provider} />
      {/each}
    </div>

    <!-- Federation context -->
    <div
      class="
				mt-7
				flex
				items-start
				gap-2.5
				rounded-xl
				bg-muted/60
				px-3.5
				py-3
			"
    >
      <ShieldCheck
        size={16}
        strokeWidth={1.8}
        class="
					mt-0.5
					shrink-0
					text-muted-foreground
				"
      />

      <p
        class="
					text-[11px]
					leading-[1.55]
					text-muted-foreground
				"
      >
        Your account belongs to this marketplace instance. You can still browse and interact with
        listings from other connected instances.
      </p>
    </div>

    <p
      class="
				mt-7
				text-center
				text-[10px]
				leading-[1.6]
				text-muted-foreground
			"
    >
      By continuing, you agree to the

      <a
        href="/terms"
        class="
					underline
					underline-offset-2
					transition-colors
					hover:text-foreground
				"
      >
        Terms
      </a>

      and

      <a
        href="/privacy"
        class="
					underline
					underline-offset-2
					transition-colors
					hover:text-foreground
				"
      >
        Privacy Policy
      </a>.
    </p>
  </section>
</main>
