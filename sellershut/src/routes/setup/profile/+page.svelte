<script lang="ts">
import { ArrowRight, Camera, UserRound } from '@lucide/svelte';
import { Button, Label } from 'bits-ui';
import { enhance } from '$app/forms';

let { form } = $props();

let username = $derived(form?.values?.username ?? '');
let displayName = $derived(form?.values?.displayName ?? '');
let description = $derived(form?.values?.description ?? '');

let profilePicture = $state<File | null>(null);
let avatarPreview = $state<string | null>(null);

let submitting = $state(false);

let isValid = $derived(/^[a-z0-9_]{3,15}$/.test(username));

function errors(field: 'username' | 'displayName' | 'description' | 'profilePicture'): string[] {
  return form?.errors?.[field] ?? [];
}

function handleUsernameInput(event: Event) {
  const input = event.currentTarget as HTMLInputElement;

  username = input.value.toLowerCase().replace(/[^a-z0-9_]/g, '');
}

function handleAvatarChange(event: Event) {
  const input = event.currentTarget as HTMLInputElement;
  const file = input.files?.[0];

  if (!file) {
    profilePicture = null;
    avatarPreview = null;
    return;
  }

  if (!file.type.startsWith('image/')) {
    input.value = '';
    profilePicture = null;
    avatarPreview = null;
    return;
  }

  if (file.size > 5 * 1024 * 1024) {
    input.value = '';
    profilePicture = null;
    avatarPreview = null;
    return;
  }

  profilePicture = file;

  if (avatarPreview) {
    URL.revokeObjectURL(avatarPreview);
  }

  avatarPreview = URL.createObjectURL(file);
}

function removeProfilePicture() {
  profilePicture = null;

  if (avatarPreview) {
    URL.revokeObjectURL(avatarPreview);
    avatarPreview = null;
  }

  const input = document.getElementById('profile-picture') as HTMLInputElement | null;

  if (input) {
    input.value = '';
  }
}

function handleEnhance() {
  submitting = true;

  return async ({ update }: { update: () => Promise<void> }) => {
    try {
      await update();
    } finally {
      submitting = false;
    }
  };
}
</script>

<svelte:head>
  <title>Set up your profile</title>

  <meta name="description" content="Set up your profile and choose your username.">
</svelte:head>

<div class="min-h-screen bg-background text-foreground">
  <main>
    <div
      class="mx-auto flex min-h-[calc(100vh-4rem)] max-w-3xl items-center px-4 py-12 sm:px-6 lg:px-8"
    >
      <div class="w-full">
        <!-- Heading -->

        <div class="mb-10 text-center">
          <div
            class="mx-auto mb-5 flex size-14 items-center justify-center rounded-2xl bg-primary/10 text-primary"
          >
            <UserRound class="size-7" />
          </div>

          <h1 class="text-3xl font-semibold tracking-[-0.035em] text-foreground sm:text-4xl">
            Set up your profile
          </h1>

          <p class="mx-auto mt-3 max-w-lg text-sm leading-6 text-muted-foreground sm:text-base">
            Choose your username and tell the community a little about yourself.
          </p>
        </div>

        <!-- Form -->

        <form
          method="POST"
          enctype="multipart/form-data"
          use:enhance={handleEnhance}
          class="rounded-2xl border border-border bg-card p-5 shadow-sm sm:p-8"
        >
          <div class="grid gap-8 sm:grid-cols-[180px_1fr]">
            <!-- Profile picture -->

            <div>
              <Label.Root
                for="profile-picture"
                class="mb-3 block text-sm font-medium text-card-foreground"
              >
                Profile picture
              </Label.Root>

              <div class="flex flex-col items-center sm:items-start">
                <div
                  class="relative size-32 overflow-hidden rounded-full border border-border bg-muted"
                >
                  {#if avatarPreview}
                    <img src={avatarPreview} alt="Profile preview" class="size-full object-cover">
                  {:else}
                    <div class="flex size-full items-center justify-center text-muted-foreground">
                      <UserRound class="size-12" />
                    </div>
                  {/if}

                  <label
                    for="profile-picture"
                    class="absolute inset-x-0 bottom-0 flex cursor-pointer items-center justify-center gap-1.5 bg-foreground/70 py-2 text-xs font-medium text-background backdrop-blur-sm transition-opacity hover:opacity-90"
                  >
                    <Camera class="size-3.5" />
                    Change
                  </label>
                </div>

                <input
                  id="profile-picture"
                  name="profile_picture"
                  type="file"
                  accept="image/png,image/jpeg,image/webp"
                  class="sr-only"
                  onchange={handleAvatarChange}
                >

                {#if errors('profilePicture').length > 0}
                  <div class="mt-2 space-y-1 text-xs text-primary">
                    {#each errors('profilePicture') as message}
                      <p>{message}</p>
                    {/each}
                  </div>
                {:else if profilePicture}
                  <button
                    type="button"
                    onclick={removeProfilePicture}
                    class="mt-2 text-xs font-medium text-muted-foreground hover:text-foreground"
                  >
                    Remove
                  </button>
                {:else}
                  <p class="mt-3 text-center text-xs text-muted-foreground sm:text-left">
                    Optional. JPG, PNG or WebP up to 5 MB.
                  </p>
                {/if}
              </div>
            </div>

            <!-- Fields -->

            <div class="space-y-6">
              <!-- Username -->

              <div>
                <Label.Root
                  for="username"
                  class="mb-2 block text-sm font-medium text-card-foreground"
                >
                  Username
                  <span class="text-primary">*</span>
                </Label.Root>

                <div class="relative">
                  <span
                    class="pointer-events-none absolute left-3 top-1/2 -translate-y-1/2 text-sm text-muted-foreground"
                  >
                    @
                  </span>

                  <input
                    id="username"
                    name="username"
                    value={username}
                    oninput={handleUsernameInput}
                    placeholder="username"
                    autocomplete="username"
                    required
                    minlength="3"
                    maxlength="15"
                    aria-invalid={errors('username').length > 0}
                    aria-describedby={errors('username').length > 0
												? 'username-error'
												: undefined}
                    class="h-11 w-full rounded-xl border-border bg-background pl-8 text-foreground placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
                  >
                </div>

                {#if errors('username').length > 0}
                  <div id="username-error" class="mt-2 space-y-1 text-xs text-primary">
                    {#each errors('username') as message}
                      <p>{message}</p>
                    {/each}
                  </div>
                {:else}
                  <p class="mt-2 text-xs text-muted-foreground">
                    This will be your unique identity on the marketplace.
                  </p>
                {/if}
              </div>

              <!-- Display name -->

              <div>
                <Label.Root
                  for="display-name"
                  class="mb-2 block text-sm font-medium text-card-foreground"
                >
                  Display name
                  <span class="font-normal text-muted-foreground"> (optional) </span>
                </Label.Root>

                <input
                  id="display-name"
                  name="display_name"
                  bind:value={displayName}
                  placeholder="Your name"
                  autocomplete="name"
                  aria-invalid={errors('displayName').length > 0}
                  aria-describedby={errors('displayName').length > 0
											? 'display-name-error'
											: undefined}
                  class="h-11 w-full rounded-xl border-border bg-background text-foreground placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
                >

                {#if errors('displayName').length > 0}
                  <div id="display-name-error" class="mt-2 space-y-1 text-xs text-primary">
                    {#each errors('displayName') as message}
                      <p>{message}</p>
                    {/each}
                  </div>
                {:else}
                  <p class="mt-2 text-xs text-muted-foreground">The name other people will see.</p>
                {/if}
              </div>

              <!-- Description -->

              <div>
                <Label.Root
                  for="description"
                  class="mb-2 block text-sm font-medium text-card-foreground"
                >
                  About you
                  <span class="font-normal text-muted-foreground"> (optional) </span>
                </Label.Root>

                <textarea
                  id="description"
                  name="description"
                  bind:value={description}
                  placeholder="Tell people a little about yourself..."
                  rows={4}
                  maxlength={280}
                  aria-invalid={errors('description').length > 0}
                  aria-describedby={errors('description').length > 0
											? 'description-error'
											: undefined}
                  class="w-full resize-none rounded-xl border-border bg-background text-sm text-foreground placeholder:text-muted-foreground focus-visible:ring-2 focus-visible:ring-ring"
                ></textarea>

                {#if errors('description').length > 0}
                  <div id="description-error" class="mt-2 space-y-1 text-xs text-primary">
                    {#each errors('description') as message}
                      <p>{message}</p>
                    {/each}
                  </div>
                {:else}
                  <div class="mt-2 flex justify-between">
                    <p class="text-xs text-muted-foreground">You can always change this later.</p>

                    <span class="text-xs text-muted-foreground"> {description.length}/280 </span>
                  </div>
                {/if}
              </div>
            </div>
          </div>

          <!-- General server error -->

          {#if form?.message}
            <div
              role="alert"
              class="mt-6 rounded-xl border border-primary/20 bg-primary/5 px-4 py-3 text-sm text-primary"
            >
              {form.message}
            </div>
          {/if}

          <!-- Submit -->

          <div
            class="mt-8 flex flex-col-reverse gap-3 border-t border-border pt-6 sm:flex-row sm:items-center sm:justify-between"
          >
            <p class="text-xs text-muted-foreground">You can update your profile anytime.</p>

            <Button.Root
              type="submit"
              disabled={submitting || !isValid}
              class="inline-flex h-11 items-center justify-center gap-2 rounded-xl bg-primary px-6 text-sm font-semibold text-primary-foreground transition-opacity hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
            >
              {#if submitting}
                Saving...
              {:else}
                Continue
                <ArrowRight class="size-4" />
              {/if}
            </Button.Root>
          </div>
        </form>
      </div>
    </div>
  </main>
</div>
