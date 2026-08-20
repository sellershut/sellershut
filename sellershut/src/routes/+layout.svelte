<script lang="ts">
import favicon from '$lib/assets/favicon.svg';

import SiteNav from '$lib/components/navigation/SiteNav.svelte';

let { data, children } = $props();

import '../app.css';
</script>

<svelte:head>
  <link rel="icon" href={favicon}>
</svelte:head>

<SiteNav user={data.user} domain={data.domain} brandName="sellershut" />
{@render children()}
