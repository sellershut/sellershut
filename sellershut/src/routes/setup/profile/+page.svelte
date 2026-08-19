<script lang="ts">
import { ArrowRight, AtSign, Check } from '@lucide/svelte';
import { Button, Label } from 'bits-ui';
import { enhance } from '$app/forms';

let { form } = $props();

let username = $derived(form?.username ?? '');

let isValid = $derived(/^[a-z0-9_]{3,30}$/.test(username));

function handleInput(event: Event) {
  const input = event.currentTarget as HTMLInputElement;

  username = input.value.toLowerCase().replace(/[^a-z0-9_]/g, '');
}

let submitting = $state(false);
</script>

<svelte:head>
  <title>Choose your username</title>
  <meta name="description" content="Choose your username for your marketplace profile.">
</svelte:head>

<main class="flex min-h-[calc(100vh-4rem)] items-center justify-center px-4 py-12 sm:px-6">
  <div class="w-full max-w-md">
    <!-- Progress -->
    <div class="mb-10 flex items-center gap-3">
      <div class="h-1.5 flex-1 rounded-full bg-primary"></div>
      <div class="h-1.5 flex-1 rounded-full bg-muted"></div>
      <div class="h-1.5 flex-1 rounded-full bg-muted"></div>
    </div>

    <!-- Heading -->
    <div class="mb-8 text-center">
      <div class="mx-auto mb-6 flex size-14 items-center justify-center rounded-2xl bg-muted">
        <AtSign class="size-6 text-primary" />
      </div>

      <h1 class="text-2xl font-semibold tracking-[-0.03em] sm:text-3xl">Choose your username</h1>

      <p class="mx-auto mt-3 max-w-sm text-sm leading-6 text-muted-foreground">
        Your username identifies you across the marketplace. You can change your display name later.
      </p>
    </div>

    <!-- Form -->
    <form
      method="POST"
      use:enhance={() => {
		submitting = true;

		return async ({ update }) => {
			await update();
			submitting = false;
		};
	}}
      class="rounded-2xl border border-border bg-card p-5 shadow-sm sm:p-6"
    >
      <div class="space-y-2">
        <Label.Root for="username" class="text-sm font-medium text-card-foreground">
          Username
        </Label.Root>

        <div class="relative">
          <div
            class="pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3.5 text-muted-foreground"
          >
            <AtSign class="size-4" />
          </div>

          <input
            id="username"
            name="username"
            type="text"
            autocomplete="username"
            placeholder="yourusername"
            value={username}
            oninput={handleInput}
            aria-invalid={form?.error ? 'true' : 'false'}
            aria-describedby="username-help username-error"
            class="h-12 w-full rounded-xl border-border bg-background pl-10 pr-10 text-sm text-foreground outline-none transition-all placeholder:text-muted-foreground focus:border-ring focus:ring-2 focus:ring-ring/20"
            required
            minlength={3}
            maxlength={30}
            pattern="[a-z0-9_]+"
          >

          {#if isValid && !form?.error}
            <div class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-3.5">
              <div
                class="flex size-5 items-center justify-center rounded-full bg-primary text-primary-foreground"
              >
                <Check class="size-3" />
              </div>
            </div>
          {/if}
        </div>

        {#if form?.error}
          <p id="username-error" class="text-xs font-medium text-primary">
            {form.error}
          </p>
        {:else}
          <p id="username-help" class="text-xs text-muted-foreground">
            3–30 characters. Lowercase letters, numbers, and underscores.
          </p>
        {/if}
      </div>

      <!-- Profile URL preview -->
      <div class="mt-6 rounded-xl bg-muted p-4">
        <p class="text-xs font-medium text-muted-foreground">Your profile</p>

        <p class="mt-1 truncate text-sm font-medium text-card-foreground">
          /@{username || 'yourusername'}
        </p>
      </div>

      <Button.Root
        type="submit"
        disabled={!isValid || submitting}
        class="mt-6 flex h-12 w-full items-center justify-center gap-2 rounded-xl bg-primary px-5 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
      >
        Continue
        <ArrowRight class="size-4" />
      </Button.Root>
    </form>

    <p class="mt-6 text-center text-xs leading-5 text-muted-foreground">
      Your username is public and will be visible to other users on the marketplace.
    </p>
  </div>
</main>
