<script>
	import { auth } from '$lib/userState.svelte';
</script>

<h1>Stražkvaldo</h1>

{#if auth.userState.signedIn}
Vitaj {auth.userState.username}!
{:else}
Prihlás sa a odomkni svoje funkcionality.
{/if}